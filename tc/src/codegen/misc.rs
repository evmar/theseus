use crate::{
    State,
    codegen::{Writer, gen_addr, get_op, instr_name, op_size, set_op},
};

pub fn codegen(w: &mut Writer, _state: &State, instr: &iced_x86::Instruction) -> bool {
    use iced_x86::Mnemonic::*;
    match instr.mnemonic() {
        Push => {
            let func = match op_size(instr, 0) {
                16 => "push16",
                32 => "push",
                _ => return false,
            };
            w.line(format!("{func}(ctx, {});", get_op(instr, 0)));
        }
        Pop => {
            let func = match op_size(instr, 0) {
                16 => "pop16",
                32 => "pop",
                _ => return false,
            };
            w.line(format!("let x = {func}(ctx);"));
            w.line(set_op(instr, 0, "x".into()))
        }
        Pushad => w.line("pushad(ctx);"),
        Popad => w.line("popad(ctx);"),
        Mov => w.line(set_op(instr, 0, get_op(instr, 1))),

        Sete => w.line(set_op(instr, 0, "sete(ctx)".into())),
        Cmp => {
            let op0 = get_op(instr, 0);
            let op1 = get_op(instr, 1);
            w.line(format!("sub({op0}, {op1}, &mut ctx.cpu.flags);"));
        }
        Test => {
            w.line(format!(
                "and({}, {}, &mut ctx.cpu.flags);",
                get_op(instr, 0),
                get_op(instr, 1)
            ));
        }

        Lea => w.line(format!("{} = {};", get_op(instr, 0), gen_addr(instr))),

        Movzx => {
            w.line(set_op(instr, 0, format!("{} as _", get_op(instr, 1))));
        }
        Movsx => {
            let read = format!(
                "{read} as i{src} as i{dst} as u{dst}",
                read = get_op(instr, 1),
                src = op_size(instr, 1),
                dst = op_size(instr, 0)
            );
            w.line(set_op(instr, 0, read));
        }

        Leave => w.line("leave(ctx);"),
        Enter => {
            assert!(instr.op1_kind() == iced_x86::OpKind::Immediate8_2nd);
            let op1 = instr.immediate8_2nd();
            w.line(format!("enter(ctx, {}, {:x});", get_op(instr, 0), op1));
        }

        Xchg => {
            w.line(format!("let t = {};", get_op(instr, 0)));
            w.line(set_op(instr, 0, get_op(instr, 1)));
            w.line(set_op(instr, 1, "t".into()));
        }
        Nop => {}

        Not | Int | Int3 | Cmpxchg | Pushfd | Setne | Cpuid | Xgetbv | Setg | Bt | Div | Setge => {
            w.todo()
        }

        Cbw => w.line("ctx.cpu.regs.set_ax(ctx.cpu.regs.get_al() as i8 as i16 as u16);"),
        Cwde => w.line("ctx.cpu.regs.eax = ctx.cpu.regs.get_ax() as i16 as i32 as u32;"),
        Cdq => {
            w.line("let t = ctx.cpu.regs.eax as i32 as i64 as u64;");
            w.line("ctx.cpu.regs.edx = (t >> 32) as u32;");
            w.line("ctx.cpu.regs.eax = t as u32;");
        }

        Stc | Clc | Std | Cld | Sahf => {
            w.line(format!("{}(ctx);", instr_name(instr)));
        }

        _ => return false,
    }
    true
}
