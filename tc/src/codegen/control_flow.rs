use std::collections::HashMap;

use crate::{
    codegen::{CodeGen, gen_addr, get_reg, instr_name},
    is_abs_memory_ref,
};

/// If instr is call [foo] where foo is a fn in the IAT, return the name of the
/// actual symbol (e.g `user32::CreateWindow_stdcall`) that reference resolves to.
fn is_iat_ref(instr: &iced_x86::Instruction, iat_refs: &HashMap<u32, String>) -> Option<String> {
    if instr.op_kind(0) != iced_x86::OpKind::Memory {
        return None;
    }
    let Some(addr) = is_abs_memory_ref(instr) else {
        return None;
    };
    let Some(func) = iat_refs.get(&addr) else {
        return None;
    };
    return Some(format!("{func}_stdcall"));
}

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
                // If it's like `jmp [someaddr]` where someaddr is in the IAT, resolve it directly.
                // (Note that `call [someaddr@IAT]` is resolved to a direct function call.)
                if let Some(func) = is_iat_ref(instr, &self.iat_refs) {
                    return format!("Cont({func})");
                }
                format!("indirect(ctx, ctx.memory.read({}))", gen_addr(instr))
            }
            iced_x86::OpKind::Register => {
                format!("indirect(ctx, {})", get_reg(instr.op0_register()))
            }
            k => todo!("{:?}", k),
        }
    }

    pub fn codegen_control_flow(&mut self, instr: &iced_x86::Instruction) -> bool {
        use iced_x86::Mnemonic::*;
        match instr.mnemonic() {
            Jmp => self.line(self.gen_jmp(instr)),
            Call => {
                if let Some(call) = is_iat_ref(instr, &self.iat_refs) {
                    self.line(format!("call_builtin(ctx, {call});",));
                    self.line(self.gen_abs_jmp(instr.next_ip32()))
                } else {
                    // Create a temporary here in case gen_jmp needs to borrow ctx.
                    self.line(format!("let dst = {};", self.gen_jmp(instr)));
                    self.line(format!("call(ctx, {:#x}, dst)", instr.next_ip32()));
                }
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
                self.line(format!("ret(ctx, {n})"));
            }
            Je | Jne | Jb | Js | Jns | Ja | Jae | Jl | Jg | Jge | Jecxz | Jle | Jbe => {
                let next = self.gen_abs_jmp(instr.next_ip32());
                let dst = self.gen_jmp(instr);
                let func = instr_name(instr);
                self.line(format!("{func}(ctx, {next}, {dst})"));
            }

            Loop => {
                let next = self.gen_abs_jmp(instr.next_ip32());
                let dst = self.gen_jmp(instr);
                self.line("ctx.cpu.regs.ecx = ctx.cpu.regs.ecx.wrapping_sub(1);");
                self.line(format!("if ctx.cpu.regs.ecx == 0 {{ {next} }}"));
                self.line(format!("else {{ {dst} }}"));
            }

            _ => return false,
        }
        true
    }
}
