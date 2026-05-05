use std::collections::HashMap;

use crate::memory::Memory;

mod codegen;
mod load;
pub use load::load_pe;
mod gather;
mod memory;
pub use gather::{EntryPoint, Gather};

#[derive(Debug, Clone)]
pub struct Import {
    pub dll: String,
    pub func: String,
    /// address to write func_addr to
    pub iat_addr: u32,
    /// address of code
    pub func_addr: u32,
}

#[derive(Default)]
pub struct Module {
    pub image_base: u32,
    pub entry_point: u32,
    pub code_memory: std::ops::Range<u32>,
    pub resources: Option<std::ops::Range<u32>>,
    pub imports: Vec<Import>,
    pub vtables: Vec<(String, u32)>,
}

#[derive(Default)]
pub struct State {
    pub module: Module,
    pub mem: Memory,
    symbol_names: HashMap<u32, String>,
    pub blocks: HashMap<u32, Block>,
}

pub struct Instr {
    pub iced: iced_x86::Instruction,
    pub hint: Option<String>,
}

impl Instr {
    pub fn next_ip(&self) -> u32 {
        self.iced.next_ip32()
    }
}

pub enum Block {
    Instrs(Vec<Instr>),
    Stdcall(String),
    Extern(u32),
}

impl Block {
    pub fn name(&self) -> String {
        match self {
            Block::Instrs(instrs) => format!("x{:x}", instrs[0].iced.ip32()),
            Block::Stdcall(func) => format!("{}_stdcall", func),
            Block::Extern(ip) => format!("x{:x}", ip),
        }
    }
}

pub fn write_if_changed(path: &str, contents: &[u8]) -> anyhow::Result<()> {
    let existing = std::fs::read(&path).unwrap_or_default();
    if existing != contents {
        std::fs::write(path, contents)?;
    }
    Ok(())
}

impl State {
    pub fn load_symbols(&mut self, csv: impl std::io::Read) -> anyhow::Result<()> {
        let mut rdr = csv::Reader::from_reader(csv);
        for result in rdr.records() {
            let record = result?;
            let name = &record[0];
            if name.starts_with("FUN_") {
                continue;
            }
            let addr = &record[1];
            if !addr.starts_with('0') {
                continue;
            }
            let addr = u32::from_str_radix(&addr, 16)
                .map_err(|err| anyhow::anyhow!(format!("{addr:?}: {err}")))?;
            self.symbol_names.insert(addr, name.to_string());
        }
        Ok(())
    }

    /// For any dll used by the module, write its vtables to the executable memory.
    fn add_vtables(&mut self) {
        let vtables_addr = self.mem.mappings.alloc("vtables".into(), 0x1000);
        let mut iat_addr = vtables_addr;
        assert!(iat_addr != 0);
        for (dll, vtables) in [
            ("ddraw", winapi::ddraw::VTABLES.as_slice()),
            ("dsound", winapi::dsound::VTABLES.as_slice()),
        ] {
            if !self.module.imports.iter().any(|imp| imp.dll == dll) {
                continue;
            }
            for (interface, entries) in vtables {
                self.module
                    .vtables
                    .push((format!("{dll}::{interface}"), iat_addr));
                for func in entries.iter() {
                    self.module.imports.push(Import {
                        dll: dll.to_string(),
                        func: format!("{interface}::{func}"),
                        iat_addr,
                        func_addr: 0,
                    });
                    iat_addr += 4;
                }
            }
        }
    }

    fn write_iat(&mut self) {
        let mut func_addr = 0xfafbfc00;
        for import in self.module.imports.iter_mut() {
            if import.iat_addr == 0 {
                panic!("{import:#x?}");
            }
            import.func_addr = func_addr;
            func_addr += 1;
            self.mem.write::<u32>(import.iat_addr, import.func_addr);
        }
    }

    pub fn init_imports(&mut self) {
        self.add_vtables();
        self.write_iat();
    }

    pub fn gather(&mut self, gather: Gather) {
        self.blocks = gather.run(self);
    }

    pub fn generate(&mut self, outdir: &str) -> anyhow::Result<()> {
        let mut codegen = codegen::CodeGen::new(self);
        codegen.gen_file(outdir)?;

        let data_dir = format!("{outdir}/data");
        std::fs::create_dir_all(&data_dir)?;
        for map in self.mem.mappings.vec().iter() {
            let buf = self.mem.slice(map.addr, map.size);
            if buf.iter().all(|&b| b == 0) {
                continue;
            }
            log::info!(
                "section {:?} @{:x} ({:x} bytes)",
                map.desc,
                map.addr,
                map.size
            );
            write_if_changed(&format!("{outdir}/data/{:08x}.raw", map.addr), buf)?;
        }
        Ok(())
    }
}
