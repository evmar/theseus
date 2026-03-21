use crate::{
    State,
    codegen::{Writer, gen_addr, get_mem, get_op, mem_size, op_size, set_op},
};

fn fpu_get_mem(instr: &iced_x86::Instruction) -> String {
    let size = mem_size(instr);
    if size != 64 {
        format!("{} as f64", get_mem(format!("f{size}"), gen_addr(instr)))
    } else {
        get_mem(format!("f{}", mem_size(instr)), gen_addr(instr))
    }
}

fn fpu_set_mem(instr: &iced_x86::Instruction, expr: String) -> String {
    // TODO: is this only needed by fst?
    let addr = gen_addr(instr);
    let size = mem_size(instr);
    format!("m.memory.write::<f{size}>({addr}, {expr});")
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
    format!("m.fpu.get({index})")
}

fn fpu_set_reg(index: usize, expr: String) -> String {
    format!("m.fpu.set({index}, {expr});")
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
        Register => fpu_set_reg(reg_to_index(instr.op_register(n)), expr),
        k => todo!("{k:?}"),
    }
}

pub fn codegen(w: &mut Writer, _state: &State, instr: &iced_x86::Instruction) -> bool {
    use iced_x86::Mnemonic::*;
    match instr.mnemonic() {
        Fld => {
            let expr = fpu_get_op(instr, 0);
            w.line(format!("m.fpu.push({expr});"));
        }
        Fild => {
            w.line(format!(
                "m.fpu.push({} as i{size} as f64);",
                get_op(instr, 0),
                size = op_size(instr, 0)
            ));
        }
        Fldz => w.line("m.fpu.push(0.0);"),
        Fld1 => w.line("m.fpu.push(1.0);"),

        Fst | Fstp => {
            w.line(fpu_set_op(instr, 0, fpu_get_reg(0)));
            if instr.mnemonic() == Fstp {
                w.line("m.fpu.pop();");
            }
        }

        Fist | Fistp => {
            let size = op_size(instr, 0);
            w.line(set_op(
                instr,
                0,
                format!("{}.round() as i{size} as u{size}", fpu_get_reg(0)),
            ));
            if instr.mnemonic() == Fistp {
                w.line("m.fpu.pop();");
            }
        }

        // Binary ops
        Fadd | Faddp | Fsub | Fsubp | Fsubr | Fsubrp | Fmul | Fmulp | Fdivp | Fdivrp | Fdivr
        | Fdiv => {
            assert!(matches!(instr.op_count(), 1 | 2));

            let (arg0, arg1) = if instr.op_count() == 1 {
                (fpu_get_reg(0), fpu_get_op(instr, 0))
            } else {
                (fpu_get_op(instr, 0), fpu_get_op(instr, 1))
            };

            let (arg0, arg1) = if matches!(instr.mnemonic(), Fsubr | Fsubrp | Fdivr | Fdivrp) {
                (arg1, arg0)
            } else {
                (arg0, arg1)
            };

            let binop = match instr.mnemonic() {
                Fadd | Faddp => "+",
                Fsub | Fsubp | Fsubr | Fsubrp => "-",
                Fmul | Fmulp => "*",
                Fdiv | Fdivp | Fdivr | Fdivrp => "/",
                _ => unreachable!(),
            };

            let expr = format!("{arg0} {binop} {arg1}");

            if instr.op_count() == 1 {
                w.line(fpu_set_reg(0, expr));
            } else {
                w.line(fpu_set_op(instr, 0, expr));
            }

            if matches!(
                instr.mnemonic(),
                Faddp | Fsubp | Fsubrp | Fmulp | Fdivp | Fdivrp
            ) {
                w.line("m.fpu.pop();");
            }
        }

        Fchs => {
            w.line(fpu_set_reg(0, format!("-{}", fpu_get_reg(0))));
        }

        Fsin => {
            w.line(fpu_set_reg(0, format!("{}.sin()", fpu_get_reg(0))));
        }
        Fcos => {
            w.line(fpu_set_reg(0, format!("{}.cos()", fpu_get_reg(0))));
        }
        Fsqrt => {
            w.line(fpu_set_reg(0, format!("{}.sqrt()", fpu_get_reg(0))));
        }

        Fxch => {
            assert_eq!(instr.op_count(), 2);
            w.line(format!("let t = {};", fpu_get_op(instr, 0)));
            w.line(fpu_set_op(instr, 0, fpu_get_op(instr, 1)));
            w.line(fpu_set_op(instr, 1, "t".into()));
        }

        Fcomp => {
            assert_eq!(instr.op_count(), 1);
            w.line(format!(
                "m.fpu.cmp = {}.total_cmp(&({}));",
                fpu_get_reg(0),
                fpu_get_op(instr, 0)
            ));
        }

        Fnstsw => {
            assert_eq!(instr.op_count(), 1);
            w.line(set_op(instr, 0, "m.fpu.status()".into()));
        }

        Fpatan => {
            w.line("let t = m.fpu.get(0);");
            w.line("m.fpu.pop();");
            w.line("m.fpu.set(0, m.fpu.get(0).atan2(t));");
        }
        _ => return false,
    }
    true
}
