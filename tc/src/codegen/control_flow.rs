use crate::{
    codegen::{CodeGen, Writer, gen_addr, get_reg, instr_name},
    is_abs_memory_ref,
};

impl<'a> CodeGen<'a> {
    fn gen_abs_jmp(&self, addr: u32) -> String {
        if self.blocks.contains_key(&addr) {
            format!("Cont(x{:x})", addr)
        } else {
            format!("todo!(\"indirect jmp to {:#x}?\")", addr)
        }
    }

    fn gen_jmp(&self, instr: &iced_x86::Instruction) -> String {
        match instr.op_kind(0) {
            iced_x86::OpKind::NearBranch32 => {
                let addr = instr.near_branch32();
                self.gen_abs_jmp(addr)
            }
            iced_x86::OpKind::Memory => {
                // If it's like `call [someaddr]` where someaddr is in the IAT, resolve it directly.
                if let Some(addr) = is_abs_memory_ref(instr) {
                    if let Some(func) = self.iat_refs.get(&addr) {
                        return format!("Cont({func}_stdcall)");
                    }
                }
                format!("indirect(ctx, ctx.memory.read({}))", gen_addr(instr))
            }
            iced_x86::OpKind::Register => {
                format!("indirect(ctx, {})", get_reg(instr.op0_register()))
            }
            k => todo!("{:?}", k),
        }
    }

    pub fn codegen_control_flow(&self, w: &mut Writer, instr: &iced_x86::Instruction) -> bool {
        use iced_x86::Mnemonic::*;
        match instr.mnemonic() {
            Jmp => w.line(self.gen_jmp(instr)),
            Call => {
                // Create a temporary here in case gen_jmp needs to borrow ctx.
                w.line(format!("let dst = {};", self.gen_jmp(instr)));
                w.line(format!("call(ctx, {:#x}, dst)", instr.next_ip32()));
            }
            Ret => {
                let n = match instr.op_count() {
                    0 => 0,
                    1 => {
                        assert!(instr.op0_kind() == iced_x86::OpKind::Immediate16);
                        instr.immediate16()
                    }
                    _ => todo!(),
                };
                w.line(format!("ret(ctx, {n})"));
            }
            Je | Jne | Jb | Js | Jns | Ja | Jae | Jl | Jg | Jge | Jecxz | Jle | Jbe => {
                let next = self.gen_abs_jmp(instr.next_ip32());
                let dst = self.gen_jmp(instr);
                let func = instr_name(instr);
                w.line(format!("{func}(ctx, {next}, {dst})"));
            }

            Loop => {
                let next = self.gen_abs_jmp(instr.next_ip32());
                let dst = self.gen_jmp(instr);
                w.line("ctx.cpu.regs.ecx = ctx.cpu.regs.ecx.wrapping_sub(1);");
                w.line(format!("if ctx.cpu.regs.ecx == 0 {{ {next} }}"));
                w.line(format!("else {{ {dst} }}"));
            }

            _ => return false,
        }
        true
    }
}
