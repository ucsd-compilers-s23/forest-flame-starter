use regex::Regex;
use sexp::{Atom::*, Sexp};

use crate::syntax::{Expr, FunDecl, Op1, Op2, Prog, Symbol};

pub fn parse(s: &str) -> Prog {
    let s = format!("({})", s);
    let s = sexp::parse(&s).unwrap_or_else(|_| syntax_error("invalid s-expr"));
    Parser::new().parse_prog(&s)
}

struct Parser {
    id_regex: Regex,
}

impl Parser {
    fn new() -> Parser {
        Parser {
            id_regex: Regex::new(r"^[a-zA-Z][a-zA-Z0-9_-]*$").unwrap(),
        }
    }

    fn parse_prog(&self, e: &Sexp) -> Prog {
        let Sexp::List(es) = e else{
            syntax_error("expected a list")
        };
        if let [funcs @ .., main] = &es[..] {
            let funcs = funcs.iter().map(|e| self.parse_func(e)).collect();
            let main = self.parse_expr(main);
            Prog { funs: funcs, main }
        } else {
            syntax_error("program must contain a main expression")
        }
    }

    fn parse_expr(&self, e: &Sexp) -> Expr {
        match e {
            &Sexp::Atom(I(n)) => {
                if n >= -4611686018427387904 && n < 4611686018427387904 {
                    Expr::Number(n)
                } else {
                    syntax_error("integer literal overflow")
                }
            }
            Sexp::Atom(S(id)) => match id.as_str() {
                "true" => Expr::Boolean(true),
                "false" => Expr::Boolean(false),
                "input" => Expr::Input,
                "nil" => Expr::Nil,
                _ => {
                    if is_keyword(id) {
                        syntax_error("invalid use of keyword `{id}`")
                    } else {
                        Expr::Var(Symbol::new(id))
                    }
                }
            },
            Sexp::List(vec) => match &vec[..] {
                // (snek-printstack)
                [Sexp::Atom(S(keyword)), es @ ..] if keyword == "snek-printstack" => {
                    if !es.is_empty() {
                        return syntax_error("snek-prinstack doesn't take any arguments");
                    }
                    Expr::PrintStack
                }
                // (gc)
                [Sexp::Atom(S(keyword)), es @ ..] if keyword == "gc" => {
                    if !es.is_empty() {
                        return syntax_error("gc doesn't take any arguments");
                    }
                    Expr::Gc
                }
                // (make-vec size elem)
                [Sexp::Atom(S(keyword)), es @ ..] if keyword == "make-vec" => {
                    let [size, elem] = &es[..] else {
                        return syntax_error("malformed vec");
                    };
                    let size = self.parse_expr(size);
                    let elem = self.parse_expr(elem);
                    Expr::MakeVec(Box::new(size), Box::new(elem))
                }
                // (vec elem*)
                [Sexp::Atom(S(keyword)), es @ ..] if keyword == "vec" => {
                    Expr::Vec(es.iter().map(|e| self.parse_expr(e)).collect())
                }
                // (vec-set! idx elem)
                [Sexp::Atom(S(keyword)), es @ ..] if keyword == "vec-set!" => {
                    let [vec, size, elem] = &es[..] else {
                        return syntax_error("malformed vec-set!");
                    };
                    let vec = self.parse_expr(vec);
                    let idx = self.parse_expr(size);
                    let elem = self.parse_expr(elem);
                    Expr::VecSet(Box::new(vec), Box::new(idx), Box::new(elem))
                }
                // (vec-get idx elem)
                [Sexp::Atom(S(keyword)), es @ ..] if keyword == "vec-get" => {
                    let [vec, idx] = &es[..] else {
                        return syntax_error("malformed vec-get");
                    };
                    let vec = self.parse_expr(vec);
                    let idx = self.parse_expr(idx);
                    Expr::VecGet(Box::new(vec), Box::new(idx))
                }
                // (vec-len vec)
                [Sexp::Atom(S(keyword)), es @ ..] if keyword == "vec-len" => {
                    let [vec] = &es[..] else {
                        return syntax_error("malformed vec-len");
                    };
                    let vec = self.parse_expr(vec);
                    Expr::VecLen(Box::new(vec))
                }
                // Block
                [Sexp::Atom(S(keyword)), es @ ..] if keyword == "block" => {
                    let es: Vec<_> = es.iter().map(|e| self.parse_expr(e)).collect();
                    if !es.is_empty() {
                        Expr::Block(es)
                    } else {
                        syntax_error("blocks must contain at least one expression")
                    }
                }

                // (let <bindings> <expr>)
                [Sexp::Atom(S(keyword)), es @ ..] if keyword == "let" => {
                    let [e1, e2] = &es[..] else {
                        return syntax_error("malformed let");
                    };
                    match e1 {
                        Sexp::List(bindings) => {
                            if bindings.is_empty() {
                                return syntax_error("empty bindings");
                            }
                            let bindings: Vec<_> =
                                bindings.iter().map(|e| self.parse_binding(e)).collect();
                            let body = self.parse_expr(e2);
                            Expr::Let(bindings, Box::new(body))
                        }
                        _ => syntax_error("invalid let expr"),
                    }
                }

                // set! <name> <expr> => Set
                [Sexp::Atom(S(keyword)), Sexp::Atom(S(id)), e] if keyword == "set!" => {
                    let e = self.parse_expr(e);
                    Expr::Set(Symbol::new(id), Box::new(e))
                }

                // if <expr> <expr> <expr> => If
                [Sexp::Atom(S(keyword)), es @ ..] if keyword == "if" => {
                    let [e1, e2, e3] = &es[..] else {
                        syntax_error("malformed if")
                    };
                    let e1 = self.parse_expr(e1);
                    let e2 = self.parse_expr(e2);
                    let e3 = self.parse_expr(e3);

                    Expr::If(Box::new(e1), Box::new(e2), Box::new(e3))
                }

                [Sexp::Atom(S(keyword)), es @ ..]
                    if matches!(
                        &keyword[..],
                        "loop" | "break" | "add1" | "sub1" | "isnum" | "isbool" | "isvec" | "print"
                    ) =>
                {
                    let [e] = es else {
                        return syntax_error("expected a single expression after keyword");
                    };
                    let e_expr = self.parse_expr(e);

                    match keyword.as_str() {
                        "loop" => Expr::Loop(Box::new(e_expr)),
                        "break" => Expr::Break(Box::new(e_expr)),
                        "print" => Expr::UnOp(Op1::Print, Box::new(e_expr)),
                        "add1" => Expr::UnOp(Op1::Add1, Box::new(e_expr)),
                        "sub1" => Expr::UnOp(Op1::Sub1, Box::new(e_expr)),
                        "isnum" => Expr::UnOp(Op1::IsNum, Box::new(e_expr)),
                        "isbool" => Expr::UnOp(Op1::IsBool, Box::new(e_expr)),
                        "isvec" => Expr::UnOp(Op1::IsVec, Box::new(e_expr)),
                        _ => unreachable!(),
                    }
                }

                [Sexp::Atom(S(op)), es @ ..]
                    if matches!(
                        op.as_str(),
                        "+" | "-" | "*" | "/" | ">" | "<" | ">=" | "<=" | "="
                    ) =>
                {
                    let [e1, e2] = es else {
                        return syntax_error("expected two expressions after operator");
                    };
                    let expr_op = match op.as_str() {
                        "+" => Op2::Plus,
                        "-" => Op2::Minus,
                        "*" => Op2::Times,
                        "/" => Op2::Divide,
                        ">" => Op2::Greater,
                        "<" => Op2::Less,
                        ">=" => Op2::GreaterEqual,
                        "<=" => Op2::LessEqual,
                        "=" => Op2::Equal,
                        _ => unreachable!(),
                    };

                    let e1_instrs = self.parse_expr(e1);
                    let e2_instrs = self.parse_expr(e2);

                    Expr::BinOp(expr_op, Box::new(e1_instrs), Box::new(e2_instrs))
                }

                [func, args @ ..] => {
                    let func = self.parse_identifier(func);
                    let exprs: Vec<_> = args.iter().map(|e| self.parse_expr(e)).collect();
                    Expr::Call(Symbol::new(func), exprs)
                }
                _ => syntax_error("unexpected s-expr"),
            },

            _ => syntax_error("unexpected s-expr"),
        }
    }

    fn parse_binding(&self, e: &Sexp) -> (Symbol, Expr) {
        let Sexp::List(es) = e else {
            return syntax_error("expected a list")
        };
        if let [name, expr] = &es[..] {
            (self.parse_identifier(name), self.parse_expr(expr))
        } else {
            syntax_error("malformed binding")
        }
    }

    fn parse_func(&self, e: &Sexp) -> FunDecl {
        let Sexp::List(es) = e else {
            return syntax_error("expected a list");
        };
        match &es[..] {
            [Sexp::Atom(S(keyword)), Sexp::List(es), body] if keyword == "fun" => {
                let [name, params @ ..] = &es[..] else {
                    return syntax_error("missing function name");
                };
                let params = params.iter().map(|e| self.parse_identifier(e)).collect();
                let body = self.parse_expr(body);
                let name = self.parse_identifier(name);
                FunDecl {
                    name: Symbol::new(name),
                    params,
                    body,
                }
            }
            _ => syntax_error("malformed function"),
        }
    }

    fn parse_identifier(&self, e: &Sexp) -> Symbol {
        let Sexp::Atom(S(s)) = e  else {
            return syntax_error("expected an identifier");
        };

        if is_keyword(s) {
            syntax_error(format!("cannot use keyword `{s}` as identifier"))
        } else if self.id_regex.is_match(s) {
            Symbol::new(s)
        } else {
            syntax_error("invalid identifier")
        }
    }
}

fn is_keyword(s: &str) -> bool {
    matches!(
        s,
        "loop"
            | "true"
            | "false"
            | "break"
            | "add1"
            | "sub1"
            | "isnum"
            | "isbool"
            | "isvec"
            | "print"
            | "let"
            | "set!"
            | "input"
            | "nil"
            | "fun"
            | "make-vec"
            | "vec"
            | "vec-set!"
            | "vec-get"
            | "vec-len"
            | "snek-printstack"
            | "gc"
    )
}

fn syntax_error<T>(note: impl ToString) -> T {
    panic!("Invalid syntax: {}", note.to_string())
}
