use super::ast::*;
use std::collections::{HashMap, HashSet};

fn simplify_phi_expr(expr: &mut Expr) -> bool {
    let Expr::Call(call) = expr else {
        return false;
    };
    if call.op != "phi" {
        return false;
    }

    let mut vals = HashSet::new();
    for arg in call.args.iter() {
        if let Expr::Var(var) = arg {
            vals.insert(var.clone());
        } else {
            return false;
        }
    }

    if vals.len() == 1 {
        let new = vals.into_iter().next().unwrap();
        *expr = Expr::Var(new);
        return true;
    } else if vals.len() != call.args.len() {
        *expr = Expr::Call(Box::new(Call {
            op: "phi".to_string(),
            args: vals.into_iter().map(|var| Expr::Var(var)).collect(),
        }));
        return true;
    }
    false
}

fn simplify_phi(block: &mut Block) -> bool {
    let mut changed = false;
    for instr in block.instrs.iter_mut() {
        visit_effect_mut(&mut instr.eff, &mut |expr| {
            if simplify_phi_expr(expr) {
                changed = true;
            }
        });
    }
    changed
}

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

fn inline_var(block: &mut Block, var: &Var) -> bool {
    let Some(val) = remove_def(block, var) else {
        return false;
    };
    log::info!("inlining {} = {}", var, val);

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
    true
}

pub fn inline_block(block: &mut Block) {
    let mut changed = true;
    while changed {
        changed = false;
        if simplify_phi(block) {
            changed = true;
        }
        log::info!("phi simp {:?}", changed);

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
