use crate::{
    State,
    codegen::{Writer, gen_abs_jmp, gen_jmp},
};

pub fn codegen(w: &mut Writer, state: &State, instr: &iced_x86::Instruction) -> bool {
    use iced_x86::Mnemonic::*;
    match instr.mnemonic() {
        Stosb => {
            assert!(!instr.has_repne_prefix());
            if instr.has_rep_prefix() {
                w.line("rep(Rep::REP, stosb);");
            } else {
                w.line("stosb();");
            };
        }
        Stosd => {
            assert!(!instr.has_repne_prefix());
            if instr.has_rep_prefix() {
                w.line("rep(Rep::REP, stosd);");
            } else {
                w.line("stosd();");
            };
        }
        Cmpsb => {
            assert!(!instr.has_repne_prefix());
            if instr.has_rep_prefix() {
                w.line("rep(Rep::REP, cmpsb);");
            } else {
                w.line("cmpsb();");
            };
        }
        Scasb => {
            assert!(!instr.has_repne_prefix());
            if instr.has_rep_prefix() {
                w.line("rep(Rep::REP, scasb);");
            } else {
                w.line("scasb();");
            };
        }
        Lodsb => {
            assert!(!instr.has_repne_prefix());
            if instr.has_rep_prefix() {
                w.line("rep(Rep::REP, lodsb);");
            } else {
                w.line("lodsb();");
            };
        }
        Lodsd => {
            assert!(!instr.has_repne_prefix());
            if instr.has_rep_prefix() {
                w.line("rep(Rep::REP, lodsd);");
            } else {
                w.line("lodsd();");
            };
        }
        Loop => {
            let next = gen_abs_jmp(state, instr.next_ip32());
            let dst = gen_jmp(state, instr);
            w.line("m.regs.ecx = m.regs.ecx.wrapping_sub(1);");
            w.line(format!("if m.regs.ecx == 0 {{ {next} }}"));
            w.line(format!("else {{ {dst} }}"));
        }
        _ => return false,
    }
    true
}
