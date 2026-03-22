use crate::{
    State,
    codegen::{self, Writer},
};

fn is_mmx_reg(reg: iced_x86::Register) -> bool {
    use iced_x86::Register::*;
    matches!(reg, MM0 | MM1 | MM2 | MM3 | MM4 | MM5 | MM6 | MM7)
}

fn mmx_reg(reg: iced_x86::Register) -> String {
    use iced_x86::Register::*;
    match reg {
        MM0 => "m.mmx.mm0".into(),
        MM1 => "m.mmx.mm1".into(),
        MM2 => "m.mmx.mm2".into(),
        MM3 => "m.mmx.mm3".into(),
        MM4 => "m.mmx.mm4".into(),
        MM5 => "m.mmx.mm5".into(),
        MM6 => "m.mmx.mm6".into(),
        MM7 => "m.mmx.mm7".into(),
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
        _ => unreachable!(),
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

pub fn codegen(w: &mut Writer, _state: &State, instr: &iced_x86::Instruction) -> bool {
    use iced_x86::Mnemonic::*;
    match instr.mnemonic() {
        Movd => w.line(mmx_set_32(instr, 0, mmx_get_32(instr, 1))),
        Movq => {
            w.line(mmx_set(instr, 0, mmx_get(instr, 1)));
        }
        Pxor => {
            w.line(mmx_set(
                instr,
                0,
                format!("{} ^ {}", mmx_get(instr, 0), mmx_get(instr, 1)),
            ));
        }

        Paddsb => {
            w.line(mmx_set(
                instr,
                0,
                format!("paddsb({}, {})", mmx_get(instr, 0), mmx_get(instr, 1)),
            ));
        }
        Paddsw => {
            w.line(mmx_set(
                instr,
                0,
                format!("paddsw({}, {})", mmx_get(instr, 0), mmx_get(instr, 1)),
            ));
        }

        Punpcklbw => {
            w.line(mmx_set(
                instr,
                0,
                format!(
                    "punpcklbw({}, {})",
                    mmx_get_32(instr, 0),
                    mmx_get_32(instr, 1)
                ),
            ));
        }

        Pmullw => {
            w.line(mmx_set(
                instr,
                0,
                format!("pmullw({}, {})", mmx_get(instr, 0), mmx_get(instr, 1)),
            ));
        }

        Psrlw | Packuswb | Emms | Psubusb | Paddusb | Psubw | Psraw | Movdqa => {
            w.todo();
        }
        _ => return false,
    }
    true
}
