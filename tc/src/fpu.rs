use crate::{
    State,
    codegen::{Writer, gen_addr, get_op, mem_size, op_size, read_mem},
};

fn fpu_get_mem(instr: &iced_x86::Instruction) -> String {
    read_mem(format!("f{}", mem_size(instr)), gen_addr(instr))
}

fn fpu_set_mem(instr: &iced_x86::Instruction, expr: String) -> String {
    // TODO: is this only needed by fst?
    let addr = gen_addr(instr);
    let size = mem_size(instr);
    format!("MACHINE.memory.write::<f{size}>({addr}, {expr});")
}

fn reg_to_index(register: iced_x86::Register) -> usize {
    use iced_x86::Register::*;
    match register {
        ST0 => 0,
        ST1 => 1,
        ST2 => 2,
        ST3 => 3,
        ST4 => 4,
        ST5 => 5,
        ST6 => 6,
        ST7 => 7,
        r => todo!("{r:?}"),
    }
}

fn fpu_get_reg(index: usize) -> String {
    format!("MACHINE.fpu.get({index})")
}

fn fpu_set_reg(index: usize, expr: String) -> String {
    format!("MACHINE.fpu.set({index}, {expr});")
}

fn fpu_get_op(instr: &iced_x86::Instruction, n: u32) -> String {
    use iced_x86::OpKind::*;
    match instr.op_kind(n) {
        Memory => fpu_get_mem(instr),
        Register => fpu_get_reg(reg_to_index(instr.op_register(n))),
        k => todo!("{k:?}"),
    }
}

fn fpu_set_op(instr: &iced_x86::Instruction, n: u32, expr: String) -> String {
    use iced_x86::OpKind::*;
    match instr.op_kind(n) {
        Memory => {
            let size = mem_size(instr);
            let expr = if size != 64 {
                format!("{expr} as f{size}")
            } else {
                expr
            };
            fpu_set_mem(instr, expr)
        }
        k => todo!("{k:?}"),
    }
}

pub fn codegen(w: &mut Writer, _state: &State, instr: &iced_x86::Instruction) -> bool {
    use iced_x86::Mnemonic::*;
    match instr.mnemonic() {
        Fld => {
            let expr = fpu_get_op(instr, 0);
            w.line(format!("MACHINE.fpu.push({expr} as f64);"));
        }
        Fild => {
            w.line(format!(
                "fild({} as i{size} as f64);",
                get_op(instr, 0),
                size = op_size(instr, 0)
            ));
        }

        Fst => {
            w.line(fpu_set_op(instr, 0, fpu_get_reg(0)));
        }
        Fstp => {
            w.line(fpu_set_op(instr, 0, fpu_get_reg(0)));
            w.line("MACHINE.fpu.pop();");
        }

        Fmul => match instr.op_count() {
            1 => {
                w.line(fpu_set_reg(
                    0,
                    format!("fmul({}, {} as f64)", fpu_get_reg(0), fpu_get_mem(instr)),
                ));
            }
            2 => {
                w.line("todo!();");
            }
            _ => todo!(),
        },

        Fistp | Fcomp | Fnstsw | Fsub | Fsubp | Fsubrp | Fdivp | Fadd | Fdivrp | Fmulp | Fsubr
        | Faddp | Fsqrt | Fld1 | Fxch | Fchs | Fldz | Fpatan | Fdivr | Fsin | Fcos | Fdiv => {
            w.line("todo!();");
        }
        _ => return false,
    }
    true
}
