use std::collections::{HashMap, HashSet};

use crate::{
    asm::{
        instrs_to_string, Arg32, Arg64, BinArgs, CMov, Instr, Loc, MemRef, MovArgs, Offset,
        Reg::{self, *},
        Reg32,
    },
    syntax::{Expr, FunDecl, Op1, Op2, Prog, Symbol},
};

struct Session {
    tag: u32,
    instrs: Vec<Instr>,
    funs: HashMap<Symbol, usize>,
}

const INVALID_ARG_LBL: &str = "invalid_argument";
const OVERFLOW_LBL: &str = "overflow";
const INDEX_OUT_OF_BOUNDS: &str = "index_out_of_bounds";

const STACK_BASE_REG: Reg = Rbx;
const INPUT_REG: Reg = R12;

const MEM_SET_VAL: i32 = 0b001;

#[derive(Debug, Clone)]
struct Ctxt<'a> {
    env: im::HashMap<Symbol, MemRef>,
    si: u32,
    curr_lbl: Option<&'a str>,
    in_fun: bool,
}

impl<'a> Ctxt<'a> {
    fn new() -> Ctxt<'a> {
        Ctxt {
            si: 0,
            curr_lbl: None,
            env: im::HashMap::default(),
            in_fun: false,
        }
    }

    fn fun(params: &[Symbol]) -> Ctxt<'a> {
        let env = params
            .iter()
            .enumerate()
            .map(|(i, param)| {
                let mem = MemRef {
                    reg: Rbp,
                    offset: Offset::Constant(8 * (i as i32 + 2)),
                };
                (*param, mem)
            })
            .collect();
        Ctxt {
            si: 0,
            curr_lbl: None,
            env,
            in_fun: true,
        }
    }

    fn lookup(&self, x: Symbol) -> MemRef {
        *self
            .env
            .get(&x)
            .unwrap_or_else(|| raise_unbound_identifier(x))
    }

    fn set_curr_lbl(&self, lbl: &'a str) -> Ctxt<'a> {
        Ctxt {
            curr_lbl: Some(lbl),
            ..self.clone()
        }
    }

    fn next_local(&self) -> (Ctxt<'a>, MemRef) {
        let si: i32 = (self.si + 1).try_into().unwrap();
        (
            Ctxt {
                si: self.si + 1,
                ..self.clone()
            },
            MemRef {
                reg: Rbp,
                offset: Offset::Constant(-(8 * si)),
            },
        )
    }

    fn add_binding(&self, x: Symbol, memref: MemRef) -> Ctxt<'a> {
        Ctxt {
            env: self.env.update(x, memref),
            ..*self
        }
    }
}

pub fn compile(prg: &Prog) -> String {
    match fun_arity_map(prg) {
        Ok(funs) => {
            let mut sess = Session::new(funs);
            let locals = prg.main.depth();
            sess.compile_funs(&prg.funs);
            sess.append_instr(Instr::Label("our_code_starts_here".to_string()));
            sess.fun_entry(locals, &[STACK_BASE_REG, INPUT_REG]);
            sess.append_instrs([
                Instr::Mov(MovArgs::ToReg(INPUT_REG, Arg64::Reg(Rdi))),
                Instr::Mov(MovArgs::ToReg(STACK_BASE_REG, Arg64::Reg(Rbp))),
            ]);
            sess.compile_expr(&Ctxt::new(), Loc::Reg(Rax), &prg.main);
            sess.fun_exit(locals, &[STACK_BASE_REG, INPUT_REG]);

            format!(
                "
section .text
extern snek_error
extern snek_print
extern snek_alloc_vec
extern snek_print_stack
global our_code_starts_here
{}
{INVALID_ARG_LBL}:
  mov edi, 1
  call snek_error
{OVERFLOW_LBL}:
  mov edi, 2
  call snek_error
{INDEX_OUT_OF_BOUNDS}:
  mov edi, 3
  call snek_error
",
                instrs_to_string(&sess.instrs)
            )
        }
        Err(dup) => raise_duplicate_function(dup),
    }
}

impl Session {
    fn new(funs: HashMap<Symbol, usize>) -> Session {
        Session {
            tag: 0,
            instrs: vec![],
            funs,
        }
    }

    fn fun_entry(&mut self, locals: u32, callee_saved: &[Reg]) {
        let size = stack_size(locals, callee_saved);
        for reg in callee_saved {
            self.append_instr(Instr::Push(Arg32::Reg(*reg)));
        }
        self.append_instrs([
            Instr::Push(Arg32::Reg(Rbp)),
            Instr::Mov(MovArgs::ToReg(Rbp, Arg64::Reg(Rsp))),
            Instr::Sub(BinArgs::ToReg(Rsp, Arg32::Imm(8 * (size as i32)))),
        ]);
        self.memset(0, size, Reg32::Imm(MEM_SET_VAL));
    }

    fn fun_exit(&mut self, locals: u32, calle_saved: &[Reg]) {
        let size = stack_size(locals, calle_saved);
        for reg in calle_saved.iter().rev() {
            self.append_instr(Instr::Pop(Loc::Reg(*reg)));
        }
        self.append_instrs([
            Instr::Add(BinArgs::ToReg(Rsp, Arg32::Imm(8 * (size as i32)))),
            Instr::Pop(Loc::Reg(Rbp)),
            Instr::Ret,
        ]);
    }

    fn memset(&mut self, start: u32, count: u32, elem: Reg32) {
        for i in start..start + count {
            let mem = MemRef {
                reg: Rbp,
                offset: Offset::Constant(-(8 * (i as i32 + 1))),
            };
            self.append_instr(Instr::Mov(MovArgs::ToMem(mem, elem)));
        }
    }

    fn compile_funs(&mut self, funs: &[FunDecl]) {
        for fun in funs {
            self.compile_fun(fun)
        }
    }

    fn compile_fun(&mut self, fun: &FunDecl) {
        check_dup_bindings(&fun.params);
        let locals = fun.body.depth();
        self.append_instr(Instr::Label(format!("fun_start_{}", fun.name)));
        self.fun_entry(locals, &[]);
        self.compile_expr(&Ctxt::fun(&fun.params), Loc::Reg(Rax), &fun.body);
        self.fun_exit(locals, &[]);
    }

    fn compile_expr(&mut self, cx: &Ctxt, dst: Loc, e: &Expr) {
        match e {
            Expr::Number(n) => self.move_to(dst, n.repr64()),
            Expr::Boolean(b) => self.move_to(dst, b.repr64()),
            Expr::Var(x) => self.move_to(dst, Arg32::Mem(cx.lookup(*x))),
            Expr::Let(bindings, body) => {
                check_dup_bindings(bindings.iter().map(|(id, _)| id));
                let mut currcx = cx.clone();
                for (var, rhs) in bindings {
                    let (nextcx, mem) = currcx.next_local();
                    self.compile_expr(&currcx, Loc::Mem(mem), rhs);
                    currcx = nextcx.add_binding(*var, mem);
                }
                self.compile_expr(&currcx, Loc::Reg(Rax), body);
                self.memset(cx.si, bindings.len() as u32, Reg32::Imm(MEM_SET_VAL));
                self.move_to(dst, Arg64::Reg(Rax))
            }
            Expr::UnOp(op, e) => self.compile_un_op(cx, dst, *op, e),
            Expr::BinOp(op, e1, e2) => self.compile_bin_op(cx, dst, *op, e1, e2),
            Expr::If(e1, e2, e3) => {
                let tag = self.next_tag();
                let else_lbl = format!("if_else_{tag}");
                let end_lbl = format!("if_end_{tag}");

                self.compile_expr(cx, Loc::Reg(Rax), e1);
                self.append_instrs([
                    Instr::Cmp(BinArgs::ToReg(Rax, false.repr32().into())),
                    Instr::Je(else_lbl.clone()),
                ]);
                self.compile_expr(cx, dst, e2);
                self.append_instrs([Instr::Jmp(end_lbl.clone()), Instr::Label(else_lbl)]);
                self.compile_expr(cx, dst, e3);
                self.append_instr(Instr::Label(end_lbl))
            }
            Expr::Loop(e) => {
                let tag = self.next_tag();
                let loop_start_lbl = format!("loop_start_{tag}");
                let loop_end_lbl = format!("loop_end_{tag}");

                self.append_instr(Instr::Label(loop_start_lbl.clone()));
                self.compile_expr(&cx.set_curr_lbl(&loop_end_lbl), dst, e);
                self.append_instrs([Instr::Jmp(loop_start_lbl), Instr::Label(loop_end_lbl)])
            }
            Expr::Break(e) => {
                if let Some(lbl) = cx.curr_lbl {
                    self.compile_expr(cx, dst, e);
                    self.append_instr(Instr::Jmp(lbl.to_string()));
                } else {
                    raise_break_outside_loop()
                }
            }
            Expr::Set(var, e) => {
                let mem = cx.lookup(*var);
                self.compile_expr(cx, Loc::Mem(mem), e);
                self.move_to(dst, Arg32::Mem(mem));
            }
            Expr::Block(es) => {
                for e in &es[..es.len() - 1] {
                    self.compile_expr(cx, Loc::Reg(Rcx), e);
                }
                self.compile_expr(cx, dst, &es[es.len() - 1]);
            }
            Expr::Call(fun, args) => {
                let Some(arity) = self.funs.get(fun) else {
                    return raise_undefined_fun(*fun);
                };
                if args.len() != *arity {
                    raise_worng_number_of_args(*arity, args.len());
                }

                let mut nargs = args.len() as i32;
                if nargs % 2 == 0 {
                    self.append_instr(Instr::Sub(BinArgs::ToReg(Rsp, Arg32::Imm(8 * nargs))));
                } else {
                    let mem = MemRef {
                        reg: Rsp,
                        offset: Offset::Constant(8 * nargs),
                    };
                    nargs += 1;
                    self.append_instrs([
                        Instr::Sub(BinArgs::ToReg(Rsp, Arg32::Imm(8 * nargs))),
                        Instr::Mov(MovArgs::ToMem(mem, Reg32::Imm(MEM_SET_VAL))),
                    ]);
                }

                for (i, arg) in args.iter().enumerate() {
                    let mem = MemRef {
                        reg: Rsp,
                        offset: Offset::Constant(8 * (i as i32)),
                    };
                    self.compile_expr(cx, Loc::Mem(mem), arg);
                }
                self.append_instrs([
                    Instr::Call(format!("fun_start_{fun}")),
                    Instr::Add(BinArgs::ToReg(Rsp, Arg32::Imm(8 * nargs))),
                ]);
                self.move_to(dst, Arg64::Reg(Rax));
            }
            Expr::Input => {
                if cx.in_fun {
                    raise_input_in_fun()
                } else {
                    self.move_to(dst, Arg32::Reg(INPUT_REG))
                }
            }
            Expr::VecNew(size, elem) => {
                let (nextcx, size_mem) = cx.next_local();
                self.compile_expr(cx, Loc::Mem(size_mem), size);
                self.compile_expr(&nextcx, Loc::Reg(Rsi), elem);
                self.append_instr(Instr::Mov(MovArgs::ToReg(Rdi, Arg64::Mem(size_mem))));
                self.memset(cx.si, 1, Reg32::Imm(MEM_SET_VAL));
                self.append_instrs([
                    Instr::Mov(MovArgs::ToReg(Rdx, Arg64::Reg(STACK_BASE_REG))),
                    Instr::Mov(MovArgs::ToReg(Rcx, Arg64::Reg(Rsp))),
                    Instr::Call("snek_alloc_vec".to_string()),
                ]);
                self.move_to(dst, Arg64::Reg(Rax));
            }
            Expr::VecSet(vec, idx, elem) => {
                let (nextcx1, vec_mem) = cx.next_local();
                let (nextcx2, idx_mem) = nextcx1.next_local();

                self.compile_expr(cx, Loc::Mem(vec_mem), vec);
                self.compile_expr(&nextcx1, Loc::Mem(idx_mem), idx);
                self.compile_expr(&nextcx2, Loc::Reg(Rax), elem);

                self.append_instrs([
                    Instr::Mov(MovArgs::ToReg(Rdx, Arg64::Mem(vec_mem))),
                    Instr::Mov(MovArgs::ToReg(Rsi, Arg64::Mem(idx_mem))),
                ]);
                self.memset(cx.si, 2, Reg32::Imm(MEM_SET_VAL));
                self.check_is_vec(Rdx);
                self.check_is_num(Rsi);
                self.append_instrs([
                    Instr::Sub(BinArgs::ToReg(Rdx, Arg32::Imm(1))),
                    Instr::Mov(MovArgs::ToReg(
                        Rcx,
                        Arg64::Mem(MemRef {
                            reg: Rdx,
                            offset: Offset::Constant(8),
                        }),
                    )),
                    Instr::Sar(BinArgs::ToReg(Rsi, Arg32::Imm(1))),
                    Instr::Cmp(BinArgs::ToReg(Rsi, Arg32::Imm(0))),
                    Instr::Jl(INDEX_OUT_OF_BOUNDS.to_string()),
                    Instr::Cmp(BinArgs::ToReg(Rsi, Arg32::Reg(Rcx))),
                    Instr::Jge(INDEX_OUT_OF_BOUNDS.to_string()),
                    Instr::Mov(MovArgs::ToMem(
                        MemRef {
                            reg: Rdx,
                            offset: Offset::Computed {
                                reg: Rsi,
                                factor: 8,
                                constant: 16,
                            },
                        },
                        Reg32::Reg(Rax),
                    )),
                ]);
                self.move_to(dst, Arg64::Reg(Rax));
            }
            Expr::PrintStack => {
                self.append_instrs([
                    Instr::Mov(MovArgs::ToReg(Rdi, Arg64::Reg(STACK_BASE_REG))),
                    Instr::Mov(MovArgs::ToReg(Rsi, Arg64::Reg(Rsp))),
                    Instr::Call("snek_print_stack".to_string()),
                ]);
                self.move_to(dst, 0.repr32());
            }
        }
    }

    fn compile_un_op(&mut self, cx: &Ctxt, dst: Loc, op: Op1, e: &Expr) {
        self.compile_expr(cx, Loc::Reg(Rax), e);
        match op {
            Op1::Add1 => {
                self.check_is_num(Reg::Rax);
                self.append_instrs([
                    Instr::Add(BinArgs::ToReg(Rax, 1.repr32())),
                    Instr::Jo(OVERFLOW_LBL.to_string()),
                ])
            }
            Op1::Sub1 => {
                self.check_is_num(Reg::Rax);
                self.append_instrs([
                    Instr::Sub(BinArgs::ToReg(Rax, 1.repr32())),
                    Instr::Jo(OVERFLOW_LBL.to_string()),
                ])
            }
            Op1::IsNum => {
                self.append_instrs([
                    Instr::And(BinArgs::ToReg(Rax, Arg32::Imm(0b001))),
                    Instr::Mov(MovArgs::ToReg(Rax, false.repr64())),
                    Instr::Mov(MovArgs::ToReg(Rcx, true.repr64())),
                    Instr::CMov(CMov::Z(Rax, Arg64::Reg(Rcx))),
                ]);
            }
            Op1::IsBool => {
                self.append_instrs([
                    Instr::And(BinArgs::ToReg(Rax, Arg32::Imm(0b011))),
                    Instr::Cmp(BinArgs::ToReg(Rax, Arg32::Imm(0b011))),
                    Instr::Mov(MovArgs::ToReg(Rax, false.repr64())),
                    Instr::Mov(MovArgs::ToReg(Rcx, true.repr64())),
                    Instr::CMov(CMov::E(Rax, Arg64::Reg(Rcx))),
                ]);
            }

            Op1::Print => self.append_instrs([
                Instr::Mov(MovArgs::ToReg(Rdi, Arg64::Reg(Rax))),
                Instr::Call("snek_print".to_string()),
            ]),
        }
        self.move_to(dst, Arg32::Reg(Rax));
    }

    fn move_to(&mut self, dst: Loc, src: impl Into<Arg64>) {
        let src = src.into();
        match (dst, src) {
            (Loc::Reg(reg1), Arg64::Reg(reg2)) if reg1 == reg2 => return,
            _ => {}
        }
        match (dst, src) {
            (Loc::Reg(reg), _) => self.append_instr(Instr::Mov(MovArgs::ToReg(reg, src))),
            (Loc::Mem(dst), Arg64::Reg(src)) => {
                self.append_instr(Instr::Mov(MovArgs::ToMem(dst, Reg32::Reg(src))))
            }
            (Loc::Mem(dst), Arg64::Imm(src)) => {
                if let Ok(src) = src.try_into() {
                    self.append_instr(Instr::Mov(MovArgs::ToMem(dst, Reg32::Imm(src))))
                } else {
                    self.append_instrs([
                        Instr::Mov(MovArgs::ToReg(Rdx, Arg64::Imm(src))),
                        Instr::Mov(MovArgs::ToMem(dst, Reg32::Reg(Rdx))),
                    ])
                }
            }
            (Loc::Mem(dst), Arg64::Mem(src)) => self.append_instrs([
                Instr::Mov(MovArgs::ToReg(Rdx, Arg64::Mem(src))),
                Instr::Mov(MovArgs::ToMem(dst, Reg32::Reg(Rdx))),
            ]),
        }
    }

    fn compile_bin_op(&mut self, cx: &Ctxt, dst: Loc, op: Op2, e1: &Expr, e2: &Expr) {
        let (nextcx, mem) = cx.next_local();
        self.compile_expr(cx, Loc::Mem(mem), e1);
        self.compile_expr(&nextcx, Loc::Reg(Rcx), e2);
        self.append_instr(Instr::Mov(MovArgs::ToReg(Rax, Arg64::Mem(mem))));
        self.memset(cx.si, 1, Reg32::Imm(MEM_SET_VAL));

        match op {
            Op2::Plus
            | Op2::Minus
            | Op2::Times
            | Op2::Greater
            | Op2::GreaterEqual
            | Op2::Less
            | Op2::LessEqual => {
                self.check_is_num(Rax);
                self.check_is_num(Rcx);
            }
            Op2::Equal => {
                self.append_instrs([
                    Instr::Cmp(BinArgs::ToReg(Rdx, Arg32::Reg(Rax))),
                    Instr::Xor(BinArgs::ToReg(Rdx, Arg32::Reg(Rcx))),
                    Instr::Test(BinArgs::ToReg(Rdx, Arg32::Imm(1))),
                    Instr::Jnz(INVALID_ARG_LBL.to_string()),
                ]);
            }
        }

        match op {
            Op2::Plus => {
                self.append_instrs([
                    Instr::Add(BinArgs::ToReg(Rax, Arg32::Reg(Rcx))),
                    Instr::Jo(OVERFLOW_LBL.to_string()),
                ]);
            }
            Op2::Minus => {
                self.append_instrs([
                    Instr::Sub(BinArgs::ToReg(Rax, Arg32::Reg(Rcx))),
                    Instr::Jo(OVERFLOW_LBL.to_string()),
                ]);
            }
            Op2::Times => {
                self.append_instrs([
                    Instr::Sar(BinArgs::ToReg(Rax, Arg32::Imm(1))),
                    Instr::IMul(BinArgs::ToReg(Rax, Arg32::Reg(Rcx))),
                    Instr::Jo(OVERFLOW_LBL.to_string()),
                ]);
            }
            Op2::Equal => self.compile_cmp(CMov::E),
            Op2::Greater => self.compile_cmp(CMov::G),
            Op2::GreaterEqual => self.compile_cmp(CMov::GE),
            Op2::Less => self.compile_cmp(CMov::L),
            Op2::LessEqual => self.compile_cmp(CMov::LE),
        }
        self.move_to(dst, Arg32::Reg(Rax));
    }

    fn compile_cmp(&mut self, cmp: impl FnOnce(Reg, Arg64) -> CMov) {
        self.append_instrs([
            Instr::Cmp(BinArgs::ToReg(Rax, Arg32::Reg(Rcx))),
            Instr::Mov(MovArgs::ToReg(Rax, false.repr64())),
            Instr::Mov(MovArgs::ToReg(Rcx, true.repr64())),
            Instr::CMov(cmp(Rax, Arg64::Reg(Rcx))),
        ]);
    }

    fn check_is_num(&mut self, reg: Reg) {
        self.append_instrs([
            Instr::Test(BinArgs::ToReg(reg, Arg32::Imm(0b001))),
            Instr::Jnz(INVALID_ARG_LBL.to_string()),
        ]);
    }

    fn check_is_vec(&mut self, reg: Reg) {
        self.append_instrs([
            Instr::Test(BinArgs::ToReg(reg, Arg32::Imm(0b001))),
            Instr::Jz(INVALID_ARG_LBL.to_string()), // jump if is num
            Instr::Test(BinArgs::ToReg(reg, Arg32::Imm(0b010))),
            Instr::Jnz(INVALID_ARG_LBL.to_string()), // jump if is bool
        ]);
    }

    fn append_instrs(&mut self, instrs: impl IntoIterator<Item = Instr>) {
        self.instrs.extend(instrs);
    }

    fn append_instr(&mut self, instr: Instr) {
        self.instrs.push(instr)
    }

    fn next_tag(&mut self) -> u32 {
        self.tag = self.tag.checked_add(1).unwrap();
        self.tag - 1
    }
}

fn stack_size(locals: u32, calle_saved: &[Reg]) -> u32 {
    let n = locals + calle_saved.len() as u32;
    if n % 2 == 0 {
        locals
    } else {
        locals + 1
    }
}

trait Repr64 {
    fn repr64(&self) -> Arg64;
}

trait Repr32 {
    fn repr32(&self) -> Arg32;
}

impl<T: Repr32> Repr64 for T {
    fn repr64(&self) -> Arg64 {
        self.repr32().into()
    }
}

impl Repr32 for i32 {
    fn repr32(&self) -> Arg32 {
        Arg32::Imm(*self << 1)
    }
}

impl Repr64 for i64 {
    fn repr64(&self) -> Arg64 {
        Arg64::Imm(self.checked_shl(1).unwrap())
    }
}

impl Repr32 for bool {
    fn repr32(&self) -> Arg32 {
        Arg32::Imm(if *self { 7 } else { 3 })
    }
}

fn fun_arity_map(prg: &Prog) -> Result<HashMap<Symbol, usize>, Symbol> {
    let mut map = HashMap::new();
    for fun in &prg.funs {
        if map.insert(fun.name, fun.params.len()).is_some() {
            return Err(fun.name);
        }
    }
    Ok(map)
}

fn check_dup_bindings<'a>(bindings: impl IntoIterator<Item = &'a Symbol>) {
    let mut seen = HashSet::new();
    for name in bindings {
        if !seen.insert(*name) {
            raise_duplicate_binding(*name);
        }
    }
}

fn raise_duplicate_binding(id: Symbol) {
    panic!("duplicate binding {id}");
}

fn raise_duplicate_function<T>(name: Symbol) -> T {
    panic!("duplicate function name {name}")
}

fn raise_unbound_identifier<T>(id: Symbol) -> T {
    panic!("unbound variable identifier {id}")
}

fn raise_break_outside_loop() {
    panic!("break outside loop")
}

fn raise_input_in_fun<T>() -> T {
    panic!("cannot use input inside function definition")
}

fn raise_undefined_fun(fun: Symbol) {
    panic!("function {fun} not defined")
}

fn raise_worng_number_of_args(expected: usize, got: usize) {
    panic!("function takes {expected} arguments but {got} were supplied")
}
