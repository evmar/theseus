//! PE loading.

use crate::{Block, Import, State};

pub fn load_pe(state: &mut State, buf: Vec<u8>) {
    let f = pe::File::parse(&buf).unwrap();

    let image_base = f.opt_header.ImageBase;
    state.entry_point = image_base + f.opt_header.AddressOfEntryPoint;
    state.image_base = image_base;
    state.mem.alloc("exe header".into(), image_base, 0x1000);
    state.mem.put(image_base, &buf[..0x1000.min(buf.len())]);
    let mut code_range = None;
    for sec in &f.sections {
        let addr = image_base + sec.VirtualAddress;
        let size = winapi::kernel32::round_to_page(sec.SizeOfRawData.max(sec.VirtualSize));
        state.mem.alloc(sec.name().unwrap().into(), addr, size);

        let flags = sec.characteristics().unwrap();
        let load_data =
            flags.contains(pe::IMAGE_SCN::CODE) || flags.contains(pe::IMAGE_SCN::INITIALIZED_DATA);
        if load_data {
            let data = &buf[sec.PointerToRawData as usize..][..sec.SizeOfRawData as usize];
            state.mem.put(addr, data);
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
    state.code_memory = code_range.unwrap();

    state.resources = f
        .get_data_directory(pe::IMAGE_DIRECTORY_ENTRY::RESOURCE)
        .map(|dir| (image_base + dir.VirtualAddress, dir.Size));

    read_imports(&f, state);
}

fn read_imports(pe_file: &pe::File, state: &mut State) {
    let Some(imports) = pe_file.get_data_directory(pe::IMAGE_DIRECTORY_ENTRY::IMPORT) else {
        return;
    };
    let image_base = pe_file.opt_header.ImageBase;
    let image = state.mem.slice_all(image_base);
    for imp in pe::read_imports(imports.as_slice(image).unwrap()) {
        let name = std::str::from_utf8(imp.image_name(image))
            .unwrap()
            .to_lowercase();
        let name = name.trim_end_matches(".dll");
        for (addr, entry) in imp.iat_iter(image) {
            let addr = image_base + addr;
            state.imports.insert(
                addr,
                Import {
                    dll: name.to_string(),
                    func: match entry.as_import_symbol(image) {
                        pe::ImportSymbol::Name(name) => {
                            std::str::from_utf8(name).unwrap().to_string()
                        }
                        pe::ImportSymbol::Ordinal(n) => format!("ordinal{n}"),
                    },
                    iat_addr: addr,
                    func_addr: 0,
                },
            );
        }
    }

    // Reserve some fake addresses for imported functions so they can be assigned addresses.
    // If we never write to the memory it stays zero and doesn't end up in the output.
    let mut import_func_addr = state.mem.mappings.alloc(
        "imported functions".into(),
        None,
        state.imports.len() as u32,
    );

    let mut imports = state.imports.values_mut().collect::<Vec<_>>();
    imports.sort_by_key(|i| i.iat_addr);
    for import in imports.iter_mut() {
        import.func_addr = import_func_addr;
        import_func_addr += 1;
        state.mem.write::<u32>(import.iat_addr, import.func_addr);
        state.blocks.insert(
            import.func_addr,
            Block::Stdcall(format!("{}::{}", import.dll, import.func)),
        );
    }

    for (lib, exports) in [
        ("ddraw", winapi::ddraw::EXPORTS.as_slice()),
        ("dsound", winapi::dsound::EXPORTS.as_slice()),
    ] {
        if imports.iter().find(|i| i.dll == lib).is_some() {
            for func in exports {
                state.blocks.insert(
                    import_func_addr,
                    Block::Stdcall(format!("{lib}::{}", func.to_string())),
                );
                import_func_addr += 1;
            }
        }
    }
}
