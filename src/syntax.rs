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
    MakeVec(Box<Expr>, Box<Expr>),
    Vec(Vec<Expr>),
    VecSet(Box<Expr>, Box<Expr>, Box<Expr>),
    VecGet(Box<Expr>, Box<Expr>),
    VecLen(Box<Expr>),
    Block(Vec<Expr>),
    Call(Symbol, Vec<Expr>),
    Input,
    Nil,
    PrintStack,
    Gc,
}

#[derive(Debug, Copy, Clone)]
pub enum Op1 {
    Add1,
    Sub1,
    IsNum,
    IsBool,
    IsVec,
    Print,
}

#[derive(Debug, Copy, Clone)]
pub enum Op2 {
    Plus,
    Minus,
    Times,
    Divide,
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

    pub fn replace(&self, from: &str, to: &str) -> String {
        self.0.replace(from, to)
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
