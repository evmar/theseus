use crate::codegen::{CodeGen, gen_addr, get_op, instr_name, op_size, set_op};

impl<'a> CodeGen<'a> {
    pub fn codegen_misc(&mut self, instr: &iced_x86::Instruction) -> bool {
        use iced_x86::Mnemonic::*;
        match instr.mnemonic() {
            Push => {
                let func = match op_size(instr, 0) {
                    16 => "push16",
                    32 => "push",
                    _ => return false,
                };
                self.line(format!("{func}(ctx, {});", get_op(instr, 0)));
            }
            Pop => {
                let func = match op_size(instr, 0) {
                    16 => "pop16",
                    32 => "pop",
                    _ => return false,
                };
                self.line(format!("let x = {func}(ctx);"));
                self.line(set_op(instr, 0, "x".into()))
            }
            Pushad => self.line("pushad(ctx);"),
            Popad => self.line("popad(ctx);"),
            Mov => self.line(set_op(instr, 0, get_op(instr, 1))),

            Sete => self.line(set_op(instr, 0, "sete(ctx)".into())),
            Cmp => {
                let op0 = get_op(instr, 0);
                let op1 = get_op(instr, 1);
                self.line(format!("sub({op0}, {op1}, &mut ctx.cpu.flags);"));
            }
            Test => {
                self.line(format!(
                    "and({}, {}, &mut ctx.cpu.flags);",
                    get_op(instr, 0),
                    get_op(instr, 1)
                ));
            }

            Lea => self.line(format!("{} = {};", get_op(instr, 0), gen_addr(instr))),

            Movzx => {
                self.line(set_op(instr, 0, format!("{} as _", get_op(instr, 1))));
            }
            Movsx => {
                let read = format!(
                    "{read} as i{src} as i{dst} as u{dst}",
                    read = get_op(instr, 1),
                    src = op_size(instr, 1),
                    dst = op_size(instr, 0)
                );
                self.line(set_op(instr, 0, read));
            }

            Leave => self.line("leave(ctx);"),
            Enter => {
                assert!(instr.op1_kind() == iced_x86::OpKind::Immediate8_2nd);
                let op1 = instr.immediate8_2nd();
                self.line(format!("enter(ctx, {}, {:x});", get_op(instr, 0), op1));
            }

            Xchg => {
                self.line(format!("let t = {};", get_op(instr, 0)));
                self.line(set_op(instr, 0, get_op(instr, 1)));
                self.line(set_op(instr, 1, "t".into()));
            }
            Nop => {}

            Not | Int | Int3 | Cmpxchg | Pushfd | Setne | Cpuid | Xgetbv | Setg | Bt | Div
            | Setge => self.todo(),

            Cbw => self.line("ctx.cpu.regs.set_ax(ctx.cpu.regs.get_al() as i8 as i16 as u16);"),
            Cwde => self.line("ctx.cpu.regs.eax = ctx.cpu.regs.get_ax() as i16 as i32 as u32;"),
            Cdq => {
                self.line("let t = ctx.cpu.regs.eax as i32 as i64 as u64;");
                self.line("ctx.cpu.regs.edx = (t >> 32) as u32;");
                self.line("ctx.cpu.regs.eax = t as u32;");
            }

            Stc | Clc | Std | Cld | Sahf => {
                self.line(format!("{}(ctx);", instr_name(instr)));
            }

            _ => return false,
        }
        true
    }
}
