use crate::codegen::{CodeGen, get_reg, instr_name};

impl<'a> CodeGen<'a> {
    pub fn codegen_string(&mut self, instr: &iced_x86::Instruction) -> bool {
        use iced_x86::Mnemonic::*;
        match instr.mnemonic() {
            Movsb | Movsw | Movsd | // x
            Stosb | Stosw | Stosd => {
                let name = instr_name(instr);
                // Note: repe/repne behaves the same as rep for these instructions,
                if instr.has_rep_prefix() || instr.has_repne_prefix() {
                    let bitness = self.module.bitness();
                    self.line(format!("ctx.rep{bitness}(Rep::REP, Context::{name});"));
                } else {
                    self.line(format!("ctx.{name}();"));
                }
            }

            Lodsb | Lodsw | Lodsd => {
                let name = instr_name(instr);
                let call = format!(
                    "ctx.{name}({})",
                    get_reg(instr.memory_segment())
                );
                if instr.has_rep_prefix() || instr.has_repne_prefix() {
                    let bitness = self.module.bitness();
                    self.line(format!("ctx.rep{bitness}(Rep::REP, |ctx: &mut Context| {call});"));
                } else {
                    self.line(format!("{call};"));
                }
            }

            // Careful: cmps/scas use repe, not rep
            Cmpsb | Cmpsw | Cmpsd | //x
            Scasb | Scasw | Scasd => {
                // TODO: segment override
                let name = instr_name(instr);
                let bitness = self.module.bitness();
                if instr.has_repe_prefix() {
                    self.line(format!("ctx.rep{bitness}(Rep::REPE, Context::{name});"));
                } else if instr.has_repne_prefix() {
                    self.line(format!("ctx.rep{bitness}(Rep::REPNE, Context::{name});"));
                } else {
                    self.line(format!("ctx.{name}();"));
                };
            }

            _ => return false,
        }
        true
    }
}
