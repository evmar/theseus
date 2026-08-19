//! Filesystem access, as the emulated program sees it.
//!
//! Natively this is a thin wrapper over `std::fs`. In the browser there is no
//! filesystem at all, so the wasm build serves an in-memory tree that the page
//! fills in before starting the program (see [`mount`]).

use std::{
    io::{Read, Result, Seek, Write},
    path::{Path, PathBuf},
};

/// How to open a file, mirroring the subset of `std::fs::OpenOptions` that
/// CreateFile needs.
#[derive(Default, Clone, Copy)]
pub struct OpenOptions {
    pub read: bool,
    pub write: bool,
    pub create: bool,
    /// Fail if the file already exists.
    pub create_new: bool,
    pub truncate: bool,
}

impl OpenOptions {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn read(&mut self, read: bool) -> &mut Self {
        self.read = read;
        self
    }
    pub fn write(&mut self, write: bool) -> &mut Self {
        self.write = write;
        self
    }
    pub fn create(&mut self, create: bool) -> &mut Self {
        self.create = create;
        self
    }
    pub fn create_new(&mut self, create_new: bool) -> &mut Self {
        self.create_new = create_new;
        self
    }
    pub fn truncate(&mut self, truncate: bool) -> &mut Self {
        self.truncate = truncate;
        self
    }

    pub fn open(&self, path: impl AsRef<Path>) -> Result<File> {
        open(path.as_ref(), *self)
    }
}

/// One entry of a directory listing.
pub struct DirEntry {
    pub name: String,
    pub is_dir: bool,
    pub len: u64,
}

pub fn read(path: impl AsRef<Path>) -> Result<Vec<u8>> {
    let mut buf = Vec::new();
    OpenOptions::new()
        .read(true)
        .open(path)?
        .read_to_end(&mut buf)?;
    Ok(buf)
}

#[cfg(not(target_family = "wasm"))]
mod imp {
    use super::*;

    pub struct File(std::fs::File);

    impl File {
        pub fn set_len(&mut self, len: u64) -> Result<()> {
            self.0.set_len(len)
        }
    }

    impl Read for File {
        fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
            self.0.read(buf)
        }
    }

    impl Write for File {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.0.write(buf)
        }
        fn flush(&mut self) -> Result<()> {
            self.0.flush()
        }
    }

    impl Seek for File {
        fn seek(&mut self, pos: std::io::SeekFrom) -> Result<u64> {
            self.0.seek(pos)
        }
    }

    pub fn open(path: &Path, options: OpenOptions) -> Result<File> {
        let mut opts = std::fs::OpenOptions::new();
        opts.read(options.read)
            .write(options.write)
            .create(options.create)
            .create_new(options.create_new)
            .truncate(options.truncate);
        opts.open(path).map(File)
    }

    pub fn read_dir(path: &Path) -> Result<Vec<DirEntry>> {
        let mut entries = Vec::new();
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let meta = entry.metadata().ok();
            entries.push(DirEntry {
                name: entry.file_name().to_string_lossy().into_owned(),
                is_dir: meta.as_ref().map(|m| m.is_dir()).unwrap_or(false),
                len: meta.as_ref().map(|m| m.len()).unwrap_or(0),
            });
        }
        Ok(entries)
    }

    pub fn remove_file(path: &Path) -> Result<()> {
        std::fs::remove_file(path)
    }

    pub fn exists(path: &Path) -> bool {
        path.exists()
    }

    pub fn current_dir() -> Result<PathBuf> {
        std::env::current_dir()
    }

    pub fn set_current_dir(path: &Path) -> Result<()> {
        std::env::set_current_dir(path)
    }
}

#[cfg(target_family = "wasm")]
mod imp {
    use std::{
        collections::BTreeMap,
        sync::{Mutex, MutexGuard},
    };

    use super::*;

    /// An in-memory filesystem, keyed by absolute '/'-separated path.
    /// Directories are implied by the paths of the files inside them.
    #[derive(Default)]
    struct MemFs {
        files: BTreeMap<String, Vec<u8>>,
        cwd: String,
    }

    static FS: Mutex<Option<MemFs>> = Mutex::new(None);

    fn fs() -> MutexGuard<'static, Option<MemFs>> {
        let mut fs = FS.lock().unwrap();
        if fs.is_none() {
            *fs = Some(MemFs {
                files: Default::default(),
                cwd: "/".into(),
            });
        }
        fs
    }

    fn normalize(path: &Path) -> String {
        let path = path.to_string_lossy().replace('\\', "/");
        let mut parts: Vec<String> = Vec::new();
        if !path.starts_with('/') {
            let cwd = fs().as_ref().unwrap().cwd.clone();
            parts.extend(cwd.split('/').filter(|p| !p.is_empty()).map(String::from));
        }
        for part in path.split('/') {
            match part {
                "" | "." => {}
                ".." => {
                    parts.pop();
                }
                part => parts.push(part.to_string()),
            }
        }
        format!("/{}", parts.join("/"))
    }

    fn not_found() -> std::io::Error {
        std::io::Error::new(std::io::ErrorKind::NotFound, "no such file")
    }

    /// Add a file to the in-memory filesystem. The page calls this for each
    /// file of the program's data before starting it.
    pub fn mount(path: &str, data: Vec<u8>) {
        let path = normalize(Path::new(path));
        fs().as_mut().unwrap().files.insert(path, data);
    }

    pub struct File {
        path: String,
        data: Vec<u8>,
        pos: u64,
        writable: bool,
        dirty: bool,
    }

    impl File {
        pub fn set_len(&mut self, len: u64) -> Result<()> {
            self.data.resize(len as usize, 0);
            self.dirty = true;
            Ok(())
        }

        fn store(&mut self) {
            if !self.dirty {
                return;
            }
            self.dirty = false;
            fs().as_mut()
                .unwrap()
                .files
                .insert(self.path.clone(), self.data.clone());
            crate::host().persist_file(&self.path, &self.data);
        }
    }

    impl Drop for File {
        fn drop(&mut self) {
            self.store();
        }
    }

    impl Read for File {
        fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
            let start = (self.pos as usize).min(self.data.len());
            let n = buf.len().min(self.data.len() - start);
            buf[..n].copy_from_slice(&self.data[start..start + n]);
            self.pos += n as u64;
            Ok(n)
        }
    }

    impl Write for File {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            if !self.writable {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::PermissionDenied,
                    "file not open for writing",
                ));
            }
            let start = self.pos as usize;
            if self.data.len() < start + buf.len() {
                self.data.resize(start + buf.len(), 0);
            }
            self.data[start..start + buf.len()].copy_from_slice(buf);
            self.pos += buf.len() as u64;
            self.dirty = true;
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            self.store();
            Ok(())
        }
    }

    impl Seek for File {
        fn seek(&mut self, pos: std::io::SeekFrom) -> Result<u64> {
            use std::io::SeekFrom::*;
            let base = match pos {
                Start(n) => n as i64,
                Current(n) => self.pos as i64 + n,
                End(n) => self.data.len() as i64 + n,
            };
            if base < 0 {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "seek before start",
                ));
            }
            self.pos = base as u64;
            Ok(self.pos)
        }
    }

    pub fn open(path: &Path, options: OpenOptions) -> Result<File> {
        let path = normalize(path);
        let existing = fs().as_ref().unwrap().files.get(&path).cloned();
        if existing.is_some() && options.create_new {
            return Err(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                "file exists",
            ));
        }
        let data = match existing {
            Some(data) if !options.truncate => data,
            Some(_) => Vec::new(),
            None if options.create || options.create_new => Vec::new(),
            None => return Err(not_found()),
        };
        Ok(File {
            path,
            data,
            pos: 0,
            writable: options.write,
            // A newly created file exists even if nothing is written to it.
            dirty: options.create || options.create_new || options.truncate,
        })
    }

    pub fn read_dir(path: &Path) -> Result<Vec<DirEntry>> {
        let dir = normalize(path);
        let prefix = dir_prefix(&dir);
        let mut entries: BTreeMap<String, DirEntry> = BTreeMap::new();
        for (path, data) in &fs().as_ref().unwrap().files {
            let Some(rest) = path.strip_prefix(&prefix) else {
                continue;
            };
            match rest.split_once('/') {
                // A file directly in this directory.
                None => {
                    entries.insert(
                        rest.to_string(),
                        DirEntry {
                            name: rest.to_string(),
                            is_dir: false,
                            len: data.len() as u64,
                        },
                    );
                }
                // Something deeper: the first component is a subdirectory.
                Some((name, _)) => {
                    entries.entry(name.to_string()).or_insert_with(|| DirEntry {
                        name: name.to_string(),
                        is_dir: true,
                        len: 0,
                    });
                }
            }
        }
        if entries.is_empty() {
            return Err(not_found());
        }
        Ok(entries.into_values().collect())
    }

    pub fn remove_file(path: &Path) -> Result<()> {
        let path = normalize(path);
        match fs().as_mut().unwrap().files.remove(&path) {
            Some(_) => Ok(()),
            None => Err(not_found()),
        }
    }

    /// The prefix every path inside `dir` starts with, without doubling the
    /// slash at the root.
    fn dir_prefix(dir: &str) -> String {
        if dir.ends_with('/') {
            dir.to_string()
        } else {
            format!("{dir}/")
        }
    }

    pub fn exists(path: &Path) -> bool {
        let path = normalize(path);
        // The root is always there, even before anything is mounted.
        if path == "/" {
            return true;
        }
        let fs = fs();
        let files = &fs.as_ref().unwrap().files;
        // Either a file, or a directory that has something in it.
        let prefix = dir_prefix(&path);
        files.contains_key(&path) || files.keys().any(|key| key.starts_with(&prefix))
    }

    pub fn current_dir() -> Result<PathBuf> {
        Ok(PathBuf::from(&fs().as_ref().unwrap().cwd))
    }

    pub fn set_current_dir(path: &Path) -> Result<()> {
        let path = normalize(path);
        if !exists(Path::new(&path)) {
            return Err(not_found());
        }
        fs().as_mut().unwrap().cwd = path;
        Ok(())
    }
}

#[cfg(target_family = "wasm")]
pub use imp::mount;
pub use imp::{File, current_dir, exists, open, read_dir, remove_file, set_current_dir};
