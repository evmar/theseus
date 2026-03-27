use crate::{
    State,
    codegen::{Writer, gen_abs_jmp, gen_jmp},
};

pub fn codegen(w: &mut Writer, state: &State, instr: &iced_x86::Instruction) -> bool {
    use iced_x86::Mnemonic::*;
    match instr.mnemonic() {
        Movsb => {
            assert!(!instr.has_repne_prefix());
            if instr.has_rep_prefix() {
                w.line("rep(ctx, Rep::REP, movsb);");
            } else {
                w.line("movsb(ctx);");
            }
        }
        Movsd => {
            assert!(!instr.has_repne_prefix());
            if instr.has_rep_prefix() {
                w.line("rep(ctx, Rep::REP, movsd);");
            } else {
                w.line("movsd(ctx);");
            }
        }

        Lodsb => {
            assert!(!instr.has_repne_prefix());
            if instr.has_rep_prefix() {
                w.line("rep(ctx, Rep::REP, lodsb);");
            } else {
                w.line("lodsb(ctx);");
            };
        }
        Lodsd => {
            assert!(!instr.has_repne_prefix());
            if instr.has_rep_prefix() {
                w.line("rep(ctx, Rep::REP, lodsd);");
            } else {
                w.line("lodsd(ctx);");
            };
        }

        Stosb => {
            assert!(!instr.has_repne_prefix());
            if instr.has_rep_prefix() {
                w.line("rep(ctx, Rep::REP, stosb);");
            } else {
                w.line("stosb(ctx);");
            };
        }
        Stosw => w.todo(),
        Stosd => {
            assert!(!instr.has_repne_prefix());
            if instr.has_rep_prefix() {
                w.line("rep(ctx, Rep::REP, stosd);");
            } else {
                w.line("stosd(ctx);");
            };
        }

        // XXX: cmps/scas use repe, not rep
        Cmpsb => {
            if instr.has_repe_prefix() {
                w.line("rep(ctx, Rep::REPE, cmpsb);");
            } else if instr.has_repne_prefix() {
                w.line("rep(ctx, Rep::REPNE, cmpsb);");
            } else {
                w.line("cmpsb(ctx);");
            };
        }

        // XXX: cmps/scas use repe, not rep
        Scasb => {
            if instr.has_repe_prefix() {
                w.line("rep(ctx, Rep::REPE, scasb);");
            } else if instr.has_repne_prefix() {
                w.line("rep(ctx, Rep::REPNE, scasb);");
            } else {
                w.line("scasb(ctx);");
            };
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
