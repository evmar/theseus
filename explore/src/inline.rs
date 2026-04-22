use std::collections::HashMap;

use super::ast::*;

fn count_uses(block: &Block) -> HashMap<Var, usize> {
    let mut used: HashMap<Var, usize> = Default::default();
    let mut mark_read = |var: &Var| {
        *used.entry(var.clone()).or_default() += 1;
    };
    let visit = &mut |expr: &Expr| match expr {
        Expr::Var(var) => mark_read(var),
        _ => {}
    };
    for instr in block.instrs.iter() {
        visit_effect(&instr.eff, visit);
    }
    log::info!(
        "used {}",
        used.iter()
            .map(|(var, val)| format!("{}={}", var, val))
            .collect::<Vec<_>>()
            .join(" ")
    );

    used
}

fn remove_def(block: &mut Block, var: &Var) -> Option<Expr> {
    let set = block
        .instrs
        .extract_if(.., |instr| {
            if let Effect::Def(dst, _) = &instr.eff {
                dst == var
            } else {
                false
            }
        })
        .next()?;

    let Effect::Def(_, val) = set.eff else {
        unreachable!();
    };
    return Some(val);
}

fn replace_var(block: &mut Block, var: &Var, val: Expr) {
    for instr in block.instrs.iter_mut() {
        visit_effect_mut(&mut instr.eff, &mut |expr: &mut Expr| {
            if let Expr::Var(dst) = expr {
                if dst == var {
                    *expr = val.clone();
                }
            }
        });
    }
}

fn inline_var(block: &mut Block, var: &Var) -> bool {
    let Some(val) = remove_def(block, var) else {
        return false;
    };
    log::info!("inlining {} = {}", var, val);
    replace_var(block, var, val);
    true
}

pub fn inline_block(block: &mut Block) {
    let mut changed = true;
    while changed {
        changed = false;

        let used = count_uses(block);
        let used_once = used
            .iter()
            .filter(|&(_, &count)| count == 1)
            .map(|(var, _)| var)
            .collect::<Vec<_>>();
        for var in used_once {
            if inline_var(block, var) {
                changed = true;
            }
        }
    }
}

pub fn inline(blocks: &mut Blocks) {
    for block in blocks.vec.iter_mut() {
        inline_block(block);
    }
}
