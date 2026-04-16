use super::ast::*;
use std::collections::{HashMap, HashSet};

/// If a variable is only used once, inline it into where it is used.
fn inline_block(block: &mut Block) {
    // Gather the inlinable new vars introduced by this block
    let mut new_vars: HashSet<Var> = HashSet::default();
    for instr in block.instrs.iter() {
        match &instr.eff {
            Effect::Set(Expr::Var(dst), _) => {
                new_vars.insert(dst.clone());
            }
            _ => {}
        }
    }

    // Count the times they are used
    let mut used: HashMap<Var, usize> = Default::default();
    let mut mark_read = |var: &Var| {
        if !new_vars.contains(var) {
            return;
        }
        *used.entry(var.clone()).or_default() += 1;
    };
    let visit = &mut |expr: &Expr| match expr {
        Expr::Var(var) => mark_read(var),
        _ => {}
    };
    for instr in block.instrs.iter() {
        match &instr.eff {
            Effect::Set(_, src) => visit_expr(src, visit),
            eff => visit_effect(eff, visit),
        }
    }

    for var in used
        .iter()
        .filter(|&(_, &count)| count == 1)
        .map(|(var, _)| var)
    {
        let set = block
            .instrs
            .iter()
            .position(|instr| {
                if let Effect::Set(Expr::Var(dst), _) = &instr.eff {
                    dst == var
                } else {
                    false
                }
            })
            .unwrap();
        let Effect::Set(_, val) = block.instrs.remove(set).eff else {
            unreachable!()
        };

        let mut do_inline = |expr: &mut Expr| {
            if let Expr::Var(dst) = expr {
                if dst == var {
                    *expr = val.clone();
                }
            }
        };
        for instr in block.instrs.iter_mut() {
            visit_effect_mut(&mut instr.eff, &mut do_inline);
        }
    }
}

pub fn inline(blocks: &mut Blocks) {
    for block in blocks.vec.iter_mut() {
        inline_block(block);
    }
}
