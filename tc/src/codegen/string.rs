use crate::codegen::{CodeGen, instr_name};

impl<'a> CodeGen<'a> {
    pub fn codegen_string(&mut self, instr: &iced_x86::Instruction) -> bool {
        use iced_x86::Mnemonic::*;
        match instr.mnemonic() {
            Movsb => {
                assert!(!instr.has_repne_prefix());
                if instr.has_rep_prefix() {
                    self.line("ctx.rep(Rep::REP, Context::movsb);");
                } else {
                    self.line("ctx.movsb();");
                }
            }
            Movsd => {
                assert!(!instr.has_repne_prefix());
                if instr.has_rep_prefix() {
                    self.line("ctx.rep(Rep::REP, Context::movsd);");
                } else {
                    self.line("ctx.movsd();");
                }
            }

            Lodsb => {
                assert!(!instr.has_repne_prefix());
                if instr.has_rep_prefix() {
                    self.line("ctx.rep(Rep::REP, Context::lodsb);");
                } else {
                    self.line("ctx.lodsb();");
                };
            }
            Lodsd => {
                assert!(!instr.has_repne_prefix());
                if instr.has_rep_prefix() {
                    self.line("ctx.rep(Rep::REP, Context::lodsd);");
                } else {
                    self.line("ctx.lodsd();");
                };
            }

            Stosb | Stosw | Stosd => {
                let op = instr_name(instr);
                assert!(!instr.has_repne_prefix());
                if instr.has_rep_prefix() {
                    self.line(format!("ctx.rep(Rep::REP, Context::{op});"));
                } else {
                    self.line(format!("ctx.{op}();"));
                };
            }

            // XXX: cmps/scas use repe, not rep
            Cmpsb => {
                if instr.has_repe_prefix() {
                    self.line("ctx.rep(Rep::REPE, Context::cmpsb);");
                } else if instr.has_repne_prefix() {
                    self.line("ctx.rep(Rep::REPNE, Context::cmpsb);");
                } else {
                    self.line("ctx.cmpsb();");
                };
            }

            // XXX: cmps/scas use repe, not rep
            Scasb => {
                if instr.has_repe_prefix() {
                    self.line("ctx.rep(Rep::REPE, Context::scasb);");
                } else if instr.has_repne_prefix() {
                    self.line("ctx.rep(Rep::REPNE, Context::scasb);");
                } else {
                    self.line("ctx.scasb();");
                };
            }

            _ => return false,
        }
        true
    }
}
