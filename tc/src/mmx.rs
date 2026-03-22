use crate::{
    State,
    codegen::{self, Writer},
};

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
        _ => format!("{} as u64", codegen::get_reg(reg)),
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

fn mmx_set(instr: &iced_x86::Instruction, n: u32, expr: String) -> String {
    use iced_x86::OpKind::*;
    match instr.op_kind(n) {
        Register => format!("{} = {};", mmx_reg(instr.op_register(n)), expr),
        Memory => {
            let addr = codegen::gen_addr(instr);
            let size = codegen::mem_size(instr);
            codegen::set_mem(format!("u{size}"), addr, expr)
        }
        _ => unreachable!(),
    }
}

pub fn codegen(w: &mut Writer, _state: &State, instr: &iced_x86::Instruction) -> bool {
    use iced_x86::Mnemonic::*;
    match instr.mnemonic() {
        Movd | Paddsb | Paddsw => w.todo(),
        // Movd => {
        //     w.line(mmx_set(instr, 0, mmx_get(instr, 1)));
        // }
        Pxor => {
            w.line(mmx_set(
                instr,
                0,
                format!("{} ^ {}", mmx_get(instr, 0), mmx_get(instr, 1)),
            ));
        }

        // Paddsb => {
        //     w.line(mmx_set(
        //         instr,
        //         0,
        //         format!("paddsb({}, {})", mmx_get(instr, 0), mmx_get(instr, 1)),
        //     ));
        // }
        // Paddsw => {
        //     w.line(mmx_set(
        //         instr,
        //         0,
        //         format!("paddsw({}, {})", mmx_get(instr, 0), mmx_get(instr, 1)),
        //     ));
        // }
        Punpcklbw | Pmullw | Psrlw | Packuswb | Emms | Psubusb | Paddusb | Psubw | Psraw => {
            w.line("todo!();");
        }
        _ => return false,
    }
    true
}
