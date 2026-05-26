use crate::codegen::{CodeGen, instr_name};

impl<'a> CodeGen<'a> {
    pub fn codegen_string(&mut self, instr: &iced_x86::Instruction) -> bool {
        use iced_x86::Mnemonic::*;
        match instr.mnemonic() {
            Movsb | Movsw | Movsd | // x
            Lodsb | Lodsw | Lodsd | // x
            Stosb | Stosw | Stosd => {
                assert!(!instr.has_repne_prefix());
                let name = instr_name(instr);
                if instr.has_rep_prefix() {
                    self.line(format!("ctx.rep(Rep::REP, Context::{name});"));
                } else {
                    self.line(format!("ctx.{name}();"));
                }
            }

            // Careful: cmps/scas use repe, not rep
            Cmpsb | Cmpsw | Cmpsd | //x
            Scasb | Scasw | Scasd => {
                let name = instr_name(instr);
                if instr.has_repe_prefix() {
                    self.line(format!("ctx.rep(Rep::REPE, Context::{name});"));
                } else if instr.has_repne_prefix() {
                    self.line(format!("ctx.rep(Rep::REPNE, Context::{name});"));
                } else {
                    self.line(format!("ctx.{name}();"));
                };
            }

            _ => return false,
        }
        true
    }
}
