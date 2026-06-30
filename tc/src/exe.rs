//! EXE loading.

use runtime::segofs;

use crate::{DOSModule, Import, Module, WindowsModule, memory::Memory};

pub fn load_exe(mem: &mut Memory, buf: Vec<u8>) -> Module {
    match exe::parse(&buf).unwrap() {
        exe::Parse::PE(pe) => Module::Windows(load_pe(mem, &buf, pe)),
        exe::Parse::DOS(dos) => Module::DOS(load_dos(mem, &buf, dos)),
    }
}

fn load_dos(mem: &mut Memory, buf: &[u8], dos: exe::DOS) -> DOSModule {
    let load_segment = dos::DOSBOX_SEG;
    assert_eq!(dos.header.e_cs, 0);

    mem.reserve("psp".into(), segofs(load_segment, 0), 0x100);

    let code_segment = load_segment + 0x10;
    let code_addr = segofs(code_segment, 0);
    let data = &buf[dos.header_size()..];
    mem.reserve("dos data".into(), code_addr, data.len() as u32);
    mem.slice_mut(code_addr, data.len() as u32)
        .copy_from_slice(data);
    mem.mappings.dump();

    DOSModule {
        load_segment,
        code_segment: load_segment + 0x10,
        entry_point: dos.header.e_ip,
        code_memory: (code_addr..data.len() as u32),
    }
}

fn load_pe(mem: &mut Memory, buf: &[u8], f: exe::PE) -> WindowsModule {
    mem.mappings.alloc("null page".into(), 0x1000);

    let image_base = f.opt_header.ImageBase;
    mem.reserve("exe header".into(), image_base, 0x1000);
    mem.put(image_base, &buf[..0x1000.min(buf.len())]);
    let mut code_range = None;
    for sec in &f.sections {
        let addr = image_base + sec.VirtualAddress;
        let size = runtime::round_to_page(sec.SizeOfRawData.max(sec.VirtualSize));
        mem.reserve(sec.name().unwrap().into(), addr, size);

        let flags = sec.characteristics().unwrap();
        let load_data = flags.contains(exe::IMAGE_SCN::CODE)
            || flags.contains(exe::IMAGE_SCN::INITIALIZED_DATA);
        if load_data {
            let data = &buf[sec.PointerToRawData as usize..][..sec.SizeOfRawData as usize];
            mem.put(addr, data);
        }
        if flags.contains(exe::IMAGE_SCN::CODE) || flags.contains(exe::IMAGE_SCN::MEM_EXECUTE) {
            match &mut code_range {
                None => code_range = Some(addr..addr + sec.SizeOfRawData),
                Some(range) => {
                    range.start = range.start.min(addr);
                    range.end = range.end.max(addr + sec.SizeOfRawData);
                }
            }
        }
    }

    let resources = f
        .get_data_directory(exe::IMAGE_DIRECTORY_ENTRY::RESOURCE)
        .map(|dir| {
            let addr = image_base + dir.VirtualAddress;
            addr..(addr + dir.Size)
        });

    let imports = read_imports(&f, mem);

    WindowsModule {
        imports,
        image_base,
        entry_point: image_base + f.opt_header.AddressOfEntryPoint,
        code_memory: code_range.unwrap(),
        resources,
        vtables: Default::default(),
    }
}

fn is_data(dll: &str, func: &str) -> bool {
    if dll == "msvcrt" {
        return matches!(func, "_adjust_fdiv" | "_acmdln");
    }
    false
}

/// Read the file's imported symbols.
fn read_imports(pe_file: &exe::PE, mem: &Memory) -> Vec<Import> {
    let mut imports = vec![];
    let Some(dir) = pe_file.get_data_directory(exe::IMAGE_DIRECTORY_ENTRY::IMPORT) else {
        return imports;
    };
    let image_base = pe_file.opt_header.ImageBase;
    let image = mem.slice_all(image_base);
    for imp in exe::read_imports(dir.as_slice(image).unwrap()) {
        let name = std::str::from_utf8(imp.image_name(image))
            .unwrap()
            .to_lowercase();
        let name = name.trim_end_matches(".dll");
        for (addr, entry) in imp.iat_iter(image) {
            let func = match entry.as_import_symbol(image) {
                exe::ImportSymbol::Name(name) => std::str::from_utf8(name).unwrap().to_string(),
                exe::ImportSymbol::Ordinal(n) => format!("ordinal{n}"),
            };
            let data = is_data(name, &func);
            imports.push(Import {
                dll: name.to_string(),
                func,
                iat_addr: image_base + addr,
                addr: 0,
                data,
            });
        }
    }
    imports
}
