use crate::{
    Instr,
    codegen::{CodeGen, get_reg, instr_name},
    gather::IP,
};

impl<'a> CodeGen<'a> {
    /// Codegen the Cont for a jump to an statically known address.
    /// This should always resolve to a real symbol at translation time.
    fn resolve_jmp(&self, ip: IP) -> String {
        if let Some(block) = self.blocks.get(&ip.to_addr()) {
            format!("Cont({})", block.name())
        } else {
            format!("todo!(\"static jmp to unknown block {}\")", ip)
        }
    }

    /// Codegen the Cont for a jump or call instruction.
    /// Returns (code, uses_ctx, far) where
    ///   uses_ctx is true if code uses ctx (needed for lifetime reasons)
    ///   far is the target segment for far jmps/calls
    fn gen_jmp(&self, instr: &Instr) -> (String, bool, Option<u16>) {
        assert_eq!(instr.iced.op_count(), 1);
        let expr: String;
        let mut uses_ctx = false;
        let mut far = None;
        match instr.iced.op0_kind() {
            iced_x86::OpKind::NearBranch16 => {
                let ip = instr.ip.with_local(instr.iced.near_branch16() as u32);
                expr = self.resolve_jmp(ip);
            }
            iced_x86::OpKind::NearBranch32 => {
                let ip = instr.ip.with_local(instr.iced.near_branch32());
                expr = self.resolve_jmp(ip);
            }
            iced_x86::OpKind::FarBranch16 => {
                let ip = IP::Seg(instr.iced.far_branch_selector(), instr.iced.far_branch16());
                expr = self.resolve_jmp(ip);
                far = Some(instr.iced.far_branch_selector());
            }
            iced_x86::OpKind::Memory => {
                // If it's like `jmp [someaddr]` where someaddr is in the IAT, resolve it directly.
                // (Note that `call [someaddr@IAT]` is generated as a direct function call.)
                if let Some(func) = &instr.hint {
                    return (format!("Cont({func})"), false, None);
                }

                // TODO: what about far calls?
                let indirect = if self.module.bitness() == 16 {
                    "indirect_near"
                } else {
                    "indirect32"
                };
                expr = format!(
                    "ctx.{indirect}(ctx.memory.read({addr}))",
                    addr = self.gen_addr(&instr.iced)
                );
                uses_ctx = true;
            }
            iced_x86::OpKind::Register => {
                let indirect = if self.module.bitness() == 16 {
                    "indirect_near"
                } else {
                    "indirect32"
                };
                expr = format!(
                    "ctx.{indirect}({reg})",
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
            Jmp => {
                let (cont, _, far) = self.gen_jmp(instr);
                if let Some(seg) = far {
                    self.line(format!("ctx.cpu.regs.cs = {seg:#x};"));
                }
                self.line(cont);
            }
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
                    if let Some(seg) = far {
                        self.line(format!(
                            "ctx.callf16({ip:#x}, {seg:#x}, {dst})",
                            ip = instr.next_ip().local()
                        ));
                    } else {
                        self.line(format!(
                            "ctx.call{bitness}({ip:#x}, {dst})",
                            bitness = self.module.bitness(),
                            ip = instr.next_ip().local()
                        ));
                    }
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
                let next = self.resolve_jmp(instr.next_ip());
                let dst = self.gen_jmp(instr).0;
                let func = instr_name(&instr.iced);
                self.line(format!("ctx.{func}({next}, {dst})"));
            }

            _ => return false,
        }
        true
    }
}
