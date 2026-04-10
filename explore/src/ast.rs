#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, ts_rs::TS)]
pub struct Var {
    pub reg: String,
    pub ver: usize,
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
    pub fn new(reg: String) -> Self {
        Var { reg, ver: 0 }
    }
}

#[derive(Clone, Default, Debug, serde::Serialize, ts_rs::TS)]
pub struct VarSet(Vec<Var>);

impl std::fmt::Display for VarSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for (i, var) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, " ")?;
            }
            write!(f, "{}", var)?;
        }
        write!(f, "]")
    }
}

impl VarSet {
    pub fn get(&mut self, reg: &str) -> Option<&mut Var> {
        let i = self.0.iter().position(|v| v.reg == reg)?;
        Some(&mut self.0[i])
    }

    pub fn insert(&mut self, var: Var) {
        if let Some(prev) = self.get(&var.reg) {
            prev.ver = prev.ver.max(var.ver);
            return;
        }
        self.0.push(var.clone());
    }

    pub fn iter(&self) -> impl Iterator<Item = &Var> {
        self.0.iter()
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Var> {
        self.0.iter_mut()
    }

    pub fn new_var(&mut self, base: &Var) -> Var {
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

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, ts_rs::TS)]
pub enum Expr {
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
    pub fn from_reg(r: iced_x86::Register) -> Expr {
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

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, ts_rs::TS)]
pub struct Call {
    pub op: String,
    pub args: Vec<Expr>,
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
pub struct Jmp {
    pub cond: Call,
    pub dsts: Vec<Expr>,
}

impl Jmp {
    pub fn new(cond: impl Into<String>, dsts: Vec<Expr>) -> Self {
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
pub enum Effect {
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
pub struct Instr {
    pub iced: iced_x86::Instruction,
    pub eff: Effect,
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

#[derive(Debug, serde::Serialize, ts_rs::TS)]
pub struct Link {
    pub addr: u32,
    // key=val pairs
    pub params: Vec<(Var, Var)>,
}

#[derive(serde::Serialize, ts_rs::TS)]
pub struct Block {
    pub id: usize,
    pub addr: u32,
    pub instrs: Vec<Instr>,
    pub params: VarSet,
    pub links: Vec<Link>,
}

#[derive(serde::Serialize, ts_rs::TS)]
#[ts(export)]
pub struct Blocks {
    pub vec: Vec<Block>,
}

pub fn visit_expr(expr: &mut Expr, f: &mut impl FnMut(&mut Expr)) {
    f(expr);
    if let Expr::Call(call) = expr {
        for arg in call.args.iter_mut() {
            visit_expr(arg, f);
        }
    }
}

pub fn visit_effect(effect: &mut Effect, f: &mut impl FnMut(&mut Expr)) {
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
pub fn visit_block(block: &mut Block, f: &mut impl FnMut(&mut Expr)) {
    for instr in block.instrs.iter_mut() {
        visit_effect(&mut instr.eff, f);
    }
}
