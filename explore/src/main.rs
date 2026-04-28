#![allow(dead_code)]

mod ast;
use ast::*;
mod ssa;
use ssa::*;
mod inline;
use inline::*;
mod union;

use std::collections::HashSet;

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
    let (ip, asm) = P3;
    let decoder = iced_x86::Decoder::with_ip(32, asm, ip as u64, iced_x86::DecoderOptions::NONE);
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
            Dec => {
                let [x] = args.try_into().unwrap();
                Effect::Set(x.clone(), call!("-", x, Expr::Const(1)))
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
            Add | Sub | Shl | Xor | And | Sar => {
                let op = match instr.mnemonic() {
                    Add => "+",
                    Sub => "-",
                    Shl => "<<",
                    Sar => ">>",
                    Xor => "^",
                    And => "&",
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
            Imul => match args.len() {
                2 => {
                    let [x, y] = args.try_into().unwrap();
                    Effect::Set(x.clone(), call!("imul", x, y))
                }
                3 => {
                    let [x, y, z] = args.try_into().unwrap();
                    Effect::Set(x, call!("imul", y, z))
                }
                _ => unreachable!("{instr}"),
            },
            Idiv => {
                let [x] = args.try_into().unwrap();
                instrs.push(Instr {
                    src: iced.len() - 1,
                    eff: Effect::Set(
                        Expr::Var(Var::new("idiv".into())),
                        call!(
                            "idiv",
                            Expr::Var(Var::new("edx".into())),
                            Expr::Var(Var::new("eax".into())),
                            x
                        ),
                    ),
                });
                instrs.push(Instr {
                    src: iced.len() - 1,
                    eff: Effect::Set(
                        Expr::Var(Var::new("eax".into())),
                        call!("idiv-quot", Expr::Var(Var::new("idiv".into()))),
                    ),
                });
                instrs.push(Instr {
                    src: iced.len() - 1,
                    eff: Effect::Set(
                        Expr::Var(Var::new("edx".into())),
                        call!("idiv-rem", Expr::Var(Var::new("idiv".into()))),
                    ),
                });

                continue;
            }
            Cdq => Effect::Set(
                Expr::Var(Var::new("edx".into())),
                call!("sign-extend-cdq", Expr::Var(Var::new("eax".into()))),
            ),
            Lea => {
                log::info!("{:?}", args);
                let [x, mut y] = args.try_into().unwrap();
                let Expr::Call(call) = &mut y else {
                    unreachable!();
                };
                assert_eq!(call.op, "mem");
                call.op = "+".into();
                Effect::Set(x, y)
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
            Je | Jne | Jl | Jle | Jg | Jge | Jb | Jbe | Ja | Jae | Js | Jns | Jo | Jno | Jp
            | Jnp => {
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

            let is_self_test = test.op == "test" && test.args[0] == test.args[1];

            let op = match jmp.cond.op.as_str() {
                "je" => match test.op.as_str() {
                    "cmp" => "==",
                    "test" if is_self_test => {
                        test.args.truncate(1);
                        "==0"
                    }
                    _ => continue,
                },
                "ja" => match test.op.as_str() {
                    "cmp" => "u>",
                    "test" if is_self_test => {
                        test.args.truncate(1);
                        "!=0"
                    }
                    _ => continue,
                },
                "jae" => match test.op.as_str() {
                    "cmp" => "u>=",
                    _ => continue,
                },
                "jb" => match test.op.as_str() {
                    "cmp" => "u<",
                    _ => continue,
                },
                "jbe" => match test.op.as_str() {
                    "cmp" => "u<=",
                    "test" if is_self_test => {
                        test.args.truncate(1);
                        "==0"
                    }
                    _ => continue,
                },
                "jge" => match test.op.as_str() {
                    "cmp" => ">=",
                    "test" if is_self_test => {
                        test.args.truncate(1);
                        ">=0"
                    }
                    _ => continue,
                },
                "jg" => match test.op.as_str() {
                    "cmp" => ">",
                    "test" if is_self_test => {
                        test.args.truncate(1);
                        ">0"
                    }
                    _ => continue,
                },
                "jl" => match test.op.as_str() {
                    "cmp" => "<",
                    "test" if is_self_test => {
                        test.args.truncate(1);
                        "<0"
                    }
                    _ => continue,
                },
                "jne" => match test.op.as_str() {
                    "cmp" => "!=",
                    "test" if is_self_test => {
                        test.args.truncate(1);
                        "!=0"
                    }
                    _ => continue,
                },
                "jns" => match test.op.as_str() {
                    "test" if is_self_test => {
                        test.args.truncate(1);
                        ">=0"
                    }
                    _ => continue,
                },
                "js" => match test.op.as_str() {
                    "test" if is_self_test => {
                        test.args.truncate(1);
                        "<0"
                    }
                    _ => continue,
                },
                "jle" => match test.op.as_str() {
                    "cmp" => "<=",
                    "test" if is_self_test => {
                        test.args.truncate(1);
                        "<=0"
                    }
                    _ => continue,
                },
                _ => continue,
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

fn main() {
    logger::init();
    let args: Args = argh::from_env();

    let (instrs, iced) = decode();
    let mut blocks = blocks(instrs, iced);

    //blocks.vec.truncate(5);

    simplify_branches(&mut blocks);
    ssa(&mut blocks);
    inline(&mut blocks);
    // union(&mut blocks);

    if args.json {
        std::fs::write("web/data.json", serde_json::to_string(&blocks).unwrap()).unwrap();
    } else {
        print(&blocks);
    }
}

// from chillin-unpacked
const P1: (u32, &'static [u8]) = (0x401d0f, b"\x53\x51\x52\x56\x2e\xff\x15\x58\xb1\x40\x00\x8b\x15\x7c\x3f\x43\x00\x31\xdb\x29\xd0\x31\xf6\xa3\x84\x3f\x43\x00\x31\xd2\xeb\x18\xc7\x80\xac\x39\x43\x00\x00\x00\x00\x00\x42\x83\xfa\x28\x0f\x8d\xa3\x00\x00\x00\x85\xf6\x75\x44\x6b\xc2\x0c\x83\xb8\x28\x3c\x43\x00\x01\x75\x38\x8b\x88\x24\x3c\x43\x00\x81\xc1\xfa\x00\x00\x00\x3b\x0d\x84\x3f\x43\x00\x7d\x24\xc7\x80\x28\x3c\x43\x00\x00\x00\x00\x00\xc7\x05\x53\xc7\x40\x00\x01\x00\x00\x00\x8b\x80\x20\x3c\x43\x00\xbe\x01\x00\x00\x00\xa3\x57\xc7\x40\x00\x89\xd0\xc1\xe0\x04\x83\xb8\xac\x39\x43\x00\x01\x75\xa0\x8b\x88\xa8\x39\x43\x00\x81\xc1\xfa\x00\x00\x00\x3b\x0d\x84\x3f\x43\x00\x7d\x8c\x3b\x98\xa8\x39\x43\x00\x0f\x8d\x76\xff\xff\xff\x8b\x88\xa0\x39\x43\x00\x8b\x98\xa8\x39\x43\x00\x89\x0d\x3f\xc7\x40\x00\x8b\x88\xa4\x39\x43\x00\xc7\x80\xac\x39\x43\x00\x00\x00\x00\x00\x89\x0d\x43\xc7\x40\x00\xe9\x53\xff\xff\xff\x5e\x5a\x59\x5b\xc3");

// from mofo-dump
const P2: (u32, &'static [u8]) = (0x4071b0, b"\xa1\x14\x17\x42\x00\x83\xec\x10\x53\x55\x56\x57\x50\xe8\xce\x0c\x00\x00\x8b\x74\x24\x34\x83\xc4\x04\x33\xc9\x8b\xf8\x85\xf6\x0f\x84\x4d\x01\x00\x00\x8b\x5c\x24\x2c\x85\xdb\x0f\x84\x41\x01\x00\x00\x33\xd2\x85\xff\x0f\x8e\x37\x01\x00\x00\x8a\x44\x24\x34\x8b\x2d\x14\x17\x42\x00\x38\x04\x2a\x75\x02\x8b\xca\x42\x3b\xd7\x7c\xf4\x85\xc9\x0f\x84\x19\x01\x00\x00\x8b\x7c\x24\x24\x8b\xc3\x99\x2b\xc2\x8b\x6c\x24\x28\xd1\xf8\x2b\xf8\xb8\x00\x80\x00\x00\x99\xf7\xfb\xc1\xe1\x08\x89\x7c\x24\x18\xc7\x44\x24\x34\x00\x00\x00\x00\x89\x44\x24\x14\xb8\x00\x80\x00\x00\x99\xf7\xfe\x89\x44\x24\x1c\xa1\xe0\x8c\x42\x00\x03\xc8\x8b\xc6\x99\x2b\xc2\x89\x4c\x24\x10\xd1\xf8\x2b\xe8\x85\xf6\x0f\x8e\xc5\x00\x00\x00\x8d\x6c\xad\x00\x89\x74\x24\x28\xc1\xe5\x07\x33\xc0\x8b\xf7\x85\xdb\x89\x44\x24\x30\x0f\x8e\x87\x00\x00\x00\x89\x5c\x24\x24\x85\xf6\x7c\x61\x81\xfe\x80\x02\x00\x00\x7d\x59\x85\xed\x7c\x55\x81\xfd\x00\xf4\x01\x00\x7d\x4d\x8b\x54\x24\x34\x8b\x5c\x24\x10\xc1\xfa\x07\x83\xe2\xf0\x8b\x3d\xf8\x8c\x42\x00\xc1\xf8\x0b\x03\xda\x33\xd2\x8d\x0c\x2e\x8a\x14\x18\x8b\xc2\x8b\x54\x24\x38\x81\xe2\xff\x00\x00\x00\x0f\xaf\xc2\x33\xd2\x8a\x14\x0f\xc1\xf8\x08\x03\xc2\x3d\xff\x00\x00\x00\x7e\x05\xb8\xff\x00\x00\x00\x8b\x5c\x24\x2c\x88\x04\x0f\x8b\x44\x24\x30\x8b\x7c\x24\x14\x8b\x4c\x24\x24\x03\xc7\x46\x49\x89\x44\x24\x30\x89\x4c\x24\x24\x75\x81\x8b\x7c\x24\x18\x8b\x44\x24\x1c\x8b\x54\x24\x34\x03\xd0\x8b\x44\x24\x28\x81\xc5\x80\x02\x00\x00\x48\x89\x54\x24\x34\x89\x44\x24\x28\x0f\x85\x46\xff\xff\xff\x5f\x5e\x5d\x5b\x83\xc4\x10\xc3");

// strlen, from mofo-dump
const P3: (u32, &'static [u8]) = (
    0x407e90,
    b"\x8b\x4c\x24\x04\x33\xc0\x80\x39\x00\x74\x09\x8a\x54\x08\x01\x40\x84\xd2\x75\xf7\xc3",
);
