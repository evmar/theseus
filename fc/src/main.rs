mod memory;

use std::collections::HashMap;

use memory::*;

#[derive(Debug)]
struct State {
    pe_file: pe::File,
    mem: Memory,
    imports: HashMap<u32, (String, String)>,
}

impl State {
    fn new(buf: Vec<u8>) -> State {
        let f = pe::File::parse(&buf).unwrap();
        let mut mem = Memory::default();

        let image_base = AddrAbs(f.opt_header.ImageBase);
        mem.alloc("exe header".into(), image_base, 0x1000);
        mem.put(image_base, &buf[..0x1000.min(buf.len())]);
        for sec in &f.sections {
            let addr = AddrImage(sec.VirtualAddress).to_abs(image_base);
            let size = sec.SizeOfRawData.max(sec.VirtualSize);
            mem.alloc(sec.name().unwrap().into(), addr, size);
            let data = &buf[sec.PointerToRawData as usize
                ..(sec.PointerToRawData + sec.SizeOfRawData) as usize];
            mem.put(addr, data);
        }
        println!("{:#x?}", mem);

        State {
            pe_file: f,
            mem,
            imports: Default::default(),
        }
    }

    fn read_imports(&mut self) {
        let Some(imports) = self
            .pe_file
            .get_data_directory(pe::IMAGE_DIRECTORY_ENTRY::IMPORT)
        else {
            return;
        };
        let image_base = self.image_base();
        let image = self.mem.slice_all(image_base);
        for imp in pe::read_imports(imports.as_slice(image).unwrap()) {
            let name = std::str::from_utf8(imp.image_name(image)).unwrap();
            println!("{name:?}");
            for (addr, entry) in imp.iat_iter(image) {
                let addr = AddrImage(addr);
                self.imports.insert(
                    addr.to_abs(image_base).0,
                    (name.into(), entry.as_import_symbol(image).to_string()),
                );
            }
        }
    }

    fn image_base(&self) -> AddrAbs {
        AddrAbs(self.pe_file.opt_header.ImageBase)
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let [_, exe_path, outdir] = args.as_slice() else {
        println!("usage: {} exe outdir", args[0]);
        return;
    };

    let buf = std::fs::read(exe_path).unwrap();
    let mut state = State::new(buf);
    let image_base = state.image_base();
    state.read_imports();
    println!("{:#x?}", state.imports);

    let ip = AddrImage(state.pe_file.opt_header.AddressOfEntryPoint);
    let mut decoder = iced_x86::Decoder::with_ip(
        32,
        &state.mem.data[ip.to_abs(image_base).0 as usize..],
        ip.to_abs(image_base).0 as u64,
        iced_x86::DecoderOptions::NONE,
    );
    for instr in &mut decoder {
        println!("{:08x} {}", AddrAbs(instr.ip32()).0, instr);
        if instr.flow_control() != iced_x86::FlowControl::Next {
            if instr.mnemonic() == iced_x86::Mnemonic::Call {
            } else {
                break;
            }
        }
    }

    for map in &state.mem.mappings {
        std::fs::write(
            format!("{outdir}/data/{:08x}.raw", map.addr.0),
            state.mem.slice(map.addr, map.len),
        )
        .unwrap();
    }
}
