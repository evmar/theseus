#![allow(dead_code)]

mod ast;
use ast::*;

use std::collections::{HashMap, HashSet};

fn expr_from_iced(instr: iced_x86::Instruction, i: u32) -> Expr {
    use iced_x86::OpKind::*;
    match instr.op_kind(i) {
        Register => Expr::from_reg(instr.op_register(i)),
        Memory => Expr::Call(Box::new(call_from_memory(instr))),
        NearBranch32 => Expr::Const(instr.near_branch32()),
        Immediate8 => Expr::Const(instr.immediate8() as u32),
        Immediate32 => Expr::Const(instr.immediate32()),
        Immediate8to32 => Expr::Const(instr.immediate8to32() as u32),
        k => todo!("{k:?}"),
    }
}

macro_rules! call {
    ($x:expr, $($arg:expr),*) => { crate::Expr::Call(Box::new(crate::Call { op: $x.into(), args: vec![$($arg.into()),*] })) };
}

// macro_rules! expr {
//     ($x:expr) => { Expr::Const($x) };
//     ($x:expr, $($arg:expr) +) => { Expr::Call(Box::new(Call { op: $x, args: vec![$($arg),*] })) };
// }

fn call_from_memory(instr: iced_x86::Instruction) -> Call {
    let mut args = Vec::new();
    match instr.memory_segment() {
        iced_x86::Register::CS | iced_x86::Register::DS | iced_x86::Register::SS => {}
        r @ iced_x86::Register::FS => args.push(Expr::from_reg(r)),
        iced_x86::Register::None => {}
        r => todo!("{r:?}"),
    }

    match instr.memory_base() {
        iced_x86::Register::None => {}
        r => args.push(Expr::from_reg(r)),
    }

    if instr.memory_index() != iced_x86::Register::None {
        let mut expr = Expr::from_reg(instr.memory_index());
        if instr.memory_index_scale() != 1 {
            expr = Expr::Call(Box::new(Call {
                op: "*".into(),
                args: vec![expr, Expr::Const(instr.memory_index_scale())],
            }));
        }
        args.push(expr);
    }

    let offset = instr.memory_displacement32();
    if offset != 0 {
        args.push(Expr::Const(offset));
    }

    Call {
        op: "mem".into(),
        args,
    }
}

/// Decode the x86 instructions into our Instr type.
fn decode() -> Vec<Instr> {
    let mut instrs = Vec::new();
    let decoder = iced_x86::Decoder::with_ip(32, ASM, IP as u64, iced_x86::DecoderOptions::NONE);
    for instr in decoder {
        let mut args = Vec::new();
        for i in 0..instr.op_count() {
            args.push(expr_from_iced(instr, i));
        }

        use iced_x86::Mnemonic::*;
        let op = format!("{:?}", instr.mnemonic()).to_ascii_lowercase();
        let eff: Effect = match instr.mnemonic() {
            Inc => {
                let [x] = args.try_into().unwrap();
                Effect::Set(x.clone(), call!("+", x, Expr::Const(1)))
            }
            Mov => {
                let [x, y] = args.try_into().unwrap();
                Effect::Set(x.clone(), y.clone())
            }
            Pop => {
                let [dst] = args.try_into().unwrap();
                instrs.push(Instr {
                    iced: instr,
                    eff: Effect::Set(dst, call!("pop",)),
                });
                // TODO: stack handling
                // let esp = Expr::Var(Var::new("esp".into()));
                // instrs.push(Instr {
                //     iced: instr,
                //     eff: Effect::Set(esp.clone(), call!("-", esp, Expr::Const(4))),
                // });
                continue;
            }
            Add | Sub | Shl | Xor => {
                let op = match instr.mnemonic() {
                    Add => "+",
                    Sub => "-",
                    Shl => "<<",
                    Xor => "^",
                    _ => unreachable!(),
                };
                let [x, y] = args.try_into().unwrap();
                let y = if op == "^" && x == y {
                    0.into()
                } else {
                    call!(op, x.clone(), y)
                };
                Effect::Set(x, y)
            }
            Imul => {
                let [x, y, z] = args.try_into().unwrap();
                Effect::Set(x, call!("imul", y, z))
            }
            Call => {
                let [dst] = args.try_into().unwrap();
                instrs.push(Instr {
                    iced: instr,
                    eff: Effect::Call(Box::new(crate::Call {
                        op: "call".into(),
                        args: vec![dst],
                    })),
                });
                // assume stdcall
                instrs.push(Instr {
                    iced: instr,
                    eff: Effect::Set(
                        Expr::Var(Var::new("eax".into())),
                        Expr::Var(Var::new("?".into())),
                    ),
                });
                continue;
            }
            Jne | Jge => {
                let [x] = args.try_into().unwrap();
                Effect::Jmp(Box::new(crate::Jmp::new(
                    op,
                    vec![x, Expr::Const(instr.next_ip32())],
                )))
            }
            Jmp => {
                let [x] = args.try_into().unwrap();
                Effect::Jmp(Box::new(crate::Jmp::new(op, vec![x])))
            }
            Ret => Effect::Jmp(Box::new(crate::Jmp::new("ret", vec![]))),
            Cmp | Test => Effect::Call(Box::new(crate::Call { op, args })),
            _ => {
                let op = format!("todo:{op}");
                Effect::Call(Box::new(crate::Call { op, args }))
            }
        };

        instrs.push(Instr { iced: instr, eff });
    }
    instrs
}

/// Split the instructions into basic blocks, where each block ends with a jump.
fn blocks(instrs: Vec<Instr>) -> Blocks {
    let mut targets = HashSet::new();
    for instr in &instrs {
        let Effect::Jmp(jmp) = &instr.eff else {
            continue;
        };
        for dst in &jmp.dsts {
            let dst = match dst {
                Expr::Const(dst) => *dst,
                _ => {
                    log::warn!(
                        "{addr:x} unhandled jmp target {expr}",
                        addr = instr.iced.ip32(),
                        expr = dst
                    );
                    continue;
                }
            };
            targets.insert(dst);
        }
    }

    let mut blocks = vec![];
    let mut block = vec![];
    for instr in instrs {
        let is_jmp = matches!(instr.eff, Effect::Jmp(_));
        let last_in_block = is_jmp || targets.contains(&instr.iced.next_ip32());
        block.push(instr);
        if last_in_block {
            if !is_jmp {
                let iced = block.last().unwrap().iced;
                block.push(Instr {
                    iced,
                    eff: Effect::Jmp(Box::new(Jmp::new(
                        "jmp",
                        vec![Expr::Const(iced.next_ip32())],
                    ))),
                });
            }
            blocks.push(Block {
                id: blocks.len(),
                instrs: block,
                params: Default::default(),
                links: vec![],
            });
            block = vec![];
        }
    }
    if !block.is_empty() {
        log::error!("{:#?}", block);
        assert!(block.is_empty());
    }
    Blocks { vec: blocks }
}

fn visit_expr(expr: &mut Expr, f: &mut impl FnMut(&mut Expr)) {
    f(expr);
    if let Expr::Call(call) = expr {
        for arg in call.args.iter_mut() {
            visit_expr(arg, f);
        }
    }
}

fn visit_effect(effect: &mut Effect, f: &mut impl FnMut(&mut Expr)) {
    match effect {
        Effect::Set(x, y) => {
            visit_expr(x, f);
            visit_expr(y, f);
        }
        Effect::Call(call) => {
            for arg in call.args.iter_mut() {
                visit_expr(arg, f);
            }
        }
        Effect::Jmp(jmp) => {
            for arg in jmp.cond.args.iter_mut() {
                visit_expr(arg, f);
            }
            for dst in jmp.dsts.iter_mut() {
                visit_expr(dst, f);
            }
        }
    }
}

#[allow(unused)]
fn visit_block(block: &mut Block, f: &mut impl FnMut(&mut Expr)) {
    for instr in block.instrs.iter_mut() {
        visit_effect(&mut instr.eff, f);
    }
}

fn rename_effect(eff: &mut Effect, from: &Var, to: &Var) {
    visit_effect(eff, &mut |expr| match expr {
        Expr::Var(v) if v == from => {
            *v = to.clone();
        }
        _ => {}
    });
}

fn rename_instrs(instrs: &mut [Instr], from: &Var, to: &Var) {
    for instr in instrs {
        rename_effect(&mut instr.eff, from, to);
    }
}

fn rename_block(block: &mut Block, from: &Var, to: &Var) {
    for param in block.params.iter_mut() {
        if param == from {
            *param = to.clone();
        }
    }
    rename_instrs(&mut block.instrs, from, to);
    for link in block.links.iter_mut() {
        for (key, val) in link.params.iter_mut() {
            if key == from {
                *key = to.clone();
            }
            if val == from {
                *val = to.clone();
            }
        }
    }
}

fn ssa_block(block: &mut Block, used_vars: &mut VarSet) {
    // Gather inputs while we traverse, assigning them names immediate, so that they get assigned the lowest name.
    // But then substitute at the end after all the locals have been renamed.

    let mut params = VarSet::default();
    let mut gather_params = |used_vars: &mut VarSet, expr: &mut Expr| match expr {
        Expr::Var(var) => {
            if var.reg == "?" {
                return;
            }
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

fn ssa(blocks: &mut Blocks, used_vars: &mut VarSet) {
    for block in blocks.vec.iter_mut() {
        ssa_block(block, used_vars);
    }
}

fn simplify_branches(blocks: &mut Blocks) {
    for block in blocks.vec.iter_mut() {
        let mut i = 1;
        while i < block.instrs.len() {
            let [cur, prev] = block.instrs.get_disjoint_mut([i, i - 1]).unwrap();
            i += 1;
            let Effect::Jmp(jmp) = &mut cur.eff else {
                continue;
            };
            let Effect::Call(test) = &mut prev.eff else {
                continue;
            };

            let op = if jmp.cond.op == "jge" && test.op == "cmp" {
                ">="
            } else if jmp.cond.op == "jne" && test.op == "cmp" {
                "!="
            } else if jmp.cond.op == "jne" && test.op == "test" && test.args[0] == test.args[1] {
                "!=0"
            } else {
                continue;
            };

            assert!(jmp.cond.args.is_empty());
            jmp.cond.op = op.into();
            std::mem::swap(&mut jmp.cond.args, &mut test.args);
            cur.iced = prev.iced;
            block.instrs.remove(i - 2);
        }
    }
}

fn links(blocks: &mut Blocks, used_vars: &mut VarSet) {
    // For each block, ids of following blocks
    let nexts = blocks
        .vec
        .iter()
        .map(|block| {
            let last = block.instrs.last().unwrap();
            let Effect::Jmp(jmp) = &last.eff else {
                log::warn!("block {:x} does not end with jmp", block.addr());
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
                .flat_map(|addr| blocks.vec.iter().position(|b| b.addr() == addr))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // For each block, input vars to block
    let mut bins: Vec<VarSet> = Default::default();
    // For each block, output vars from block
    let mut bouts: Vec<HashMap<String, Var>> = Default::default();

    for block in blocks.vec.iter_mut() {
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
                    .map(|p| (p.clone(), outs.get(&p.reg).unwrap().clone()))
                    .collect();
                Link {
                    addr: blocks.vec[next_id].addr(),
                    params,
                }
            })
            .collect();

        blocks.vec[id].params = bins[id].clone();
        blocks.vec[id].links = next;
    }
}

/// Find the variables that are live at the end of the block, which will be potential parameters to the next blocks.
fn out_vars(block: &mut Block) -> HashMap<String, Var> {
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

/// If a variable is only used once, inline it into where it is used.
fn inline_block(block: &mut Block) {
    log::info!("inline {:x}", block.addr());

    // Gather the inlinable new vars introduced by this block
    let mut new_vars: HashSet<Var> = HashSet::default();
    // for var in block.params.iter() {
    //     new_vars.insert(var.clone());
    // }
    for instr in block.instrs.iter() {
        match &instr.eff {
            Effect::Set(Expr::Var(dst), _) => {
                new_vars.insert(dst.clone());
            }
            _ => {}
        }
    }
    log::info!("new vars {:?}", new_vars);

    // Count the times they are used
    let mut used_once: HashSet<Var> = HashSet::default();
    let mut used_multi: HashSet<Var> = HashSet::default();
    let mut mark_read = |var: &Var| {
        if !new_vars.contains(var) {
            return;
        }
        if used_once.get(var).is_some() {
            used_once.remove(var);
            used_multi.insert(var.clone());
        } else {
            used_once.insert(var.clone());
        }
    };
    let visit = &mut |expr: &mut Expr| match expr {
        Expr::Var(var) => mark_read(var),
        _ => {}
    };
    for instr in block.instrs.iter_mut() {
        match &mut instr.eff {
            Effect::Set(_, src) => visit_expr(src, visit),
            eff => visit_effect(eff, visit),
        }
    }
    for link in block.links.iter() {
        for (_, val) in link.params.iter() {
            mark_read(val);
        }
    }
    log::info!("used_once: {:?}", used_once);
    log::info!("used_multi: {:?}", used_multi);
}

fn inline(blocks: &mut Blocks) {
    for block in blocks.vec.iter_mut() {
        inline_block(block);
    }
}

fn print_block(block: &Block) {
    println!(
        "{ip:x} [{params}]",
        ip = block.addr(),
        params = block
            .params
            .iter()
            .map(|v| format!("{v}"))
            .collect::<Vec<_>>()
            .join(" ")
    );
    for instr in &block.instrs {
        let text = format!("{}", instr.eff);
        println!(
            "{text:40}  ; {ip:x} {iced}",
            ip = instr.iced.ip32(),
            iced = instr.iced
        );
    }
    for link in &block.links {
        println!(
            "=> {:x} {}",
            link.addr,
            link.params
                .iter()
                .map(|(k, v)| format!("{k}={v}"))
                .collect::<Vec<_>>()
                .join(" ")
        );
    }
}

fn print(blocks: &Blocks) {
    for block in &blocks.vec {
        print_block(block);
        println!();
    }
}

#[derive(argh::FromArgs)]
/// todo
struct Args {
    /// output json
    #[argh(switch)]
    json: bool,
}

fn main() {
    logger::init();
    let args: Args = argh::from_env();

    let instrs = decode();
    let mut blocks = blocks(instrs);

    //blocks.vec.truncate(5);

    simplify_branches(&mut blocks);
    let mut used_vars = VarSet::default();
    ssa(&mut blocks, &mut used_vars);
    links(&mut blocks, &mut used_vars);
    inline(&mut blocks);

    if args.json {
        std::fs::write("web/data.json", serde_json::to_string(&blocks).unwrap()).unwrap();
    } else {
        print(&blocks);
    }
}

const IP: u32 = 0x401d0f;
const ASM: &'static [u8] = b"\x53\x51\x52\x56\x2e\xff\x15\x58\xb1\x40\x00\x8b\x15\x7c\x3f\x43\x00\x31\xdb\x29\xd0\x31\xf6\xa3\x84\x3f\x43\x00\x31\xd2\xeb\x18\xc7\x80\xac\x39\x43\x00\x00\x00\x00\x00\x42\x83\xfa\x28\x0f\x8d\xa3\x00\x00\x00\x85\xf6\x75\x44\x6b\xc2\x0c\x83\xb8\x28\x3c\x43\x00\x01\x75\x38\x8b\x88\x24\x3c\x43\x00\x81\xc1\xfa\x00\x00\x00\x3b\x0d\x84\x3f\x43\x00\x7d\x24\xc7\x80\x28\x3c\x43\x00\x00\x00\x00\x00\xc7\x05\x53\xc7\x40\x00\x01\x00\x00\x00\x8b\x80\x20\x3c\x43\x00\xbe\x01\x00\x00\x00\xa3\x57\xc7\x40\x00\x89\xd0\xc1\xe0\x04\x83\xb8\xac\x39\x43\x00\x01\x75\xa0\x8b\x88\xa8\x39\x43\x00\x81\xc1\xfa\x00\x00\x00\x3b\x0d\x84\x3f\x43\x00\x7d\x8c\x3b\x98\xa8\x39\x43\x00\x0f\x8d\x76\xff\xff\xff\x8b\x88\xa0\x39\x43\x00\x8b\x98\xa8\x39\x43\x00\x89\x0d\x3f\xc7\x40\x00\x8b\x88\xa4\x39\x43\x00\xc7\x80\xac\x39\x43\x00\x00\x00\x00\x00\x89\x0d\x43\xc7\x40\x00\xe9\x53\xff\xff\xff\x5e\x5a\x59\x5b\xc3";
