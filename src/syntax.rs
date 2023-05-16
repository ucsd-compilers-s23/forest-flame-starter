use std::fmt;

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct Symbol(&'static str);

#[derive(Debug)]
pub struct Prog {
    pub funs: Vec<FunDecl>,
    pub main: Expr,
}

#[derive(Debug)]
pub struct FunDecl {
    pub name: Symbol,
    pub params: Vec<Symbol>,
    pub body: Expr,
}

#[derive(Debug)]
pub enum Expr {
    Number(i64),
    Boolean(bool),
    Var(Symbol),
    Let(Vec<(Symbol, Expr)>, Box<Expr>),
    UnOp(Op1, Box<Expr>),
    BinOp(Op2, Box<Expr>, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Loop(Box<Expr>),
    Break(Box<Expr>),
    Set(Symbol, Box<Expr>),
    VecNew(Box<Expr>, Box<Expr>),
    VecSet(Box<Expr>, Box<Expr>, Box<Expr>),
    Block(Vec<Expr>),
    Call(Symbol, Vec<Expr>),
    Input,
}

#[derive(Debug, Copy, Clone)]
pub enum Op1 {
    Add1,
    Sub1,
    IsNum,
    IsBool,
    Print,
}

#[derive(Debug, Copy, Clone)]
pub enum Op2 {
    Plus,
    Minus,
    Times,
    Equal,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}

impl Symbol {
    pub fn new(s: impl ToString) -> Symbol {
        Symbol(Box::leak(s.to_string().into_boxed_str()))
    }
}

impl Expr {
    pub fn depth(&self) -> u32 {
        match self {
            Expr::BinOp(_, e1, e2) => u32::max(e1.depth(), e2.depth() + 1),
            Expr::Let(bindings, e) => {
                let max = bindings
                    .iter()
                    .map(|(_, e)| e.depth())
                    .max()
                    .unwrap_or_default();
                u32::max(max, e.depth() + bindings.len() as u32)
            }
            Expr::If(e1, e2, e3) => e1.depth().max(e2.depth()).max(e3.depth()),
            Expr::Call(_, es) | Expr::Block(es) => es.iter().map(Expr::depth).max().unwrap_or(0),
            Expr::Input | Expr::Var(_) | Expr::Number(_) | Expr::Boolean(_) => 0,
            Expr::UnOp(_, e) | Expr::Loop(e) | Expr::Break(e) | Expr::Set(_, e) => e.depth(),
            Expr::VecNew(size, elem) => u32::max(size.depth(), elem.depth() + 1),
            Expr::VecSet(vec, idx, val) => vec.depth().max(idx.depth() + 1).max(val.depth() + 2),
        }
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Debug for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
