//! PE loading.

use crate::{Import, Module, add_dll_imports, memory::Memory};

pub fn load_pe(mem: &mut Memory, buf: Vec<u8>) -> Module {
    let f = pe::File::parse(&buf).unwrap();

    let image_base = f.opt_header.ImageBase;
    mem.alloc("exe header".into(), image_base, 0x1000);
    mem.put(image_base, &buf[..0x1000.min(buf.len())]);
    let mut code_range = None;
    for sec in &f.sections {
        let addr = image_base + sec.VirtualAddress;
        let size = winapi::kernel32::round_to_page(sec.SizeOfRawData.max(sec.VirtualSize));
        mem.alloc(sec.name().unwrap().into(), addr, size);

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
        .map(|dir| (image_base + dir.VirtualAddress, dir.Size));

    let mut imports = read_imports(&f, mem);
    resolve_iat(&mut imports, mem);
    add_dll_imports(&mut imports);

    Module {
        imports,
        image_base,
        entry_point: image_base + f.opt_header.AddressOfEntryPoint,
        code_memory: code_range.unwrap(),
        resources,
    }
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
            imports.push(Import {
                dll: name.to_string(),
                func: match entry.as_import_symbol(image) {
                    pe::ImportSymbol::Name(name) => std::str::from_utf8(name).unwrap().to_string(),
                    pe::ImportSymbol::Ordinal(n) => format!("ordinal{n}"),
                },
                iat_addr: image_base + addr,
                func_addr: 0,
            });
        }
    }
    imports
}

/// Assign addresses to the imported functions, and write those addresses to the IAT.
fn resolve_iat(imports: &mut [Import], mem: &mut Memory) {
    // Reserve some fake addresses for imported functions so they can be assigned addresses.
    // If we never write to the memory it stays zero and doesn't end up in the output.
    let mut import_func_addr =
        mem.mappings
            .alloc("imported functions".into(), None, imports.len() as u32);

    for import in imports.iter_mut() {
        import.func_addr = import_func_addr;
        import_func_addr += 1;
        if import.iat_addr != 0 {
            mem.write::<u32>(import.iat_addr, import.func_addr);
        } else {
            // hack: assign an unused iat addr just to ensure it has a unique key in state.imports.
            import.iat_addr = import_func_addr;
        }
    }
}
