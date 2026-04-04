use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Var {
    reg: String,
    ver: usize,
}

impl std::fmt::Display for Var {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.ver > 0 {
            write!(f, "{reg}#{ver}", reg = self.reg, ver = self.ver)
        } else {
            write!(f, "{reg}", reg = self.reg)
        }
    }
}

impl Var {
    fn new(reg: String) -> Self {
        Var { reg, ver: 0 }
    }
    fn next(&self) -> Var {
        Var {
            reg: self.reg.clone(),
            ver: self.ver + 1,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Expr {
    Const(u32),
    Var(Var),
    Call(Box<Call>),
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Const(c) => write!(f, "{c:#x}"),
            Expr::Var(v) => write!(f, "{v}"),
            Expr::Call(call) => call.fmt(f),
        }
    }
}

impl Expr {
    fn from_reg(r: iced_x86::Register) -> Expr {
        Expr::Var(Var::new(format!("{r:?}").to_ascii_lowercase()))
    }
}

impl From<u32> for Expr {
    fn from(value: u32) -> Self {
        Expr::Const(value)
    }
}
impl From<Call> for Expr {
    fn from(value: Call) -> Self {
        Expr::Call(Box::new(value))
    }
}
impl From<&Expr> for Expr {
    fn from(value: &Expr) -> Self {
        value.clone()
    }
}

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
    ($x:expr, $($arg:expr),+) => { crate::Call { op: $x.into(), args: vec![$($arg.into()),*] } };
}

// macro_rules! expr {
//     ($x:expr) => { Expr::Const($x) };
//     ($x:expr, $($arg:expr) +) => { Expr::Call(Box::new(Call { op: $x, args: vec![$($arg),*] })) };
// }

#[derive(Debug, Clone, PartialEq, Eq)]
struct Call {
    op: String,
    args: Vec<Expr>,
}

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

impl std::fmt::Display for Call {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({op}", op = self.op)?;
        for arg in &self.args {
            write!(f, " {arg}")?;
        }
        write!(f, ")")
    }
}

#[derive(Debug)]
struct Instr {
    ip: u32,
    iced: iced_x86::Instruction,
    call: Call,
}

fn decode() -> Vec<Instr> {
    let mut instrs = Vec::new();
    let decoder = iced_x86::Decoder::with_ip(32, ASM, IP as u64, iced_x86::DecoderOptions::NONE);
    for instr in decoder {
        let mut args = Vec::new();
        for i in 0..instr.op_count() {
            args.push(expr_from_iced(instr, i));
        }

        use iced_x86::Mnemonic::*;
        let call = match instr.mnemonic() {
            Inc => {
                let [x] = args.as_slice() else { unreachable!() };
                call!("set", x, call!("+", x, Expr::Const(1)))
            }
            Mov => {
                let [x, y] = args.as_slice() else {
                    unreachable!()
                };
                call!("set", x, y)
            }
            Add | Sub | Shl | Xor => {
                let op = match instr.mnemonic() {
                    Add => "+",
                    Sub => "-",
                    Shl => "<<",
                    Xor => "^",
                    _ => unreachable!(),
                };
                let [x, y] = args.as_slice() else {
                    unreachable!()
                };
                if op == "^" && x == y {
                    call!("set", x, 0)
                } else {
                    call!("set", x, call!(op, x, y))
                }
            }
            Cmp | Test | Jmp | Jne | Jge | Call | Ret => {
                let op = format!("{:?}", instr.mnemonic()).to_ascii_lowercase();
                crate::Call { op, args }
            }
            _ => {
                let op = format!("todo:{:?}", instr.mnemonic()).to_ascii_lowercase();
                crate::Call { op, args }
            }
        };

        instrs.push(Instr {
            ip: instr.ip32(),
            iced: instr,
            call,
        });
    }
    instrs
}

#[derive(Debug)]
struct Link {
    addr: u32,
    params: Vec<(String, Var)>,
}

struct Block {
    params: Vec<String>,
    instrs: Vec<Instr>,
    next: Vec<Link>,
}

impl Block {
    fn addr(&self) -> u32 {
        self.instrs[0].ip
    }
}

fn blocks(instrs: Vec<Instr>) -> Vec<Block> {
    let mut blocks = vec![];
    let mut block = vec![];
    for instr in instrs {
        let next = match instr.call.op.as_str() {
            "jmp" | "call" => Some(vec![instr.iced.next_ip32()]),
            "jne" | "jge" => {
                let [Expr::Const(addr)] = instr.call.args.as_slice() else {
                    panic!()
                };
                Some(vec![instr.iced.next_ip32(), *addr])
            }
            "ret" => Some(vec![0]),
            _ => None,
        };
        block.push(instr);
        if let Some(next) = next {
            blocks.push(Block {
                params: vec![],
                instrs: block,
                next: next
                    .into_iter()
                    .map(|addr| Link {
                        addr,
                        params: vec![],
                    })
                    .collect(),
            });
            block = vec![];
        }
    }
    if !block.is_empty() {
        println!("{:#?}", block);
        assert!(block.is_empty());
    }
    blocks
}

fn visit(call: &mut Call, f: &mut impl FnMut(&mut Expr) -> bool) {
    for arg in call.args.iter_mut() {
        if f(arg) {
            if let Expr::Call(c) = arg {
                visit(c, f);
            }
        }
    }
}

fn visit_block(block: &mut Block, f: &mut impl FnMut(&mut Expr) -> bool) {
    for instr in block.instrs.iter_mut() {
        visit(&mut instr.call, f);
    }
}

fn rename(call: &mut Call, from: &Var, to: &Var) {
    visit(call, &mut |expr| match expr {
        Expr::Var(v) if v == from => {
            *v = to.clone();
            true
        }
        _ => true,
    });
}

fn ssa(instrs: &mut [Instr]) {
    for i in 0..instrs.len() {
        let (instr, rest) = instrs[i..].split_first_mut().unwrap();
        let call = &mut instr.call;
        match call.op.as_str() {
            "set" => {
                let Expr::Var(var) = &call.args[0] else {
                    continue;
                };
                let new_local = var.next();
                for instr in rest {
                    rename(&mut instr.call, &var, &new_local);
                }
                call.args[0] = Expr::Var(new_local);
            }
            _ => {}
        }
    }
}

fn params(block: &mut Block) {
    let mut params = HashSet::new();
    visit_block(block, &mut |expr| match expr {
        Expr::Call(call) if call.op.starts_with("todo:") => false,
        Expr::Var(var) => {
            // XXX this only should be for reads, not writes
            params.insert(var.reg.clone());
            true
        }
        _ => true,
    });

    let mut params = params.into_iter().collect::<Vec<_>>();
    params.sort();
    block.params = params;
}

fn links(blocks: &mut [Block]) {
    #[derive(Default)]
    struct IO {
        outs: HashMap<String, Var>,
        ins: HashSet<String>,
    }

    let mut ios = HashMap::new();
    for block in blocks.iter_mut() {
        ssa(&mut block.instrs);

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
            true
        });

        params(block);
        ios.insert(
            block.addr(),
            IO {
                outs,
                ins: HashSet::from_iter(block.params.iter().map(|p| p.clone())),
            },
        );
    }

    let mut changed = true;
    while changed {
        changed = false;
        for src in blocks.iter() {
            let outs = &ios.get(&src.addr()).unwrap().outs;
            let mut add = vec![];
            for link in &src.next {
                let Some(dst) = blocks.iter().find(|b| b.addr() == link.addr) else {
                    println!("can't find {:x}", link.addr);
                    continue;
                };
                for param in &ios.get(&dst.addr()).unwrap().ins {
                    if !outs.contains_key(param.as_str()) {
                        add.push(param.clone());
                    }
                }
            }

            if !add.is_empty() {
                let io = &mut ios.get_mut(&src.addr()).unwrap();
                for add in add {
                    io.ins.insert(add.clone());
                    io.outs.insert(add.clone(), Var::new(add));
                }
                changed = true;
            }
        }
    }

    for (&addr, io) in ios.iter() {
        let block = blocks.iter_mut().find(|b| b.addr() == addr).unwrap();
        block.params = io.ins.iter().map(|s| s.clone()).collect();
        block.params.sort();
        let params = block
            .next
            .iter()
            .map(|link| {
                let Some(next) = ios.get(&link.addr) else {
                    return Link {
                        addr: link.addr,
                        params: vec![],
                    };
                };
                let params = next
                    .ins
                    .iter()
                    .map(|p| (p.clone(), io.outs.get(p).unwrap().clone()))
                    .collect();
                Link {
                    addr: link.addr,
                    params,
                }
            })
            .collect();
        block.next = params;
    }
}

fn main() {
    let instrs = decode();
    let mut blocks = blocks(instrs);
    links(&mut blocks);
    for block in blocks {
        println!(
            "{ip:x} [{params}]",
            ip = block.instrs[0].ip,
            params = block.params.join(" ")
        );
        for instr in &block.instrs {
            let text = format!("{}", instr.call);
            println!(
                "{text:40}  ; {ip:x} {iced}",
                ip = instr.ip,
                iced = instr.iced
            );
        }
        for link in block.next {
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
        println!();
    }
}

const IP: u32 = 0x401d0f;
const ASM: &'static [u8] = b"\x53\x51\x52\x56\x2e\xff\x15\x58\xb1\x40\x00\x8b\x15\x7c\x3f\x43\x00\x31\xdb\x29\xd0\x31\xf6\xa3\x84\x3f\x43\x00\x31\xd2\xeb\x18\xc7\x80\xac\x39\x43\x00\x00\x00\x00\x00\x42\x83\xfa\x28\x0f\x8d\xa3\x00\x00\x00\x85\xf6\x75\x44\x6b\xc2\x0c\x83\xb8\x28\x3c\x43\x00\x01\x75\x38\x8b\x88\x24\x3c\x43\x00\x81\xc1\xfa\x00\x00\x00\x3b\x0d\x84\x3f\x43\x00\x7d\x24\xc7\x80\x28\x3c\x43\x00\x00\x00\x00\x00\xc7\x05\x53\xc7\x40\x00\x01\x00\x00\x00\x8b\x80\x20\x3c\x43\x00\xbe\x01\x00\x00\x00\xa3\x57\xc7\x40\x00\x89\xd0\xc1\xe0\x04\x83\xb8\xac\x39\x43\x00\x01\x75\xa0\x8b\x88\xa8\x39\x43\x00\x81\xc1\xfa\x00\x00\x00\x3b\x0d\x84\x3f\x43\x00\x7d\x8c\x3b\x98\xa8\x39\x43\x00\x0f\x8d\x76\xff\xff\xff\x8b\x88\xa0\x39\x43\x00\x8b\x98\xa8\x39\x43\x00\x89\x0d\x3f\xc7\x40\x00\x8b\x88\xa4\x39\x43\x00\xc7\x80\xac\x39\x43\x00\x00\x00\x00\x00\x89\x0d\x43\xc7\x40\x00\xe9\x53\xff\xff\xff\x5e\x5a\x59\x5b\xc3";
