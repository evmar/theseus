#![allow(dead_code)]

mod ast;
use ast::*;
mod ssa;
use ssa::*;

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

/// If a variable is only used once, inline it into where it is used.
fn inline_block(block: &mut Block) {
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

    // Count the times they are used
    let mut used: HashMap<Var, usize> = Default::default();
    let mut mark_read = |var: &Var| {
        if !new_vars.contains(var) {
            return;
        }
        *used.entry(var.clone()).or_default() += 1;
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

        for instr in block.instrs.iter_mut() {
            visit_effect(&mut instr.eff, &mut |expr| {
                if let Expr::Var(dst) = expr {
                    if dst == var {
                        *expr = val.clone();
                    }
                }
            });
        }
    }
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
    ssa(&mut blocks);
    inline(&mut blocks);

    if args.json {
        std::fs::write("web/data.json", serde_json::to_string(&blocks).unwrap()).unwrap();
    } else {
        print(&blocks);
    }
}

const IP: u32 = 0x401d0f;
const ASM: &'static [u8] = b"\x53\x51\x52\x56\x2e\xff\x15\x58\xb1\x40\x00\x8b\x15\x7c\x3f\x43\x00\x31\xdb\x29\xd0\x31\xf6\xa3\x84\x3f\x43\x00\x31\xd2\xeb\x18\xc7\x80\xac\x39\x43\x00\x00\x00\x00\x00\x42\x83\xfa\x28\x0f\x8d\xa3\x00\x00\x00\x85\xf6\x75\x44\x6b\xc2\x0c\x83\xb8\x28\x3c\x43\x00\x01\x75\x38\x8b\x88\x24\x3c\x43\x00\x81\xc1\xfa\x00\x00\x00\x3b\x0d\x84\x3f\x43\x00\x7d\x24\xc7\x80\x28\x3c\x43\x00\x00\x00\x00\x00\xc7\x05\x53\xc7\x40\x00\x01\x00\x00\x00\x8b\x80\x20\x3c\x43\x00\xbe\x01\x00\x00\x00\xa3\x57\xc7\x40\x00\x89\xd0\xc1\xe0\x04\x83\xb8\xac\x39\x43\x00\x01\x75\xa0\x8b\x88\xa8\x39\x43\x00\x81\xc1\xfa\x00\x00\x00\x3b\x0d\x84\x3f\x43\x00\x7d\x8c\x3b\x98\xa8\x39\x43\x00\x0f\x8d\x76\xff\xff\xff\x8b\x88\xa0\x39\x43\x00\x8b\x98\xa8\x39\x43\x00\x89\x0d\x3f\xc7\x40\x00\x8b\x88\xa4\x39\x43\x00\xc7\x80\xac\x39\x43\x00\x00\x00\x00\x00\x89\x0d\x43\xc7\x40\x00\xe9\x53\xff\xff\xff\x5e\x5a\x59\x5b\xc3";
