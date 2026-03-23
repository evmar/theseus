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
                w.line("rep(m, Rep::REP, movsb);");
            } else {
                w.line("movsb(m);");
            }
        }
        Movsd => {
            assert!(!instr.has_repne_prefix());
            if instr.has_rep_prefix() {
                w.line("rep(m, Rep::REP, movsd);");
            } else {
                w.line("movsd(m);");
            }
        }

        Lodsb => {
            assert!(!instr.has_repne_prefix());
            if instr.has_rep_prefix() {
                w.line("rep(m, Rep::REP, lodsb);");
            } else {
                w.line("lodsb(m);");
            };
        }
        Lodsd => {
            assert!(!instr.has_repne_prefix());
            if instr.has_rep_prefix() {
                w.line("rep(m, Rep::REP, lodsd);");
            } else {
                w.line("lodsd(m);");
            };
        }

        Stosb => {
            assert!(!instr.has_repne_prefix());
            if instr.has_rep_prefix() {
                w.line("rep(m, Rep::REP, stosb);");
            } else {
                w.line("stosb(m);");
            };
        }
        Stosd => {
            assert!(!instr.has_repne_prefix());
            if instr.has_rep_prefix() {
                w.line("rep(m, Rep::REP, stosd);");
            } else {
                w.line("stosd(m);");
            };
        }

        // XXX: cmps/scas use repe, not rep
        Cmpsb => {
            if instr.has_repe_prefix() {
                w.line("rep(m, Rep::REPE, cmpsb);");
            } else if instr.has_repne_prefix() {
                w.line("rep(m, Rep::REPNE, cmpsb);");
            } else {
                w.line("cmpsb(m);");
            };
        }

        // XXX: cmps/scas use repe, not rep
        Scasb => {
            if instr.has_repe_prefix() {
                w.line("rep(m, Rep::REPE, scasb);");
            } else if instr.has_repne_prefix() {
                w.line("rep(m, Rep::REPNE, scasb);");
            } else {
                w.line("scasb(m);");
            };
        }

        Loop => {
            let next = gen_abs_jmp(state, instr.next_ip32());
            let dst = gen_jmp(state, instr);
            w.line("m.cpu.regs.ecx = m.cpu.regs.ecx.wrapping_sub(1);");
            w.line(format!("if m.cpu.regs.ecx == 0 {{ {next} }}"));
            w.line(format!("else {{ {dst} }}"));
        }
        _ => return false,
    }
    true
}
