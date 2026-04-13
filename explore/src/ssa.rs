use std::collections::HashMap;

use super::ast::*;

fn rename_instrs(instrs: &mut [Instr], from: &Var, to: &Var) {
    for instr in instrs {
        visit_effect_mut(&mut instr.eff, &mut |expr| {
            if let Expr::Var(v) = expr {
                if v == from {
                    *v = to.clone();
                }
            }
        });
    }
}

fn ssa_block(block: &mut Block, used_vars: &mut VarSet) {
    // Gather inputs while we traverse, assigning them names immediate, so that they get assigned the lowest name.
    // But then substitute at the end after all the locals have been renamed.

    let mut params = VarSet::default();
    let mut gather_params = |used_vars: &mut VarSet, expr: &Expr| match expr {
        Expr::Var(var) => {
            if var.ver == 0 && params.get(&var.reg).is_none() {
                params.insert(used_vars.new_var(var));
            }
        }
        _ => {}
    };

    for i in 0..block.instrs.len() {
        let (instr, rest) = block.instrs[i..].split_first_mut().unwrap();
        let eff = &mut instr.eff;
        match eff {
            Effect::Set(Expr::Var(var), body) => {
                visit_expr(body, &mut |expr| gather_params(used_vars, expr));
                let new = used_vars.new_var(var);
                rename_instrs(rest, &var, &new);
                *eff = Effect::Set(Expr::Var(new), body.clone())
            }
            _ => {
                visit_effect(eff, &mut |expr| gather_params(used_vars, expr));
            }
        }
    }

    for param in params.iter() {
        rename_instrs(&mut block.instrs, &Var::new(param.reg.clone()), param);
    }

    block.params = params;
}

fn link(blocks: &mut Blocks, used_vars: &mut VarSet) {
    // For each block, ids of following blocks
    let nexts = blocks
        .vec
        .iter()
        .map(|block| {
            let last = block.instrs.last().unwrap();
            let Effect::Jmp(jmp) = &last.eff else {
                log::warn!("block {:x} does not end with jmp", block.addr);
                return vec![];
            };
            let addrs = jmp
                .dsts
                .iter()
                .flat_map(|addr| {
                    let Expr::Const(addr) = addr else {
                        return None;
                    };
                    Some(*addr)
                })
                .collect::<Vec<_>>();
            addrs
                .into_iter()
                .flat_map(|addr| blocks.vec.iter().position(|b| b.addr == addr))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // For each block, input vars to block
    let mut bins: Vec<VarSet> = Default::default();
    // For each block, output vars from block
    let mut bouts: Vec<HashMap<String, Var>> = Default::default();

    for block in blocks.vec.iter() {
        bins.push(block.params.clone());
        let outs = out_vars(block);
        bouts.push(outs);
    }

    // Handle var passthrough:
    // If A -> B and B has some input X not in A's outputs,
    // add X to A's inputs and outputs.
    let mut changed = true;
    while changed {
        changed = false;
        for src in blocks.vec.iter() {
            let outs = &bouts[src.id];
            let mut add: Vec<Var> = vec![];
            for &next_id in &nexts[src.id] {
                for param in bins[next_id].iter() {
                    if !outs.contains_key(&param.reg) {
                        add.push(used_vars.new_var(param));
                    }
                }
            }

            if !add.is_empty() {
                for add in add {
                    bins[src.id].insert(add.clone());
                    bouts[src.id].insert(add.reg.clone(), add);
                }
                changed = true;
            }
        }
    }

    for id in 0..blocks.vec.len() {
        let outs = &bouts[id];

        let next = nexts[id]
            .iter()
            .map(|&next_id| {
                let params = bins[next_id]
                    .iter()
                    .map(|p| (p.clone(), Expr::Var(outs.get(&p.reg).unwrap().clone())))
                    .collect();
                Link {
                    addr: blocks.vec[next_id].addr,
                    params,
                }
            })
            .collect();

        blocks.vec[id].params = bins[id].clone();
        blocks.vec[id].links = next;
    }
}

/// Find the variables that are live at the end of the block, which will be potential parameters to the next blocks.
fn out_vars(block: &Block) -> HashMap<String, Var> {
    let mut outs: HashMap<String, Var> = HashMap::new();
    visit_block(block, &mut |expr| {
        match expr {
            Expr::Var(var) => {
                if let Some(prev) = outs.get_mut(&var.reg) {
                    prev.ver = prev.ver.max(var.ver);
                } else {
                    outs.insert(var.reg.clone(), var.clone());
                }
            }
            _ => {}
        };
    });
    outs
}

pub fn ssa(blocks: &mut Blocks) {
    let mut used_vars = VarSet::default();
    for block in blocks.vec.iter_mut() {
        ssa_block(block, &mut used_vars);
    }
    link(blocks, &mut used_vars);
}
