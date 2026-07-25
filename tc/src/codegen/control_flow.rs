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

    fn jmp_target(&self, instr: &Instr) -> (Option<String>, Option<String>, String) {
        assert_eq!(instr.iced.op_count(), 1);
        let mut extra: Option<String> = None;
        let mut seg: Option<String> = None;
        let cont: String;
        match instr.iced.op0_kind() {
            iced_x86::OpKind::NearBranch16 => {
                let ip = instr.ip.with_local(instr.iced.near_branch16() as u32);
                cont = self.resolve_jmp(ip);
            }
            iced_x86::OpKind::NearBranch32 => {
                let ip = instr.ip.with_local(instr.iced.near_branch32());
                cont = self.resolve_jmp(ip);
            }
            iced_x86::OpKind::FarBranch16 => {
                let ip = IP::Seg(instr.iced.far_branch_selector(), instr.iced.far_branch16());
                seg = Some(format!("{:#x}", instr.iced.far_branch_selector()));
                cont = self.resolve_jmp(ip);
            }
            iced_x86::OpKind::Memory => {
                // If it's like `jmp [someaddr]` where someaddr is in the IAT, resolve it directly.
                // (Note that `call [someaddr@IAT]` is generated as a direct function call.)
                if let Some(func) = &instr.hint {
                    return (None, None, format!("Cont({func})"));
                }

                let addr = self.gen_addr(&instr.iced);
                match instr.iced.memory_size() {
                    iced_x86::MemorySize::SegPtr16 => {
                        extra = Some(format!("let addr = ctx.memory.read::<SegOfs>({addr});"));
                        seg = Some("addr.seg".into());
                        cont = "ctx.indirect16(addr)".into();
                    }
                    iced_x86::MemorySize::WordOffset => {
                        extra = Some(format!("let addr = ctx.memory.read::<u16>({addr});"));
                        cont = "ctx.indirect16((ctx.cpu.regs.cs, addr).into())".into();
                    }
                    iced_x86::MemorySize::DwordOffset => {
                        extra = Some(format!("let addr = ctx.memory.read::<u32>({addr});"));
                        cont = "ctx.indirect32(addr)".into();
                    }
                    s => todo!("{s:?}"),
                }
            }
            iced_x86::OpKind::Register => {
                if self.module.bitness() == 16 {
                    cont = format!(
                        "ctx.indirect16((ctx.cpu.regs.cs, {reg}).into())",
                        reg = get_reg(instr.iced.op0_register())
                    );
                } else {
                    cont = format!(
                        "ctx.indirect({reg})",
                        reg = get_reg(instr.iced.op0_register())
                    );
                }
            }
            k => todo!("{:?}", k),
        }
        (extra, seg, cont)
    }

    pub fn codegen_control_flow(&mut self, instr: &Instr) -> bool {
        use iced_x86::Mnemonic::*;
        match instr.iced.mnemonic() {
            Jmp => {
                let (extra, seg, cont) = self.jmp_target(instr);
                if let Some(extra) = extra {
                    self.line(extra);
                }
                if let Some(seg) = seg {
                    self.line(format!("ctx.cpu.regs.cs = {seg};"));
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
                    let (extra, seg, cont) = self.jmp_target(instr);
                    if let Some(extra) = extra {
                        self.line(extra);
                    }
                    if let Some(seg) = seg {
                        self.line(format!(
                            "ctx.callf16({ip:#x}, {seg}, {cont})",
                            ip = instr.next_ip().local()
                        ));
                    } else {
                        self.line(format!(
                            "ctx.call{bitness}({ip:#x}, {cont})",
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
                let (None, None, cont) = self.jmp_target(instr) else {
                    panic!()
                };
                let func = instr_name(&instr.iced);
                self.line(format!("ctx.{func}({next}, {cont})"));
            }

            _ => return false,
        }
        true
    }
}
