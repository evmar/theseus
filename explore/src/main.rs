#![allow(dead_code)]

mod ast;
use ast::*;
mod ssa;
use ssa::*;
mod inline;
use inline::*;

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

#[macro_export]
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
fn decode() -> (Vec<Instr>, Vec<iced_x86::Instruction>) {
    let mut instrs = vec![];
    let mut iced = vec![];
    let decoder = iced_x86::Decoder::with_ip(32, ASM, IP as u64, iced_x86::DecoderOptions::NONE);
    for instr in decoder {
        iced.push(instr);
        let mut args = vec![];
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
                    src: iced.len() - 1,
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
                    src: iced.len() - 1,
                    eff: Effect::Call(Box::new(crate::Call {
                        op: "call".into(),
                        args: vec![dst],
                    })),
                });
                // assume stdcall
                instrs.push(Instr {
                    src: iced.len() - 1,
                    eff: Effect::Set(
                        Expr::Var(Var::new("eax".into())),
                        Expr::Var(Var {
                            reg: "?".into(),
                            ver: instr.ip32() as usize,
                        }),
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

        instrs.push(Instr {
            src: iced.len() - 1,
            eff,
        });
    }
    (instrs, iced)
}

/// Split the instructions into basic blocks, where each block ends with a jump.
fn blocks(instrs: Vec<Instr>, iced: Vec<iced_x86::Instruction>) -> Blocks {
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
                        addr = iced[instr.src].ip32(),
                        expr = dst
                    );
                    continue;
                }
            };
            targets.insert(dst);
        }
    }

    let mut blocks = vec![];
    let mut block: Vec<Instr> = vec![];
    for instr in instrs {
        let is_jmp = matches!(instr.eff, Effect::Jmp(_));
        let src = instr.src;
        let next_ip = iced[src].next_ip32();
        let last_in_block = is_jmp || targets.contains(&next_ip);

        block.push(instr);
        if !last_in_block {
            continue;
        }

        if !is_jmp {
            block.push(Instr {
                src,
                eff: Effect::Jmp(Box::new(Jmp::new("jmp", vec![Expr::Const(next_ip)]))),
            });
        }
        let iced = iced[block[0].src..block[block.len() - 1].src + 1].to_vec();
        let offset = block[0].src;
        for instr in block.iter_mut() {
            instr.src -= offset;
        }
        blocks.push(Block {
            id: blocks.len(),
            addr: iced[0].ip32(),
            instrs: block,
            iced,
            params: Default::default(),
            links: vec![],
        });
        block = vec![];
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
            cur.src = prev.src;
            block.instrs.remove(i - 2);
        }
    }
}

fn print_block(blocks: &Blocks, block: &Block) {
    println!(
        "{ip:x} [{params}]",
        ip = block.addr,
        params = block
            .params
            .iter()
            .map(|var| format!("{var}"))
            .collect::<Vec<_>>()
            .join(" ")
    );
    for instr in &block.instrs {
        let text = format!("{}", instr.eff);
        let iced = block.iced[instr.src];
        println!("{text:40}  ; {ip:x} {iced}", ip = iced.ip32(), iced = iced);
    }
    for link in &block.links {
        println!("=> {:x}", blocks.vec[link.id].addr);
    }
}

fn print(blocks: &Blocks) {
    for block in &blocks.vec {
        print_block(&blocks, block);
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

#[derive(Default)]
struct Union {
    map: HashMap<Var, Var>,
}

impl Union {
    fn insert(&mut self, v: &Var) {
        if !self.map.contains_key(v) {
            self.map.insert(v.clone(), v.clone());
        }
    }

    fn join(&mut self, v1: &Var, v2: &Var) {
        let v1 = self.find(v1);
        let v2 = self.find(v2);
        if v1 < v2 {
            self.map.insert(v2.clone(), v1.clone());
        } else if v1 > v2 {
            self.map.insert(v1.clone(), v2.clone());
        }
    }

    fn find<'a>(&'a self, v: &Var) -> &'a Var {
        let mut v = v;
        loop {
            let next = self.map.get(v).unwrap();
            if next == v {
                return next;
            }
            v = next;
        }
    }

    fn sets(&self) -> Vec<Vec<Var>> {
        let mut sets: HashMap<&Var, HashSet<Var>> = HashMap::new();
        for (v, u) in self.map.iter() {
            let u = self.find(u);
            log::info!("{} -> {}", v, u);
            sets.entry(u).or_default().insert(v.clone());
        }
        sets.into_values()
            .map(|set| set.into_iter().collect())
            .collect()
    }
}

// gather sets of all vars used together in phi
fn union(blocks: &mut Blocks) {
    let mut unions = Union::default();
    let gather_phi = &mut |expr: &Expr, vars: &mut HashSet<Var>| {
        let Expr::Call(call) = &expr else {
            return;
        };
        if call.op != "phi" {
            return;
        }
        for arg in call.args.iter() {
            if let Expr::Var(var) = arg {
                vars.insert(var.clone());
            }
        }
    };
    for block in blocks.vec.iter() {
        for instr in block.instrs.iter() {
            let mut vars = HashSet::new();
            match &instr.eff {
                Effect::Def(var, phi) => {
                    vars.insert(var.clone());
                    if let Expr::Var(v) = phi {
                        vars.insert(v.clone());
                    }
                    gather_phi(phi, &mut vars);
                }
                _ => visit_effect(&instr.eff, &mut |expr| gather_phi(expr, &mut vars)),
            }
            let mut iter = vars.iter();
            if let Some(v1) = iter.next() {
                unions.insert(v1);
                for v2 in iter {
                    unions.insert(v2);
                    log::info!("{}+{} from {}", v1, v2, instr.eff);
                    unions.join(v1, v2);
                }
            }
        }
    }

    for mut set in unions.sets() {
        set.sort();
        log::info!(
            "union: {}",
            set.iter()
                .map(|v| format!("{v}"))
                .collect::<Vec<_>>()
                .join(" ")
        );
    }
}

fn main() {
    logger::init();
    let args: Args = argh::from_env();

    let (instrs, iced) = decode();
    let mut blocks = blocks(instrs, iced);

    //blocks.vec.truncate(5);

    simplify_branches(&mut blocks);
    ssa(&mut blocks);
    //inline(&mut blocks);
    // union(&mut blocks);

    if args.json {
        std::fs::write("web/data.json", serde_json::to_string(&blocks).unwrap()).unwrap();
    } else {
        print(&blocks);
    }
}

const IP: u32 = 0x401d0f;
const ASM: &'static [u8] = b"\x53\x51\x52\x56\x2e\xff\x15\x58\xb1\x40\x00\x8b\x15\x7c\x3f\x43\x00\x31\xdb\x29\xd0\x31\xf6\xa3\x84\x3f\x43\x00\x31\xd2\xeb\x18\xc7\x80\xac\x39\x43\x00\x00\x00\x00\x00\x42\x83\xfa\x28\x0f\x8d\xa3\x00\x00\x00\x85\xf6\x75\x44\x6b\xc2\x0c\x83\xb8\x28\x3c\x43\x00\x01\x75\x38\x8b\x88\x24\x3c\x43\x00\x81\xc1\xfa\x00\x00\x00\x3b\x0d\x84\x3f\x43\x00\x7d\x24\xc7\x80\x28\x3c\x43\x00\x00\x00\x00\x00\xc7\x05\x53\xc7\x40\x00\x01\x00\x00\x00\x8b\x80\x20\x3c\x43\x00\xbe\x01\x00\x00\x00\xa3\x57\xc7\x40\x00\x89\xd0\xc1\xe0\x04\x83\xb8\xac\x39\x43\x00\x01\x75\xa0\x8b\x88\xa8\x39\x43\x00\x81\xc1\xfa\x00\x00\x00\x3b\x0d\x84\x3f\x43\x00\x7d\x8c\x3b\x98\xa8\x39\x43\x00\x0f\x8d\x76\xff\xff\xff\x8b\x88\xa0\x39\x43\x00\x8b\x98\xa8\x39\x43\x00\x89\x0d\x3f\xc7\x40\x00\x8b\x88\xa4\x39\x43\x00\xc7\x80\xac\x39\x43\x00\x00\x00\x00\x00\x89\x0d\x43\xc7\x40\x00\xe9\x53\xff\xff\xff\x5e\x5a\x59\x5b\xc3";
