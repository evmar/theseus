use crate::codegen::{CodeGen, instr_name, op_size};

impl<'a> CodeGen<'a> {
    pub fn codegen_misc(&mut self, instr: &iced_x86::Instruction) -> bool {
        use iced_x86::Mnemonic::*;
        match instr.mnemonic() {
            Push => {
                let func = match op_size(instr, 0) {
                    16 => "push16",
                    32 => "push32",
                    _ => return false,
                };
                self.line(format!("ctx.{func}({});", self.get_op(instr, 0)));
            }
            Pop => {
                let func = match op_size(instr, 0) {
                    16 => "pop16",
                    32 => "pop32",
                    _ => return false,
                };
                self.line(format!("let x = ctx.{func}();"));
                self.line(self.set_op(instr, 0, "x".into()))
            }
            Pushad => self.line("ctx.pushad();"),
            Popad => self.line("ctx.popad();"),
            Mov => self.line(self.set_op(instr, 0, self.get_op(instr, 1))),

            Sete | Setg | Setge | Setne => {
                self.line(self.set_op(instr, 0, format!("ctx.{}()", instr_name(instr))))
            }

            Cmp => {
                let op0 = self.get_op(instr, 0);
                let op1 = self.get_op(instr, 1);
                self.line(format!("sub({op0}, {op1}, &mut ctx.cpu.flags);"));
            }
            Test => {
                self.line(format!(
                    "and({}, {}, &mut ctx.cpu.flags);",
                    self.get_op(instr, 0),
                    self.get_op(instr, 1)
                ));
            }

            Lea => {
                // Note: in 16-bit mode lea ignores segment registers.
                let addr = self.gen_addr_offset(instr);
                self.line(self.set_op(instr, 0, addr));
            }

            Movzx => {
                self.line(self.set_op(instr, 0, format!("{} as _", self.get_op(instr, 1))));
            }
            Movsx => {
                let read = format!(
                    "{read} as i{src} as i{dst} as u{dst}",
                    read = self.get_op(instr, 1),
                    src = op_size(instr, 1),
                    dst = op_size(instr, 0)
                );
                self.line(self.set_op(instr, 0, read));
            }

            Leave => self.line("ctx.leave();"),
            Enter => {
                assert!(instr.op1_kind() == iced_x86::OpKind::Immediate8_2nd);
                let op1 = instr.immediate8_2nd();
                self.line(format!("ctx.enter({}, {:x});", self.get_op(instr, 0), op1));
            }

            Xchg => {
                self.line(format!("let t = {};", self.get_op(instr, 0)));
                self.line(self.set_op(instr, 0, self.get_op(instr, 1)));
                self.line(self.set_op(instr, 1, "t".into()));
            }
            Nop => {}

            Not => self.line(self.set_op(instr, 0, format!("!{}", self.get_op(instr, 0)))),

            Int => {
                assert!(instr.op0_kind() == iced_x86::OpKind::Immediate8);
                assert_eq!(self.module.bitness, 16);
                self.line(format!("dos::int{:x}(ctx);", instr.immediate8()));
            }
            Int3 | Cmpxchg | Pushfd | Cpuid | Xgetbv | Bt | Div => self.todo(),

            // CBW/CWDE: sign extend to next larger ax
            Cbw => self.line("ctx.cpu.regs.set_ax(ctx.cpu.regs.get_al() as i8 as i16 as u16);"),
            Cwde => self.line("ctx.cpu.regs.eax = ctx.cpu.regs.get_ax() as i16 as i32 as u32;"),

            // CWD/CDQ: sign extend to dx:ax
            Cwd => self.line("ctx.cpu.regs.set_dx_ax(ctx.cpu.regs.get_ax() as i16 as i32 as u32);"),
            Cdq => self.line("ctx.cpu.regs.set_edx_eax(ctx.cpu.regs.eax as i32 as i64 as u64);"),

            Stc | Clc | Std | Cld | Sahf => {
                self.line(format!("{}(ctx);", instr_name(instr)));
            }

            Cli | Sti => {
                self.line(format!("ctx.{}();", instr_name(instr)));
            }

            Out => {
                assert_eq!(instr.op_count(), 2);
                let port = if instr.op0_kind() == iced_x86::OpKind::Immediate8 {
                    // The Imm8 form can only reference the first 256 ports, but otherwise it's the same call.
                    format!("{:#x}u16", instr.immediate8())
                } else {
                    self.get_op(instr, 0)
                };
                self.line(format!("dos::out(ctx, {port}, {});", self.get_op(instr, 1)));
            }

            _ => return false,
        }
        true
    }
}
