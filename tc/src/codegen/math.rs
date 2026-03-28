use crate::{
    State,
    codegen::{Writer, get_op, get_reg, instr_name, op_size, set_op},
};

pub fn codegen(w: &mut Writer, _state: &State, instr: &iced_x86::Instruction) -> bool {
    use iced_x86::Mnemonic::*;
    match instr.mnemonic() {
        // Binary operations.
        And | Or | Add | Sub | Sbb | Xor | Shl | Shr | Sar => {
            assert_eq!(instr.op_count(), 2);
            let func = instr_name(instr);
            let op0 = get_op(instr, 0);
            let op1 = get_op(instr, 1);
            w.line(set_op(
                instr,
                0,
                format!("{func}({op0}, {op1}, &mut ctx.cpu.flags)"),
            ));
        }

        Adc => {
            assert_eq!(instr.op_count(), 2);
            let op0 = get_op(instr, 0);
            let op1 = get_op(instr, 1);
            w.line("let carry = ctx.cpu.flags.contains(Flags::CF) as u32;");
            w.line(set_op(
                instr,
                0,
                format!("addc({op0}, {op1}, carry as _, &mut ctx.cpu.flags)"),
            ));
        }

        Shld => {
            assert_eq!(instr.op_count(), 3);
            let op0 = get_op(instr, 0);
            let op1 = get_op(instr, 1);
            let op2 = get_op(instr, 2);
            w.line(set_op(
                instr,
                0,
                format!("shld({op0}, {op1}, {op2}, &mut ctx.cpu.flags)"),
            ));
        }
        Shrd => w.todo(),

        Mul => {
            assert_eq!(instr.op_count(), 1);
            match op_size(instr, 0) {
                32 => {
                    w.line("let x = ctx.cpu.regs.eax;");
                    w.line(format!("let y = {};", get_op(instr, 0)));
                    w.line("let out = mul(x as u64, y as u64, &mut ctx.cpu.flags);");
                    w.line("ctx.cpu.regs.edx = (out >> 32) as u32;");
                    w.line("ctx.cpu.regs.eax = out as u32;");
                }
                16 => w.todo(),
                8 => w.todo(),
                size => todo!("{size}"),
            }
        }

        Imul => {
            let size = op_size(instr, 0);
            let (x, y) = match instr.op_count() {
                1 => match op_size(instr, 0) {
                    8 => (get_reg(iced_x86::Register::AL), get_op(instr, 0)),
                    16 => (get_reg(iced_x86::Register::AX), get_op(instr, 0)),
                    _ => todo!(),
                },
                2 => {
                    assert_eq!(op_size(instr, 0), op_size(instr, 1));
                    let op0 = get_op(instr, 0);
                    let op1 = get_op(instr, 1);
                    (op0, op1)
                }
                3 => {
                    assert_eq!(op_size(instr, 0), op_size(instr, 1));
                    assert_eq!(op_size(instr, 1), op_size(instr, 2));
                    let op1 = get_op(instr, 1);
                    let op2 = get_op(instr, 2);
                    (op1, op2)
                }
                _ => todo!(),
            };
            w.line(format!("let x = {x} as i{size}; let y = {y} as i{size};",));
            w.line(
                "let (res, overflow) = x.overflowing_mul(y);
                    ctx.cpu.flags.set(Flags::CF, overflow);
                    ctx.cpu.flags.set(Flags::OF, overflow);",
            );
            w.line(set_op(instr, 0, format!("res as u{size}")));
        }

        Idiv => {
            assert_eq!(instr.op_count(), 1);
            match op_size(instr, 0) {
                8 => {
                    w.line(
                            "let x = (((ctx.cpu.regs.get_dl() as u16) << 8) | (ctx.cpu.regs.get_al() as u16)) as i16;",
                        );
                    w.line(format!("let y = {} as i16;", get_op(instr, 0)));
                    w.line("ctx.cpu.regs.set_al((x / y) as i8 as u8);");
                    w.line("ctx.cpu.regs.set_dl((x % y) as i8 as u8);");
                }
                16 => {
                    w.line(
                            "let x = (((ctx.cpu.regs.get_dx() as u32) << 16) | (ctx.cpu.regs.get_ax() as u32)) as i32;",
                        );
                    w.line(format!("let y = {} as i32;", get_op(instr, 0)));
                    w.line("ctx.cpu.regs.set_ax((x / y) as i16 as u16);");
                    w.line("ctx.cpu.regs.set_dx((x % y) as i16 as u16);");
                }
                32 => {
                    w.line(
                            "let x = (((ctx.cpu.regs.edx as u64) << 32) | (ctx.cpu.regs.eax as u64)) as i64;",
                        );
                    w.line(format!("let y = {} as i64;", get_op(instr, 0)));
                    w.line("ctx.cpu.regs.eax = (x / y) as i32 as u32;");
                    w.line("ctx.cpu.regs.edx = (x % y) as i32 as u32;");
                }
                _ => todo!(),
            }
        }

        Neg => w.line(set_op(
            instr,
            0,
            format!("neg({}, &mut ctx.cpu.flags)", get_op(instr, 0)),
        )),

        Dec => w.line(set_op(
            instr,
            0,
            format!("dec({}, &mut ctx.cpu.flags)", get_op(instr, 0)),
        )),
        Inc => w.line(set_op(
            instr,
            0,
            format!("inc({}, &mut ctx.cpu.flags)", get_op(instr, 0)),
        )),

        _ => return false,
    }
    true
}
