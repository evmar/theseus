use std::collections::{HashMap, HashSet};

use crate::call;

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

fn ssa_block(block: &mut Block, used_vars: &mut MaxVarSet) {
    // Gather inputs while we traverse, assigning them names immediately
    // so that they get assigned the lowest name.
    // But then substitute at the end after all the locals have been renamed.

    let mut params = MaxVarSet::default();
    let mut gather_params = |used_vars: &mut MaxVarSet, expr: &Expr| match expr {
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
                *eff = Effect::Def(new, body.clone())
            }
            // shouldn't hit any defs, we are introducing them now
            Effect::Def(_, _) => unreachable!(),
            _ => {
                visit_effect(eff, &mut |expr| gather_params(used_vars, expr));
            }
        }
    }

    for param in params.iter() {
        block.instrs.insert(
            0,
            Instr {
                src: 0,
                eff: Effect::Def(param.clone(), call!("phi",)),
            },
        );
        rename_instrs(&mut block.instrs, &Var::new(param.reg.clone()), param);
    }

    block.params = params;
}

fn link_blocks(blocks: &mut Blocks) {
    let addr_to_id: HashMap<u32, usize> = blocks
        .vec
        .iter()
        .enumerate()
        .map(|(i, block)| (block.addr, i))
        .collect();

    for block in blocks.vec.iter_mut() {
        let last = block.instrs.last().unwrap();
        let Effect::Jmp(jmp) = &last.eff else {
            log::warn!("block {:x} does not end with jmp", block.addr);
            continue;
        };

        let mut links = vec![];
        for addr in jmp.dsts.iter() {
            let Expr::Const(addr) = addr else {
                continue;
            };
            let Some(&next_id) = addr_to_id.get(&addr) else {
                continue;
            };
            links.push(Link { id: next_id });
        }
        block.links = links;
    }
}

fn link_vars(blocks: &mut Blocks, used_vars: &mut MaxVarSet) {
    // For each block, input vars to block
    let mut bins: Vec<HashMap<Var, HashSet<Var>>> = Default::default();
    // For each block, output vars from block
    let mut bouts: Vec<MaxVarSet> = Default::default();

    for block in blocks.vec.iter() {
        bins.push(
            block
                .params
                .iter()
                .map(|param| (param.clone(), HashSet::new()))
                .collect(),
        );
        let outs = out_vars(block);
        bouts.push(outs);
    }

    let mut changed = true;
    while changed {
        changed = false;
        for src in blocks.vec.iter_mut() {
            let mut new_vars = vec![];
            for link in src.links.iter() {
                let [src_ins, dst_ins] = bins.get_disjoint_mut([src.id, link.id]).unwrap();
                let src_outs = &mut bouts[src.id];
                for (param, values) in dst_ins.iter_mut() {
                    let out = match src_outs.get(&param.reg) {
                        Some(var) => var.clone(),
                        None => {
                            // Handle var passthrough:
                            // If src -> dst and dst has some param X not in src's outputs,
                            // add X to src's inputs and outputs.

                            let new = used_vars.new_var(param);
                            new_vars.push(new.clone());
                            src_ins.insert(new.clone(), HashSet::new());
                            src_outs.insert(new.clone());
                            changed = true;
                            new
                        }
                    };

                    if values.insert(out) {
                        changed = true;
                    }
                }
            }
            for new_var in new_vars {
                src.instrs.insert(
                    0,
                    Instr {
                        src: 0,
                        eff: Effect::Def(new_var, call!("phi",)),
                    },
                );
            }
        }
    }

    for (id, block) in blocks.vec.iter_mut().enumerate() {
        for instr in block.instrs.iter_mut() {
            match &mut instr.eff {
                Effect::Def(var, Expr::Call(call)) if call.op == "phi" => {
                    let exprs = bins[id].get(var).unwrap();
                    call.args = exprs
                        .iter()
                        .map(|val| Expr::Var(val.clone()))
                        .collect::<Vec<_>>();
                }
                _ => break, // phis at top
            }
        }

        // block.params = bins[id]
        //     .drain()
        //     .map(|(param, mut values)| (param, values.drain().map(|val| Expr::Var(val)).collect()))
        //     .collect();

        //     let outs = &bouts[id];
        //     for link in block.links.iter_mut() {
        //         link.params = bins[link.id]
        //             .iter()
        //             .map(|p| (p.clone(), Expr::Var(outs.get(&p.reg).unwrap().clone())))
        //             .collect();
        //     }
    }
}

/// Find the variables that are live at the end of the block, which will be potential parameters to the next blocks.
fn out_vars(block: &Block) -> MaxVarSet {
    let mut outs = MaxVarSet::default();
    visit_block(block, &mut |expr| {
        match expr {
            Expr::Var(var) => {
                outs.insert(var.clone());
            }
            _ => {}
        };
    });
    outs
}

pub fn ssa(blocks: &mut Blocks) {
    let mut used_vars = MaxVarSet::default();
    for block in blocks.vec.iter_mut() {
        ssa_block(block, &mut used_vars);
    }
    link_blocks(blocks);
    link_vars(blocks, &mut used_vars);
}
