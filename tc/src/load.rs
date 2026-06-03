//! PE loading.

use crate::{Import, Module, memory::Memory};

pub fn load_pe(mem: &mut Memory, buf: Vec<u8>) -> Module {
    let f = pe::File::parse(&buf).unwrap();

    let image_base = f.opt_header.ImageBase;
    mem.reserve("exe header".into(), image_base, 0x1000);
    mem.put(image_base, &buf[..0x1000.min(buf.len())]);
    let mut code_range = None;
    for sec in &f.sections {
        let addr = image_base + sec.VirtualAddress;
        let size = runtime::round_to_page(sec.SizeOfRawData.max(sec.VirtualSize));
        mem.reserve(sec.name().unwrap().into(), addr, size);

        let flags = sec.characteristics().unwrap();
        let load_data =
            flags.contains(pe::IMAGE_SCN::CODE) || flags.contains(pe::IMAGE_SCN::INITIALIZED_DATA);
        if load_data {
            let data = &buf[sec.PointerToRawData as usize..][..sec.SizeOfRawData as usize];
            mem.put(addr, data);
        }
        if flags.contains(pe::IMAGE_SCN::CODE) || flags.contains(pe::IMAGE_SCN::MEM_EXECUTE) {
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
        .get_data_directory(pe::IMAGE_DIRECTORY_ENTRY::RESOURCE)
        .map(|dir| {
            let addr = image_base + dir.VirtualAddress;
            addr..(addr + dir.Size)
        });

    let imports = read_imports(&f, mem);
    log::info!("imp {:#x?}", imports);

    Module {
        bitness: 32,
        code_segment: None,
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
fn read_imports(pe_file: &pe::File, mem: &Memory) -> Vec<Import> {
    let mut imports = vec![];
    let Some(dir) = pe_file.get_data_directory(pe::IMAGE_DIRECTORY_ENTRY::IMPORT) else {
        return imports;
    };
    let image_base = pe_file.opt_header.ImageBase;
    let image = mem.slice_all(image_base);
    for imp in pe::read_imports(dir.as_slice(image).unwrap()) {
        let name = std::str::from_utf8(imp.image_name(image))
            .unwrap()
            .to_lowercase();
        let name = name.trim_end_matches(".dll");
        for (addr, entry) in imp.iat_iter(image) {
            let func = match entry.as_import_symbol(image) {
                pe::ImportSymbol::Name(name) => std::str::from_utf8(name).unwrap().to_string(),
                pe::ImportSymbol::Ordinal(n) => format!("ordinal{n}"),
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
