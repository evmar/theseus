use super::ast::*;
use std::collections::{HashMap, HashSet};

fn simplify_phi_expr(expr: &mut Expr) -> bool {
    let Expr::Call(call) = expr else {
        return false;
    };
    if call.op != "phi" {
        return false;
    }

    let vals = call
        .args
        .iter()
        .filter_map(|arg| {
            if let Expr::Var(var) = arg {
                Some(var.clone())
            } else {
                None
            }
        })
        .collect::<HashSet<_>>();

    if vals.len() == 1 {
        *expr = Expr::Var(vals.into_iter().next().unwrap());
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

fn simplify_phi(blocks: &mut Blocks) -> bool {
    let mut changed = false;
    for block in blocks.vec.iter_mut() {
        for instr in block.instrs.iter_mut() {
            visit_effect_mut(&mut instr.eff, &mut |expr| {
                if simplify_phi_expr(expr) {
                    changed = true;
                }
            });
        }
    }
    changed
}

fn count_uses(blocks: &Blocks) -> HashMap<Var, usize> {
    let mut used: HashMap<Var, usize> = Default::default();
    let mut mark_read = |var: &Var| {
        *used.entry(var.clone()).or_default() += 1;
    };
    let visit = &mut |expr: &Expr| {
        log::info!("visit {}", expr);
        match expr {
            Expr::Var(var) => mark_read(var),
            _ => {}
        }
    };
    for block in blocks.vec.iter() {
        log::info!("block {:x}", block.addr);
        for instr in block.instrs.iter() {
            visit_effect(&instr.eff, visit);
        }
        log::info!("block {:x}", block.addr,);
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

fn remove_def(blocks: &mut Blocks, var: &Var) -> Option<Expr> {
    for block in blocks.vec.iter_mut() {
        let Some(set) = block
            .instrs
            .extract_if(.., |instr| {
                if let Effect::Def(dst, _) = &instr.eff {
                    dst == var
                } else {
                    false
                }
            })
            .next()
        else {
            continue;
        };
        let Effect::Def(_, val) = set.eff else {
            unreachable!();
        };
        return Some(val);
    }
    None
}

fn inline_var(blocks: &mut Blocks, var: &Var) -> bool {
    let Some(val) = remove_def(blocks, var) else {
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
    for block in blocks.vec.iter_mut() {
        for instr in block.instrs.iter_mut() {
            visit_effect_mut(&mut instr.eff, &mut do_inline);
        }
    }
    true
}

pub fn inline(blocks: &mut Blocks) {
    let mut changed = true;
    while changed {
        changed = false;
        // if simplify_phi(blocks) {
        //     changed = true;
        // }
        // log::info!("phi simp {:?}", changed);

        let used = count_uses(blocks);
        break;
        let used_once = used
            .iter()
            .filter(|&(_, &count)| count == 1)
            .map(|(var, _)| var)
            .collect::<Vec<_>>();
        for var in used_once {
            if inline_var(blocks, var) {
                changed = true;
            }
        }
    }
}
