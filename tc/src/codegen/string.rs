use crate::codegen::{CodeGen, get_reg, instr_name};

impl<'a> CodeGen<'a> {
    pub fn codegen_string(&mut self, instr: &iced_x86::Instruction) -> bool {
        use iced_x86::Mnemonic::*;
        match instr.mnemonic() {
            Movsb | Movsw | Movsd => {
                let name = instr_name(instr);
                // Note: repe/repne behaves the same as rep for these instructions,
                if instr.has_rep_prefix() || instr.has_repne_prefix() {
                    let bitness = self.module.bitness();
                    self.line(format!("let seg = {};", get_reg(instr.memory_segment())));
                    self.line(format!(
                        "ctx.rep{bitness}(Rep::REP, |ctx: &mut Context| ctx.{name}(seg));"
                    ));
                } else {
                    self.line(format!(
                        "ctx.{name}({seg});",
                        seg = get_reg(instr.memory_segment())
                    ));
                }
            }

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
                if instr.has_rep_prefix() || instr.has_repne_prefix() {
                    let bitness = self.module.bitness();
                    self.line(format!("let seg = {};", get_reg(instr.memory_segment())));
                    self.line(format!(
                        "ctx.rep{bitness}(Rep::REP, |ctx: &mut Context| ctx.{name}(seg));"
                    ));
                } else {
                    self.line(format!(
                        "ctx.{name}({seg});",
                        seg = get_reg(instr.memory_segment()),
                    ));
                }
            }

            // Careful: cmps/scas use repe, not rep
            Cmpsb | Cmpsw | Cmpsd => {
                let name = instr_name(instr);
                if instr.has_repe_prefix() || instr.has_repne_prefix() {
                    let bitness = self.module.bitness();
                    let rep = if instr.has_repe_prefix() {
                        "REPE"
                    } else {
                        "REPNE"
                    };
                    self.line(format!("let seg = {};", get_reg(instr.memory_segment())));
                    self.line(format!(
                        "ctx.rep{bitness}(Rep::{rep}, |ctx: &mut Context| ctx.{name}(seg));"
                    ));
                } else {
                    self.line(format!(
                        "ctx.{name}({seg});",
                        seg = get_reg(instr.memory_segment()),
                    ));
                };
            }

            // Careful: cmps/scas use repe, not rep
            Scasb | Scasw | Scasd => {
                let name = instr_name(instr);
                if instr.has_repe_prefix() || instr.has_repne_prefix() {
                    let bitness = self.module.bitness();
                    let rep = if instr.has_repe_prefix() {
                        "REPE"
                    } else {
                        "REPNE"
                    };
                    self.line(format!(
                        "ctx.rep{bitness}(Rep::{rep}, |ctx: &mut Context| ctx.{name}());"
                    ));
                } else {
                    self.line(format!("ctx.{name}();",));
                };
            }

            _ => return false,
        }
        true
    }
}
