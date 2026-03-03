mod memory;

use memory::*;

#[derive(Debug)]
struct State {
    pe_file: pe::File,
    mem: Memory,
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

        State { pe_file: f, mem }
    }

    fn image_base(&self) -> AddrAbs {
        AddrAbs(self.pe_file.opt_header.ImageBase)
    }
}

fn main() {
    let buf = std::fs::read("../../win/rs/exe/winapi/winapi.exe").unwrap();
    let mut state = State::new(buf);
    let image_base = state.image_base();
    println!("{:#x?}", state);

    let ip = AddrImage(state.pe_file.opt_header.AddressOfEntryPoint);
    let mapping = state.mem.find(ip.to_abs(image_base));

    let mut decoder = iced_x86::Decoder::with_ip(
        32,
        &mapping.data,
        ip.to_abs(image_base).to_mapping(&mapping).0 as u64,
        iced_x86::DecoderOptions::NONE,
    );
    for instr in &mut decoder {
        println!(
            "{:08x} {}",
            AddrMapping(instr.ip32()).to_abs(mapping),
            instr
        );
        if instr.flow_control() != iced_x86::FlowControl::Next {
            if instr.mnemonic() == iced_x86::Mnemonic::Call {
            } else {
                break;
            }
        }
    }
}
