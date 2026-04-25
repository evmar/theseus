use crate::codegen::CodeGen;

impl<'a> CodeGen<'a> {
    pub fn codegen_string(&mut self, instr: &iced_x86::Instruction) -> bool {
        use iced_x86::Mnemonic::*;
        match instr.mnemonic() {
            Movsb => {
                assert!(!instr.has_repne_prefix());
                if instr.has_rep_prefix() {
                    self.line("rep(ctx, Rep::REP, movsb);");
                } else {
                    self.line("movsb(ctx);");
                }
            }
            Movsd => {
                assert!(!instr.has_repne_prefix());
                if instr.has_rep_prefix() {
                    self.line("rep(ctx, Rep::REP, movsd);");
                } else {
                    self.line("movsd(ctx);");
                }
            }

            Lodsb => {
                assert!(!instr.has_repne_prefix());
                if instr.has_rep_prefix() {
                    self.line("rep(ctx, Rep::REP, lodsb);");
                } else {
                    self.line("lodsb(ctx);");
                };
            }
            Lodsd => {
                assert!(!instr.has_repne_prefix());
                if instr.has_rep_prefix() {
                    self.line("rep(ctx, Rep::REP, lodsd);");
                } else {
                    self.line("lodsd(ctx);");
                };
            }

            Stosb => {
                assert!(!instr.has_repne_prefix());
                if instr.has_rep_prefix() {
                    self.line("rep(ctx, Rep::REP, stosb);");
                } else {
                    self.line("stosb(ctx);");
                };
            }
            Stosw => self.todo(),
            Stosd => {
                assert!(!instr.has_repne_prefix());
                if instr.has_rep_prefix() {
                    self.line("rep(ctx, Rep::REP, stosd);");
                } else {
                    self.line("stosd(ctx);");
                };
            }

            // XXX: cmps/scas use repe, not rep
            Cmpsb => {
                if instr.has_repe_prefix() {
                    self.line("rep(ctx, Rep::REPE, cmpsb);");
                } else if instr.has_repne_prefix() {
                    self.line("rep(ctx, Rep::REPNE, cmpsb);");
                } else {
                    self.line("cmpsb(ctx);");
                };
            }

            // XXX: cmps/scas use repe, not rep
            Scasb => {
                if instr.has_repe_prefix() {
                    self.line("rep(ctx, Rep::REPE, scasb);");
                } else if instr.has_repne_prefix() {
                    self.line("rep(ctx, Rep::REPNE, scasb);");
                } else {
                    self.line("scasb(ctx);");
                };
            }

            _ => return false,
        }
        true
    }
}
