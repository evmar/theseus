use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, ts_rs::TS)]
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
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, ts_rs::TS)]
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
    ($x:expr, $($arg:expr),*) => { crate::Expr::Call(Box::new(crate::Call { op: $x.into(), args: vec![$($arg.into()),*] })) };
}

// macro_rules! expr {
//     ($x:expr) => { Expr::Const($x) };
//     ($x:expr, $($arg:expr) +) => { Expr::Call(Box::new(Call { op: $x, args: vec![$($arg),*] })) };
// }

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, ts_rs::TS)]
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

#[derive(Debug, serde::Serialize, ts_rs::TS)]
struct Jmp {
    cond: Call,
    dsts: Vec<Expr>,
}

impl Jmp {
    fn new(cond: impl Into<String>, dsts: Vec<Expr>) -> Self {
        Jmp {
            cond: Call {
                op: cond.into(),
                args: vec![],
            },
            dsts,
        }
    }
}

#[derive(Debug, serde::Serialize, ts_rs::TS)]
enum Effect {
    Set(Expr, Expr),
    Call(Box<Call>),
    Jmp(Box<Jmp>),
}

impl std::fmt::Display for Effect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Effect::Set(x, y) => write!(f, "{x} = {y}"),
            Effect::Call(call) => write!(f, "{call}"),
            Effect::Jmp(jmp) => write!(
                f,
                "{cond} {next}",
                cond = jmp.cond,
                next = jmp
                    .dsts
                    .iter()
                    .map(|x| format!("{x}"))
                    .collect::<Vec<_>>()
                    .join(" ")
            ),
        }
    }
}

#[derive(ts_rs::TS)]
#[allow(unused)]
struct InstrJS {
    addr: u32,
    iced: String,
    eff: Effect,
}

#[derive(Debug, ts_rs::TS)]
#[ts(as = "InstrJS")]
struct Instr {
    iced: iced_x86::Instruction,
    eff: Effect,
}

impl serde::Serialize for Instr {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("Instr", 2)?;
        state.serialize_field("addr", &self.iced.ip32())?;
        state.serialize_field("iced", &format!("{}", self.iced))?;
        state.serialize_field("eff", &self.eff)?;
        state.end()
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

#[derive(Debug, serde::Serialize, ts_rs::TS)]
struct Link {
    addr: u32,
    params: Vec<(Var, Var)>,
}

#[derive(serde::Serialize, ts_rs::TS)]
struct Block {
    id: usize,
    instrs: Vec<Instr>,
    params: VarSet,
    links: Vec<Link>,
}

impl Block {
    fn addr(&self) -> u32 {
        self.instrs[0].iced.ip32()
    }
}

#[derive(serde::Serialize, ts_rs::TS)]
#[ts(export)]
struct Blocks {
    vec: Vec<Block>,
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

#[derive(Clone, Default, Debug, serde::Serialize, ts_rs::TS)]
struct VarSet(Vec<Var>);
impl VarSet {
    fn get(&mut self, reg: &str) -> Option<&mut Var> {
        let i = self.0.iter().position(|v| v.reg == reg)?;
        Some(&mut self.0[i])
    }

    fn insert(&mut self, var: Var) {
        if let Some(prev) = self.get(&var.reg) {
            prev.ver = prev.ver.max(var.ver);
            return;
        }
        self.0.push(var.clone());
    }

    fn iter(&self) -> impl Iterator<Item = &Var> {
        self.0.iter()
    }

    fn new_var(&mut self, base: &Var) -> Var {
        match self.0.iter_mut().find(|v| v.reg == base.reg) {
            Some(prev) => {
                prev.ver = prev.ver.max(base.ver) + 1;
                prev.clone()
            }
            None => {
                let new = Var {
                    reg: base.reg.clone(),
                    ver: 1,
                };
                self.0.push(new.clone());
                new
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

    if args.json {
        std::fs::write("web/data.json", serde_json::to_string(&blocks).unwrap()).unwrap();
    } else {
        print(&blocks);
    }
}

const IP: u32 = 0x401d0f;
const ASM: &'static [u8] = b"\x53\x51\x52\x56\x2e\xff\x15\x58\xb1\x40\x00\x8b\x15\x7c\x3f\x43\x00\x31\xdb\x29\xd0\x31\xf6\xa3\x84\x3f\x43\x00\x31\xd2\xeb\x18\xc7\x80\xac\x39\x43\x00\x00\x00\x00\x00\x42\x83\xfa\x28\x0f\x8d\xa3\x00\x00\x00\x85\xf6\x75\x44\x6b\xc2\x0c\x83\xb8\x28\x3c\x43\x00\x01\x75\x38\x8b\x88\x24\x3c\x43\x00\x81\xc1\xfa\x00\x00\x00\x3b\x0d\x84\x3f\x43\x00\x7d\x24\xc7\x80\x28\x3c\x43\x00\x00\x00\x00\x00\xc7\x05\x53\xc7\x40\x00\x01\x00\x00\x00\x8b\x80\x20\x3c\x43\x00\xbe\x01\x00\x00\x00\xa3\x57\xc7\x40\x00\x89\xd0\xc1\xe0\x04\x83\xb8\xac\x39\x43\x00\x01\x75\xa0\x8b\x88\xa8\x39\x43\x00\x81\xc1\xfa\x00\x00\x00\x3b\x0d\x84\x3f\x43\x00\x7d\x8c\x3b\x98\xa8\x39\x43\x00\x0f\x8d\x76\xff\xff\xff\x8b\x88\xa0\x39\x43\x00\x8b\x98\xa8\x39\x43\x00\x89\x0d\x3f\xc7\x40\x00\x8b\x88\xa4\x39\x43\x00\xc7\x80\xac\x39\x43\x00\x00\x00\x00\x00\x89\x0d\x43\xc7\x40\x00\xe9\x53\xff\xff\xff\x5e\x5a\x59\x5b\xc3";
