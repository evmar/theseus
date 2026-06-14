use crate::{
    Instr,
    codegen::{CodeGen, get_reg, instr_name},
};

impl<'a> CodeGen<'a> {
    fn gen_abs_jmp(&self, addr: u32) -> String {
        if let Some(block) = self.blocks.get(&addr) {
            format!("Cont({})", block.name())
        } else {
            format!("todo!(\"indirect jmp to {:#x}?\")", addr)
        }
    }

    /// Returns (code, uses_ctx) where uses_ctx is true if code uses ctx.
    fn gen_jmp(&self, instr: &Instr) -> (String, bool) {
        match instr.iced.op_kind(0) {
            iced_x86::OpKind::NearBranch16 => {
                let addr = instr.iced.near_branch16() as u32;
                (self.gen_abs_jmp(addr), false)
            }
            iced_x86::OpKind::NearBranch32 => {
                let addr = instr.iced.near_branch32();
                (self.gen_abs_jmp(addr), false)
            }
            iced_x86::OpKind::Memory => {
                // If it's like `jmp [someaddr]` where someaddr is in the IAT, resolve it directly.
                // (Note that `call [someaddr@IAT]` is generated as a direct function call.)
                if let Some(func) = &instr.hint {
                    return (format!("Cont({func})"), false);
                }
                (
                    format!(
                        "ctx.indirect(ctx.memory.read({}))",
                        self.gen_addr(&instr.iced)
                    ),
                    true,
                )
            }
            iced_x86::OpKind::Register => (
                format!("ctx.indirect({})", get_reg(instr.iced.op0_register())),
                true,
            ),
            k => todo!("{:?}", k),
        }
    }

    pub fn codegen_control_flow(&mut self, instr: &Instr) -> bool {
        use iced_x86::Mnemonic::*;
        match instr.iced.mnemonic() {
            Jmp => self.line(self.gen_jmp(instr).0),
            Call => {
                if let Some(func) = &instr.hint {
                    self.line(format!("ctx.call_builtin({:#x}, {func});", instr.next_ip()));
                } else {
                    let (dst, uses_ctx) = self.gen_jmp(instr);
                    let dst = if uses_ctx {
                        self.line(format!("let dst = {};", dst));
                        "dst".into()
                    } else {
                        dst
                    };
                    self.line(format!(
                        "ctx.call{bitness}({ip:#x}, {dst})",
                        bitness = self.module.bitness,
                        ip = instr.next_ip()
                    ));
                }
            }
            Ret => {
                let n = match instr.iced.op_count() {
                    0 => 0,
                    1 => {
                        assert!(instr.iced.op0_kind() == iced_x86::OpKind::Immediate16);
                        instr.iced.immediate16()
                    }
                    _ => todo!(),
                };
                self.line(format!(
                    "ctx.ret{bitness}({n})",
                    bitness = self.module.bitness
                ));
            }
            Iret => {
                self.line(format!(
                    "ctx.iret{bitness}()",
                    bitness = self.module.bitness
                ));
            }
            Je | Jne | Jb | Js | Jns | Ja | Jae | Jl | Jg | Jge | Jecxz | Jle | Jbe | Loop
            | Loopne => {
                let next = self.gen_abs_jmp(instr.next_ip());
                let dst = self.gen_jmp(instr).0;
                let func = instr_name(&instr.iced);
                self.line(format!("ctx.{func}({next}, {dst})"));
            }

            _ => return false,
        }
        true
    }
}
