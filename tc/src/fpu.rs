use crate::{
    State,
    codegen::{Writer, gen_addr, get_op, mem_size, op_size, read_mem},
};

pub fn codegen(w: &mut Writer, _state: &State, instr: &iced_x86::Instruction) -> bool {
    use iced_x86::Mnemonic::*;
    match instr.mnemonic() {
        Fild => {
            writeln!(
                w,
                "fild({} as i{size} as f64);",
                get_op(instr, 0),
                size = op_size(instr, 0)
            );
        }

        Fmul => match instr.op_count() {
            1 => {
                let op1 = read_mem(format!("f{}", mem_size(instr)), gen_addr(instr));
                writeln!(
                    w,
                    "MACHINE.fpu.set(0, fmul(MACHINE.fpu.get(0), {op1} as f64));"
                );
            }
            2 => {
                writeln!(w, "todo!();");
            }
            _ => todo!(),
        },

        Fld | Fistp | Fcomp | Fnstsw | Fsub | Fsubp | Fsubrp | Fdivp | Fadd | Fdivrp | Fmulp
        | Fsubr | Fstp | Faddp | Fsqrt | Fld1 | Fxch | Fst | Fchs | Fldz | Fpatan | Fdivr
        | Fsin | Fcos | Fdiv => {
            writeln!(w, "todo!();");
        }
        _ => return false,
    }
    true
}
