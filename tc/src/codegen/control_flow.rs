use crate::{
    State,
    codegen::{Writer, gen_addr, get_reg, instr_name},
    is_abs_memory_ref,
};

fn gen_abs_jmp(state: &State, addr: u32) -> String {
    if state.blocks.contains_key(&addr) {
        format!("Cont(x{:x})", addr)
    } else {
        format!("/* TODO */ indirect(ctx, {:#x}u32)", addr)
    }
}

fn gen_jmp(state: &State, instr: &iced_x86::Instruction) -> String {
    match instr.op_kind(0) {
        iced_x86::OpKind::NearBranch32 => {
            let addr = instr.near_branch32();
            gen_abs_jmp(state, addr)
        }
        iced_x86::OpKind::Memory => {
            // If it's like `call [someaddr]` where someaddr is in the IAT, resolve it directly.
            if let Some(addr) = is_abs_memory_ref(instr) {
                if let Some(func) = state.iat_refs.get(&addr) {
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

pub fn codegen(w: &mut Writer, state: &State, instr: &iced_x86::Instruction) -> bool {
    use iced_x86::Mnemonic::*;
    match instr.mnemonic() {
        Jmp => w.line(gen_jmp(state, instr)),
        Call => {
            // Create a temporary here in case gen_jmp needs to borrow ctx.
            w.line(format!("let dst = {};", gen_jmp(state, instr)));
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
            let next = gen_abs_jmp(state, instr.next_ip32());
            let dst = gen_jmp(state, instr);
            let func = instr_name(instr);
            w.line(format!("{func}(ctx, {next}, {dst})"));
        }

        Loop => {
            let next = gen_abs_jmp(state, instr.next_ip32());
            let dst = gen_jmp(state, instr);
            w.line("ctx.cpu.regs.ecx = ctx.cpu.regs.ecx.wrapping_sub(1);");
            w.line(format!("if ctx.cpu.regs.ecx == 0 {{ {next} }}"));
            w.line(format!("else {{ {dst} }}"));
        }

        _ => return false,
    }
    true
}
