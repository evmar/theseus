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
    ($x:expr, $($arg:expr),+) => { crate::Expr::Call(Box::new(crate::Call { op: $x.into(), args: vec![$($arg.into()),*] })) };
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
enum Effect {
    Set(Expr, Expr),
    Call(Box<Call>),
    Jmp(String, Vec<u32>),
}

impl std::fmt::Display for Effect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Effect::Set(x, y) => write!(f, "{x} = {y}"),
            Effect::Call(call) => write!(f, "{call}"),
            Effect::Jmp(op, next) => write!(
                f,
                "{op} {next}",
                next = next
                    .iter()
                    .map(|x| format!("{x:x}"))
                    .collect::<Vec<_>>()
                    .join(" ")
            ),
        }
    }
}

#[derive(Debug)]
struct Instr {
    ip: u32,
    iced: iced_x86::Instruction,
    eff: Effect,
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
        let instr_name = format!("{:?}", instr.mnemonic()).to_ascii_lowercase();
        let eff: Effect = match instr.mnemonic() {
            Inc => {
                let [x] = args.try_into().unwrap();
                Effect::Set(x.clone(), call!("+", x, Expr::Const(1)))
            }
            Mov => {
                let [x, y] = args.try_into().unwrap();
                Effect::Set(x.clone(), y.clone())
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
            Jmp | Call => Effect::Jmp(instr_name, vec![instr.next_ip32()]),
            Jne | Jge => {
                let [addr] = args.try_into().unwrap();
                let Expr::Const(addr) = addr else { todo!() };
                Effect::Jmp(instr_name, vec![instr.next_ip32(), addr])
            }
            Ret => Effect::Jmp(instr_name, vec![]),
            Cmp | Test => {
                let op = format!("{:?}", instr.mnemonic()).to_ascii_lowercase();
                Effect::Call(Box::new(crate::Call { op, args }))
            }
            _ => {
                let op = format!("todo:{:?}", instr.mnemonic()).to_ascii_lowercase();
                Effect::Call(Box::new(crate::Call { op, args }))
            }
        };

        instrs.push(Instr {
            ip: instr.ip32(),
            iced: instr,
            eff,
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
    id: usize,
    instrs: Vec<Instr>,
    params: Vec<String>,
    links: Vec<Link>,
}

impl Block {
    fn addr(&self) -> u32 {
        self.instrs[0].ip
    }
}

struct Blocks {
    vec: Vec<Block>,
}
impl Blocks {
    fn get(&self, id: usize) -> &Block {
        &self.vec[id]
    }
}

fn blocks(instrs: Vec<Instr>) -> (Blocks, Vec<Vec<u32>>) {
    let mut blocks = vec![];
    let mut block = vec![];
    let mut nexts = vec![];
    for instr in instrs {
        block.push(instr);
        let instr = block.last().unwrap();
        if let Effect::Jmp(_, next) = &instr.eff {
            nexts.push(next.clone());
            blocks.push(Block {
                id: blocks.len(),
                instrs: block,
                params: vec![],
                links: vec![],
            });
            block = vec![];
        }
    }
    if !block.is_empty() {
        println!("{:#?}", block);
        assert!(block.is_empty());
    }
    (Blocks { vec: blocks }, nexts)
}

fn visit_expr(expr: &mut Expr, f: &mut impl FnMut(&mut Expr) -> bool) {
    if f(expr) {
        if let Expr::Call(call) = expr {
            for arg in call.args.iter_mut() {
                visit_expr(arg, f);
            }
        }
    }
}

fn visit_effect(effect: &mut Effect, f: &mut impl FnMut(&mut Expr) -> bool) {
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
        Effect::Jmp(_, _) => {}
    }
}

fn visit_block(block: &mut Block, f: &mut impl FnMut(&mut Expr) -> bool) {
    for instr in block.instrs.iter_mut() {
        visit_effect(&mut instr.eff, f);
    }
}

fn rename(eff: &mut Effect, from: &Var, to: &Var) {
    visit_effect(eff, &mut |expr| match expr {
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
        let eff = &mut instr.eff;
        match eff {
            Effect::Set(expr, _) => {
                let Expr::Var(var) = expr else {
                    continue;
                };
                let new_local = var.next();
                for instr in rest {
                    rename(&mut instr.eff, &var, &new_local);
                }
                *expr = Expr::Var(new_local);
            }
            _ => {}
        }
    }
}

fn params(block: &mut Block) {
    let mut params = HashSet::new();
    for instr in &mut block.instrs {
        if let Effect::Call(call) = &instr.eff
            && call.op.starts_with("todo:")
        {
            continue;
        }

        visit_effect(&mut instr.eff, &mut |expr| match expr {
            Expr::Var(var) if var.ver == 0 => {
                // XXX this only should be for reads, not writes
                params.insert(var.reg.clone());
                true
            }
            _ => true,
        });
    }

    let mut params = params.into_iter().collect::<Vec<_>>();
    params.sort();
    block.params = params;
}

fn links(blocks: &mut Blocks, nexts: Vec<Vec<u32>>) {
    let nexts = nexts
        .into_iter()
        .map(|addrs| {
            addrs
                .into_iter()
                .flat_map(|addr| blocks.vec.iter().position(|b| b.addr() == addr))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut bouts: Vec<HashMap<String, Var>> = Default::default();
    let mut bins: Vec<HashSet<String>> = Default::default();

    for block in blocks.vec.iter_mut() {
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
        bouts.push(outs);
        bins.push(HashSet::from_iter(block.params.iter().map(|p| p.clone())));
    }

    let mut changed = true;
    while changed {
        changed = false;
        for src in blocks.vec.iter() {
            let outs = &bouts[src.id];
            let mut add = vec![];
            for &next_id in &nexts[src.id] {
                for param in &bins[next_id] {
                    if !outs.contains_key(param.as_str()) {
                        add.push(param.clone());
                    }
                }
            }

            if !add.is_empty() {
                for add in add {
                    bins[src.id].insert(add.clone());
                    bouts[src.id].insert(add.clone(), Var::new(add));
                }
                changed = true;
            }
        }
    }

    for id in 0..blocks.vec.len() {
        let mut params = bins[id].iter().map(|s| s.clone()).collect::<Vec<_>>();
        params.sort();

        let next = nexts[id]
            .iter()
            .map(|&next_id| {
                let params = bins[next_id]
                    .iter()
                    .map(|p| (p.clone(), bouts[id].get(p).unwrap().clone()))
                    .collect();
                Link {
                    addr: blocks.vec[next_id].addr(),
                    params,
                }
            })
            .collect();

        blocks.vec[id].params = params;
        blocks.vec[id].links = next;
    }
}

fn main() {
    let instrs = decode();
    let (mut blocks, nexts) = blocks(instrs);
    blocks.vec.truncate(3);
    links(&mut blocks, nexts);
    for block in blocks.vec {
        println!(
            "{ip:x} [{params}]",
            ip = block.instrs[0].ip,
            params = block.params.join(" ")
        );
        for instr in &block.instrs {
            let text = format!("{}", instr.eff);
            println!(
                "{text:40}  ; {ip:x} {iced}",
                ip = instr.ip,
                iced = instr.iced
            );
        }
        for link in block.links {
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
