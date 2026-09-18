//! EXE loading.

use runtime::{SegOfs, segofs};

use crate::{DOSModule, Import, Module, WindowsModule, memory::Memory};

// TODO: change load api to first load a dos exe then a windows one
// so we don't have this weird load param
pub fn load_exe(mem: &mut Memory, buf: Vec<u8>, load_segment: Option<u16>) -> Module {
    match exe::parse(&buf).unwrap() {
        exe::Parse::PE(pe) => Module::Windows(load_pe(mem, &buf, pe)),
        exe::Parse::DOS(dos) => Module::DOS(load_dos(mem, &buf, dos, load_segment)),
    }
}

fn load_dos(
    mem: &mut Memory,
    buf: &[u8],
    dos_header: exe::DOS,
    load_segment: Option<u16>,
) -> DOSModule {
    let (psp_segment, image_segment) = match load_segment {
        Some(seg) => (0, seg),
        None => {
            let psp_segment = dos::DOSBOX_SEG;
            mem.reserve("psp".into(), segofs(psp_segment, 0), 0x100);
            (psp_segment, psp_segment + 0x10)
        }
    };

    let dos_header: &exe::DOS = &dos_header;
    let load_addr = SegOfs::new(image_segment, 0).abs();
    let data = dos_header.image(buf);

    mem.reserve("dos image".into(), load_addr, data.len() as u32);

    mem.slice_mut(load_addr, data.len() as u32)
        .copy_from_slice(data);
    dos_header.apply_relocations(image_segment, &mut mem.bytes[load_addr as usize..]);
    let code_memory = load_addr..load_addr + data.len() as u32;

    DOSModule {
        is_com: false,
        psp_segment,
        load_segment: psp_segment + 0x10 + dos_header.header.initial_cs,
        stack_segment: image_segment + dos_header.header.initial_ss,
        stack_pointer: dos_header.header.initial_sp,
        entry_point: dos_header.header.entry_point,
        code_memory,
    }
}

fn load_pe(mem: &mut Memory, buf: &[u8], f: exe::PE) -> WindowsModule {
    mem.mappings.alloc("null page".into(), 0x1000);

    let image_base = f.opt_header.ImageBase;
    mem.reserve("exe header".into(), image_base, 0x1000);
    mem.write_bytes(image_base, &buf[..0x1000.min(buf.len())]);
    let mut code_range = None;
    for sec in &f.sections {
        let addr = image_base + sec.VirtualAddress;
        let size = runtime::round_to_page(sec.SizeOfRawData.max(sec.VirtualSize));
        mem.reserve(sec.name().unwrap().into(), addr, size);

        use exe::pe::IMAGE_SCN;
        let flags = sec.characteristics().unwrap();
        let load_data =
            flags.contains(IMAGE_SCN::CODE) || flags.contains(IMAGE_SCN::INITIALIZED_DATA);
        if load_data {
            let data = &buf[sec.PointerToRawData as usize..][..sec.SizeOfRawData as usize];
            mem.write_bytes(addr, data);
        }
        if flags.contains(IMAGE_SCN::CODE) || flags.contains(IMAGE_SCN::MEM_EXECUTE) {
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
        .get_data_directory(exe::pe::IMAGE_DIRECTORY_ENTRY::RESOURCE)
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
        dynamic_exports: Default::default(),
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
    let Some(dir) = pe_file.get_data_directory(exe::pe::IMAGE_DIRECTORY_ENTRY::IMPORT) else {
        return imports;
    };
    let image_base = pe_file.opt_header.ImageBase;
    let image = mem.slice_all(image_base);
    for imp in exe::read_imports(dir.as_slice(image).unwrap()) {
        let name = std::str::from_utf8(imp.image_name(image))
            .unwrap()
            .to_lowercase();
        // The module name doubles as a Rust module path, so drop the extension
        // (.dll, or .drv for winspool).
        let name = name
            .rsplit_once('.')
            .map_or(name.as_str(), |(stem, _)| stem);
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
