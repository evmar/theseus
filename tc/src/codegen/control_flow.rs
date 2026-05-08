use crate::{
    Instr,
    codegen::{CodeGen, gen_addr, get_reg, instr_name},
};

impl<'a> CodeGen<'a> {
    fn gen_abs_jmp(&self, addr: u32) -> String {
        if let Some(block) = self.blocks.get(&addr) {
            format!("Cont({})", block.name())
        } else {
            format!("todo!(\"indirect jmp to {:#x}?\")", addr)
        }
    }

    fn gen_jmp(&self, instr: &Instr) -> String {
        match instr.iced.op_kind(0) {
            iced_x86::OpKind::NearBranch32 => {
                let addr = instr.iced.near_branch32();
                self.gen_abs_jmp(addr)
            }
            iced_x86::OpKind::Memory => {
                // If it's like `jmp [someaddr]` where someaddr is in the IAT, resolve it directly.
                // (Note that `call [someaddr@IAT]` is generated as a direct function call.)
                if let Some(func) = &instr.hint {
                    return format!("Cont({func})");
                }
                format!("indirect(ctx, ctx.memory.read({}))", gen_addr(&instr.iced))
            }
            iced_x86::OpKind::Register => {
                format!("indirect(ctx, {})", get_reg(instr.iced.op0_register()))
            }
            k => todo!("{:?}", k),
        }
    }

    pub fn codegen_control_flow(&mut self, instr: &Instr) -> bool {
        use iced_x86::Mnemonic::*;
        match instr.iced.mnemonic() {
            Jmp => self.line(self.gen_jmp(instr)),
            Call => {
                if let Some(func) = &instr.hint {
                    self.line(format!("ctx.call_builtin({func});"));
                } else {
                    // Create a temporary here in case gen_jmp needs to borrow ctx.
                    self.line(format!("let dst = {};", self.gen_jmp(instr)));
                    self.line(format!("ctx.call({:#x}, dst)", instr.next_ip()));
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
                self.line(format!("ctx.ret({n})"));
            }
            Je | Jne | Jb | Js | Jns | Ja | Jae | Jl | Jg | Jge | Jecxz | Jle | Jbe | Loop => {
                let next = self.gen_abs_jmp(instr.next_ip());
                let dst = self.gen_jmp(instr);
                let func = instr_name(&instr.iced);
                self.line(format!("ctx.{func}({next}, {dst})"));
            }

            _ => return false,
        }
        true
    }
}
