use crate::{
    Instr, Module,
    codegen::{CodeGen, get_reg, instr_name},
};

impl<'a> CodeGen<'a> {
    /// Codegen the Cont for a jump to an absolute address.
    /// This should always resolve to a real symbol at translation time.
    fn gen_abs_jmp(&self, addr: u32) -> String {
        if let Some(block) = self.blocks.get(&addr) {
            format!("Cont({})", block.name())
        } else {
            format!("todo!(\"indirect jmp to unknown block {:#x}?\")", addr)
        }
    }

    /// Codegen the Cont for a jump or call instruction.
    /// Returns (code, uses_ctx, far) where
    ///   uses_ctx is true if code uses ctx (needed for lifetime reasons)
    ///   far is true for far jmps/calls
    fn gen_jmp(&self, instr: &Instr) -> (String, bool, bool) {
        assert_eq!(instr.iced.op_count(), 1);
        let expr: String;
        let mut uses_ctx = false;
        let mut far = false;
        match instr.iced.op0_kind() {
            iced_x86::OpKind::NearBranch16 => {
                let addr = instr.iced.near_branch16() as u32;
                expr = self.gen_abs_jmp(addr);
            }
            iced_x86::OpKind::NearBranch32 => {
                let addr = instr.iced.near_branch32();
                expr = self.gen_abs_jmp(addr);
            }
            iced_x86::OpKind::FarBranch16 => {
                let Module::DOS(m) = self.module else {
                    unreachable!()
                };
                let seg = instr.iced.far_branch_selector();
                let addr = instr.iced.far_branch16();
                if seg == m.load_segment {
                    expr = self.gen_abs_jmp(addr as u32);
                } else {
                    expr = format!("todo!(\"far jmp to alternative seg\")");
                }
                far = true;
            }
            iced_x86::OpKind::Memory => {
                // If it's like `jmp [someaddr]` where someaddr is in the IAT, resolve it directly.
                // (Note that `call [someaddr@IAT]` is generated as a direct function call.)
                if let Some(func) = &instr.hint {
                    return (format!("Cont({func})"), false, false);
                }

                // TODO: what about far calls?
                expr = format!(
                    "ctx.indirect{bitness}(ctx.memory.read({addr}))",
                    bitness = self.module.bitness(),
                    addr = self.gen_addr(&instr.iced)
                );
                uses_ctx = true;
            }
            iced_x86::OpKind::Register => {
                expr = format!(
                    "ctx.indirect{bitness}({reg})",
                    bitness = self.module.bitness(),
                    reg = get_reg(instr.iced.op0_register())
                );
                uses_ctx = true;
            }
            k => todo!("{:?}", k),
        }
        (expr, uses_ctx, far)
    }

    pub fn codegen_control_flow(&mut self, instr: &Instr) -> bool {
        use iced_x86::Mnemonic::*;
        match instr.iced.mnemonic() {
            Jmp => self.line(self.gen_jmp(instr).0),
            Call => {
                if let Some(func) = &instr.hint {
                    self.line(format!("ctx.call_builtin({:#x}, {func});", instr.next_ip()));
                } else {
                    let (dst, uses_ctx, far) = self.gen_jmp(instr);
                    let dst = if uses_ctx {
                        self.line(format!("let dst = {};", dst));
                        "dst".into()
                    } else {
                        dst
                    };
                    self.line(format!(
                        "ctx.call{far}{bitness}({ip:#x}, {dst})",
                        far = if far { "f" } else { "" },
                        bitness = self.module.bitness(),
                        ip = instr.next_ip()
                    ));
                }
            }
            Ret | Retf => {
                let n = match instr.iced.op_count() {
                    0 => 0,
                    1 => {
                        assert!(instr.iced.op0_kind() == iced_x86::OpKind::Immediate16);
                        instr.iced.immediate16()
                    }
                    _ => todo!(),
                };
                self.line(format!(
                    "ctx.{name}{bitness}({n})",
                    name = instr_name(&instr.iced),
                    bitness = self.module.bitness()
                ));
            }
            Iret => {
                self.line(format!(
                    "ctx.iret{bitness}()",
                    bitness = self.module.bitness()
                ));
            }
            Je | Jne | Jb | Js | Jns | Ja | Jae | Jl | Jg | Jge | Jecxz | Jle | Jbe | Jcxz
            | Loop | Loopne => {
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
