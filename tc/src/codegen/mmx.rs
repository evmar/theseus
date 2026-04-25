use crate::codegen::{self, CodeGen};

fn is_mmx_reg(reg: iced_x86::Register) -> bool {
    use iced_x86::Register::*;
    matches!(reg, MM0 | MM1 | MM2 | MM3 | MM4 | MM5 | MM6 | MM7)
}

fn mmx_reg(reg: iced_x86::Register) -> String {
    use iced_x86::Register::*;
    match reg {
        MM0 => "ctx.cpu.mmx.mm0".into(),
        MM1 => "ctx.cpu.mmx.mm1".into(),
        MM2 => "ctx.cpu.mmx.mm2".into(),
        MM3 => "ctx.cpu.mmx.mm3".into(),
        MM4 => "ctx.cpu.mmx.mm4".into(),
        MM5 => "ctx.cpu.mmx.mm5".into(),
        MM6 => "ctx.cpu.mmx.mm6".into(),
        MM7 => "ctx.cpu.mmx.mm7".into(),
        _ => unreachable!(),
    }
}

fn mmx_get(instr: &iced_x86::Instruction, n: u32) -> String {
    use iced_x86::OpKind::*;
    match instr.op_kind(n) {
        Register => mmx_reg(instr.op_register(n)),
        Memory => {
            let addr = codegen::gen_addr(instr);
            let size = codegen::mem_size(instr);
            codegen::get_mem(format!("u{size}"), addr)
        }
        Immediate8 => format!("{:#x}u64", instr.immediate8()),
        k => todo!("{k:?}"),
    }
}

fn mmx_get_32(instr: &iced_x86::Instruction, n: u32) -> String {
    use iced_x86::OpKind::*;
    if matches!(instr.op_kind(n), Register) {
        let reg = instr.op_register(n);
        if is_mmx_reg(reg) {
            return format!("{} as u32", mmx_reg(instr.op_register(n)));
        }
    }
    codegen::get_op(instr, n)
}

fn mmx_set(instr: &iced_x86::Instruction, n: u32, expr: String) -> String {
    use iced_x86::OpKind::*;
    match instr.op_kind(n) {
        Register => format!("{} = {};", mmx_reg(instr.op_register(n)), expr),
        Memory => {
            let addr = codegen::gen_addr(instr);
            codegen::set_mem("u64".into(), addr, expr)
        }
        _ => unreachable!(),
    }
}

fn mmx_set_32(instr: &iced_x86::Instruction, n: u32, expr: String) -> String {
    use iced_x86::OpKind::*;
    if matches!(instr.op_kind(n), Register) {
        let reg = instr.op_register(n);
        if is_mmx_reg(reg) {
            return format!("{} = {} as u64;", mmx_reg(reg), expr);
        }
    }
    codegen::set_op(instr, n, expr)
}

impl<'a> CodeGen<'a> {
    pub fn codegen_mmx(&mut self, instr: &iced_x86::Instruction) -> bool {
        use iced_x86::Mnemonic::*;
        match instr.mnemonic() {
            Movd => self.line(mmx_set_32(instr, 0, mmx_get_32(instr, 1))),
            Movq => self.line(mmx_set(instr, 0, mmx_get(instr, 1))),

            Pxor => {
                self.line(mmx_set(
                    instr,
                    0,
                    format!("{} ^ {}", mmx_get(instr, 0), mmx_get(instr, 1)),
                ));
            }

            // Binary operations, all implemented with same name as mnemonic.
            Paddsb | Paddsw | Paddusb | Pmullw | Psrlw | Packuswb | Psubusb | Psubw | Psraw => {
                let func = format!("{:?}", instr.mnemonic()).to_ascii_lowercase();
                self.line(mmx_set(
                    instr,
                    0,
                    format!("{func}({}, {})", mmx_get(instr, 0), mmx_get(instr, 1)),
                ));
            }

            // Punpcklbw special because it only reads 4 bytes of memory.
            Punpcklbw => {
                let func = format!("{:?}", instr.mnemonic()).to_ascii_lowercase();
                self.line(mmx_set(
                    instr,
                    0,
                    format!("{func}({}, {})", mmx_get_32(instr, 0), mmx_get_32(instr, 1)),
                ));
            }

            Emms => {
                self.line("// no-op");
            }

            _ => return false,
        }
        true
    }
}
