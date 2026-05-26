use crate::codegen::{CodeGen, get_op, get_reg, instr_name, op_size, set_op};

impl<'a> CodeGen<'a> {
    pub fn codegen_math(&mut self, instr: &iced_x86::Instruction) -> bool {
        use iced_x86::Mnemonic::*;
        match instr.mnemonic() {
            // Binary operations.
            And | Or | Add | Sub | Sbb | Xor | Shl | Shr | Sar | Rol | Rcl | Rcr => {
                assert_eq!(instr.op_count(), 2);
                let func = instr_name(instr);
                let op0 = get_op(instr, 0);
                let op1 = get_op(instr, 1);
                self.line(set_op(
                    instr,
                    0,
                    format!("{func}({op0}, {op1}, &mut ctx.cpu.flags)"),
                ));
            }

            Adc => {
                assert_eq!(instr.op_count(), 2);
                let op0 = get_op(instr, 0);
                let op1 = get_op(instr, 1);
                self.line("let carry = ctx.cpu.flags.contains(Flags::CF) as u32;");
                self.line(set_op(
                    instr,
                    0,
                    format!("addc({op0}, {op1}, carry as _, &mut ctx.cpu.flags)"),
                ));
            }

            Shld | Shrd => {
                assert_eq!(instr.op_count(), 3);
                let op0 = get_op(instr, 0);
                let op1 = get_op(instr, 1);
                let op2 = get_op(instr, 2);
                self.line(set_op(
                    instr,
                    0,
                    format!(
                        "{op}({op0}, {op1}, {op2}, &mut ctx.cpu.flags)",
                        op = instr_name(instr)
                    ),
                ));
            }

            Mul => {
                assert_eq!(instr.op_count(), 1);
                let size = op_size(instr, 0);
                let size2 = size * 2;
                self.line(format!("let res = mul(ctx.cpu.regs.eax as u{size} as u{size2}, {} as u{size2}, &mut ctx.cpu.flags);", get_op(instr, 0)));
                match size {
                    8 => self.line(format!("ctx.cpu.regs.set_ax(res);")),
                    16 => self.line(format!("ctx.cpu.regs.set_dx_ax(res);")),
                    32 => self.line(format!("ctx.cpu.regs.set_edx_eax(res);")),
                    _ => unreachable!(),
                }
            }

            Div => {
                assert_eq!(instr.op_count(), 1);
                let size = op_size(instr, 0);
                let size2 = size * 2;
                let x = match size {
                    8 => get_reg(iced_x86::Register::AX),
                    16 => format!("ctx.cpu.regs.get_dx_ax()"),
                    32 => format!("ctx.cpu.regs.get_edx_eax()"),
                    _ => unreachable!(),
                };
                let y = format!("{} as u{size2}", get_op(instr, 0));
                self.line(format!("let (quot, rem) = div({x}, {y});"));
                match size {
                    8 => {
                        self.line("ctx.cpu.regs.set_al(quot as u8);");
                        self.line("ctx.cpu.regs.set_ah(rem as u8);");
                    }
                    16 => {
                        self.line("ctx.cpu.regs.set_ax(quot as u16);");
                        self.line("ctx.cpu.regs.set_dx(rem as u16);");
                    }
                    32 => {
                        self.line("ctx.cpu.regs.eax = quot as u32;");
                        self.line("ctx.cpu.regs.edx = rem as u32;");
                    }
                    _ => unreachable!(),
                }
            }

            Imul => {
                let size = op_size(instr, 0);
                if instr.op_count() == 1 {
                    // one-op imul has different in/out reg and overflow behavior from others
                    let x = format!("{} as u{size}", get_reg(iced_x86::Register::EAX));
                    let y = get_op(instr, 0);
                    let res = format!("imul1_{size}({x}, {y}, &mut ctx.cpu.flags)");
                    match size {
                        8 => self.line(format!("ctx.cpu.regs.set_ax({res});")),
                        16 => self.line(format!("ctx.cpu.regs.set_dx_ax({res});")),
                        32 => self.line(format!("ctx.cpu.regs.set_edx_eax({res});")),
                        _ => unreachable!(),
                    }
                } else {
                    let (x, y) = match instr.op_count() {
                        2 => {
                            assert_eq!(op_size(instr, 0), op_size(instr, 1));
                            (get_op(instr, 0), get_op(instr, 1))
                        }
                        3 => {
                            assert_eq!(op_size(instr, 0), op_size(instr, 1));
                            assert_eq!(op_size(instr, 1), op_size(instr, 2));
                            (get_op(instr, 1), get_op(instr, 2))
                        }
                        _ => unreachable!(),
                    };
                    self.line(set_op(
                        instr,
                        0,
                        format!("imul2_{size}({x}, {y}, &mut ctx.cpu.flags)"),
                    ));
                }
            }

            Idiv => {
                assert_eq!(instr.op_count(), 1);
                let size = op_size(instr, 0);
                let size2 = size * 2;
                let x = match size {
                    8 => format!("ctx.cpu.regs.get_ax() as i16"),
                    16 => format!("ctx.cpu.regs.get_dx_ax() as i32"),
                    32 => format!("ctx.cpu.regs.get_edx_eax() as i64"),
                    _ => unreachable!(),
                };
                let y = format!("{} as i{size} as i{size2}", get_op(instr, 0));
                self.line(format!("let x = {x};"));
                self.line(format!("let y = {y};"));
                let quot = format!("(x / y) as i{size} as u{size}");
                let rem = format!("(x % y) as i{size} as u{size}");
                match size {
                    8 => {
                        self.line(format!("ctx.cpu.regs.set_al({quot});"));
                        self.line(format!("ctx.cpu.regs.set_ah({rem});"));
                    }
                    16 => {
                        self.line(format!("ctx.cpu.regs.set_ax({quot});"));
                        self.line(format!("ctx.cpu.regs.set_dx({rem});"));
                    }
                    32 => {
                        self.line(format!("ctx.cpu.regs.eax = {quot};"));
                        self.line(format!("ctx.cpu.regs.edx = {rem};"));
                    }
                    _ => unreachable!(),
                }
            }

            Neg => self.line(set_op(
                instr,
                0,
                format!("neg({}, &mut ctx.cpu.flags)", get_op(instr, 0)),
            )),

            Dec => self.line(set_op(
                instr,
                0,
                format!("dec({}, &mut ctx.cpu.flags)", get_op(instr, 0)),
            )),
            Inc => self.line(set_op(
                instr,
                0,
                format!("inc({}, &mut ctx.cpu.flags)", get_op(instr, 0)),
            )),

            _ => return false,
        }
        true
    }
}
