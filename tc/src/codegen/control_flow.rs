use crate::{
    Instr,
    codegen::{CodeGen, get_reg, instr_name},
    gather::IP,
};

impl<'a> CodeGen<'a> {
    /// Codegen the Cont for a jump to an absolute address.
    /// This should always resolve to a real symbol at translation time.
    fn gen_abs_jmp(&self, ip: IP) -> String {
        if let Some(block) = self.blocks.get(&ip.to_addr()) {
            format!("Cont({})", block.name())
        } else {
            format!("todo!(\"indirect jmp to unknown block {}?\")", ip)
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
                let ip = instr.ip.with_local(instr.iced.near_branch16() as u32);
                expr = self.gen_abs_jmp(ip);
            }
            iced_x86::OpKind::NearBranch32 => {
                let ip = instr.ip.with_local(instr.iced.near_branch32());
                expr = self.gen_abs_jmp(ip);
            }
            iced_x86::OpKind::FarBranch16 => {
                let ip = IP::Seg(instr.iced.far_branch_selector(), instr.iced.far_branch16());
                expr = format!("todo!(\"far jmp {ip}\")");
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
                    self.line(format!(
                        "ctx.call_builtin({:#x}, {func});",
                        instr.next_ip().local()
                    ));
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
                        ip = instr.next_ip().local()
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
