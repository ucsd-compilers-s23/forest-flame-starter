use std::collections::{HashMap, HashSet};

use crate::{
    asm::{
        instrs_to_string, Arg32, Arg64, BinArgs, CMov, Instr, Loc, MemRef, MovArgs, Offset,
        Reg::{self, *},
        Reg32,
        StrOp::Stosq,
    },
    mref,
    syntax::{Expr, FunDecl, Op1, Op2, Prog, Symbol},
};

struct Session {
    tag: u32,
    instrs: Vec<Instr>,
    funs: HashMap<Symbol, usize>,
}

const INVALID_ARG: &str = "invalid_argument";
const OVERFLOW: &str = "overflow";
const INDEX_OUT_OF_BOUNDS: &str = "index_out_of_bounds";
const INVALID_SIZE: &str = "invalid_vec_size";

const STACK_BASE: Reg = Rbx;
const INPUT_REG: Reg = R13;
const HEAP_END: Reg = R14;
const HEAP_PTR: Reg = R15;

const NIL: i32 = 0b001;
const MEM_SET_VAL: i32 = NIL;
const GC_WORD_VAL: i32 = 0;

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

    fn with_params(params: &[Symbol]) -> Ctxt<'a> {
        let env = params
            .iter()
            .enumerate()
            .map(|(i, param)| (*param, mref![Rbp + %(8 * (i + 2))]))
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
            mref![Rbp - %(8 * si)],
        )
    }

    fn add_binding(&self, x: Symbol, mem: MemRef) -> Ctxt<'a> {
        Ctxt {
            env: self.env.update(x, mem),
            ..*self
        }
    }
}

pub fn compile(prg: &Prog) -> String {
    match fun_arity_map(prg) {
        Ok(funs) => {
            let mut sess = Session::new(funs);
            let locals = depth(&prg.main);
            sess.compile_funs(&prg.funs);
            sess.emit_instr(Instr::Label("our_code_starts_here".to_string()));
            let callee_saved = [Rbp, STACK_BASE, INPUT_REG, HEAP_END, HEAP_PTR];
            sess.fun_entry(locals, &callee_saved);
            sess.emit_instrs([
                Instr::Mov(MovArgs::ToReg(STACK_BASE, Arg64::Reg(Rbp))),
                Instr::Mov(MovArgs::ToReg(INPUT_REG, Arg64::Reg(Rdi))),
                Instr::Mov(MovArgs::ToReg(HEAP_PTR, Arg64::Reg(Rsi))),
                Instr::Mov(MovArgs::ToReg(HEAP_END, Arg64::Reg(Rdx))),
            ]);
            sess.compile_expr(&Ctxt::new(), Loc::Reg(Rax), &prg.main);
            sess.fun_exit(locals, &callee_saved);

            format!(
                "
section .text
extern snek_error
extern snek_print
extern snek_alloc_vec
extern snek_print_stack
extern snek_try_gc
extern snek_gc
global our_code_starts_here
{}
{INVALID_ARG}:
  mov edi, 1
  call snek_error
{OVERFLOW}:
  mov edi, 2
  call snek_error
{INDEX_OUT_OF_BOUNDS}:
  mov edi, 3
  call snek_error
{INVALID_SIZE}:
  mov edi, 4
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
        let size = frame_size(locals, callee_saved);
        for reg in callee_saved {
            self.emit_instr(Instr::Push(Arg32::Reg(*reg)));
        }
        self.emit_instrs([
            Instr::Mov(MovArgs::ToReg(Rbp, Arg64::Reg(Rsp))),
            Instr::Sub(BinArgs::ToReg(Rsp, Arg32::Imm(8 * (size as i32)))),
        ]);
        self.memset(0, size, Reg32::Imm(MEM_SET_VAL));
    }

    fn fun_exit(&mut self, locals: u32, calle_saved: &[Reg]) {
        let size = frame_size(locals, calle_saved);
        self.emit_instrs([Instr::Add(BinArgs::ToReg(
            Rsp,
            Arg32::Imm(8 * (size as i32)),
        ))]);
        for reg in calle_saved.iter().rev() {
            self.emit_instr(Instr::Pop(Loc::Reg(*reg)));
        }
        self.emit_instr(Instr::Ret);
    }

    fn compile_funs(&mut self, funs: &[FunDecl]) {
        for fun in funs {
            self.compile_fun(fun)
        }
    }

    fn compile_fun(&mut self, fun: &FunDecl) {
        check_dup_bindings(&fun.params);
        let locals = depth(&fun.body);
        self.emit_instr(Instr::Label(fun_label(fun.name)));
        self.fun_entry(locals, &[Rbp]);
        self.compile_expr(&Ctxt::with_params(&fun.params), Loc::Reg(Rax), &fun.body);
        self.fun_exit(locals, &[Rbp]);
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
                self.emit_instrs([
                    Instr::Cmp(BinArgs::ToReg(Rax, false.repr32().into())),
                    Instr::Je(else_lbl.clone()),
                ]);
                self.compile_expr(cx, dst, e2);
                self.emit_instrs([Instr::Jmp(end_lbl.clone()), Instr::Label(else_lbl)]);
                self.compile_expr(cx, dst, e3);
                self.emit_instr(Instr::Label(end_lbl))
            }
            Expr::Loop(e) => {
                let tag = self.next_tag();
                let loop_start_lbl = format!("loop_start_{tag}");
                let loop_end_lbl = format!("loop_end_{tag}");

                self.emit_instr(Instr::Label(loop_start_lbl.clone()));
                self.compile_expr(&cx.set_curr_lbl(&loop_end_lbl), dst, e);
                self.emit_instrs([Instr::Jmp(loop_start_lbl), Instr::Label(loop_end_lbl)]);
                self.move_to(dst, Arg64::Reg(Rax));
            }
            Expr::Break(e) => {
                if let Some(lbl) = cx.curr_lbl {
                    self.compile_expr(cx, Loc::Reg(Rax), e);
                    self.emit_instr(Instr::Jmp(lbl.to_string()));
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
                    raise_wrong_number_of_args(*fun, *arity, args.len());
                }

                let mut currcx = cx.clone();
                for arg in args {
                    let (nextcx, mem) = currcx.next_local();
                    self.compile_expr(&currcx, Loc::Mem(mem), arg);
                    currcx = nextcx;
                }
                self.call(*fun, locals(cx.si, args.len() as u32).map(Arg32::Mem));
                self.memset(cx.si, args.len() as u32, Reg32::Imm(MEM_SET_VAL));
                self.move_to(dst, Arg64::Reg(Rax));
            }
            Expr::Nil => {
                self.move_to(dst, Arg32::Imm(NIL));
            }
            Expr::Input => {
                if cx.in_fun {
                    raise_input_in_fun()
                } else {
                    self.move_to(dst, Arg32::Reg(INPUT_REG))
                }
            }
            Expr::MakeVec(size, elem) => {
                let tag = self.next_tag();
                let alloc_finish_lbl = format!("make_vec_alloc_finish_{tag}");

                let (nextcx, size_mem) = cx.next_local();
                let (_, elem_mem) = nextcx.next_local();

                self.compile_expr(cx, Loc::Mem(size_mem), size);
                self.compile_expr(&nextcx, Loc::Mem(elem_mem), elem);
                self.emit_instr(Instr::Mov(MovArgs::ToReg(Rdi, Arg64::Mem(size_mem))));
                self.check_is_num(Rdi);
                self.emit_instrs([
                    Instr::Sar(BinArgs::ToReg(Rdi, Arg32::Imm(1))),
                    Instr::Cmp(BinArgs::ToReg(Rdi, Arg32::Imm(0))),
                    Instr::Jl(INVALID_SIZE.to_string()),
                    Instr::Lea(Rax, mref![HEAP_PTR + 8 * Rdi + 16]),
                    Instr::Cmp(BinArgs::ToReg(Rax, Arg32::Reg(HEAP_END))),
                    Instr::Jle(alloc_finish_lbl.clone()),
                    // Call try_gc to ensure we can allocate `size + 2` quad words
                    // (1 extra for the size of the vector + 1 extra for the GC metadata)
                    Instr::Add(BinArgs::ToReg(Rdi, Arg32::Imm(2))),
                    Instr::Mov(MovArgs::ToReg(Rsi, Arg64::Reg(HEAP_PTR))),
                    Instr::Mov(MovArgs::ToReg(Rdx, Arg64::Reg(STACK_BASE))),
                    Instr::Mov(MovArgs::ToReg(Rcx, Arg64::Reg(Rbp))),
                    Instr::Mov(MovArgs::ToReg(R8, Arg64::Reg(Rsp))),
                    Instr::Call("snek_try_gc".to_string()),
                    Instr::Mov(MovArgs::ToReg(HEAP_PTR, Arg64::Reg(Rax))),
                    Instr::Label(alloc_finish_lbl),
                    // Load size again in %rsi
                    Instr::Mov(MovArgs::ToReg(Rsi, Arg64::Mem(size_mem))),
                    Instr::Sar(BinArgs::ToReg(Rsi, Arg32::Imm(1))),
                    // Write GC word in HEAP_PTR
                    Instr::Mov(MovArgs::ToMem(mref!(HEAP_PTR + 0), Reg32::Imm(GC_WORD_VAL))),
                    // Write size in HEAP_PTR + 8
                    Instr::Mov(MovArgs::ToMem(mref!(HEAP_PTR + 8), Reg32::Reg(Rsi))),
                    // Fill vector using `rep stosq` (%rdi = ptr, %rcx = count, %rax = val)
                    Instr::Lea(Rdi, mref!(HEAP_PTR + 16)),
                    Instr::Mov(MovArgs::ToReg(Rcx, Arg64::Reg(Rsi))),
                    Instr::Mov(MovArgs::ToReg(Rax, Arg64::Mem(elem_mem))),
                    Instr::Rep(Stosq),
                    // Add tag to heap ptr and store it in %rax as the result of the expression
                    Instr::Lea(Rax, mref!(HEAP_PTR + 1)),
                    // Bump heap ptr
                    Instr::Lea(HEAP_PTR, mref!(HEAP_PTR + 8 * Rsi + 16)),
                ]);
                self.memset(cx.si, 2, Reg32::Imm(MEM_SET_VAL));
                self.move_to(dst, Arg64::Reg(Rax));
            }
            Expr::Vec(elems) => {
                let tag = self.next_tag();
                let vec_alloc_finish_lbl = format!("vec_alloc_finish_{tag}");

                let size: i32 = elems.len().try_into().unwrap();
                let mut currcx = cx.clone();
                for elem in elems {
                    let (nextcx, mem) = currcx.next_local();
                    self.compile_expr(&currcx, Loc::Mem(mem), elem);
                    currcx = nextcx;
                }

                self.emit_instrs([
                    Instr::Lea(Rax, mref![HEAP_PTR + %(8 * (size + 2))]),
                    Instr::Cmp(BinArgs::ToReg(Rax, Arg32::Reg(HEAP_END))),
                    Instr::Jle(vec_alloc_finish_lbl.clone()),
                    // Call try_gc to ensure we can allocate `size + 2` quad words
                    // (1 extra for the size of the vector + 1 extra for the GC metadata)
                    Instr::Mov(MovArgs::ToReg(Rdi, Arg64::Imm(size as i64 + 2))),
                    Instr::Mov(MovArgs::ToReg(Rsi, Arg64::Reg(HEAP_PTR))),
                    Instr::Mov(MovArgs::ToReg(Rdx, Arg64::Reg(STACK_BASE))),
                    Instr::Mov(MovArgs::ToReg(Rcx, Arg64::Reg(Rbp))),
                    Instr::Mov(MovArgs::ToReg(R8, Arg64::Reg(Rsp))),
                    Instr::Call("snek_try_gc".to_string()),
                    Instr::Mov(MovArgs::ToReg(HEAP_PTR, Arg64::Reg(Rax))),
                    Instr::Label(vec_alloc_finish_lbl),
                    // Write GC word in HEAP_PTR
                    Instr::Mov(MovArgs::ToMem(mref!(HEAP_PTR + 0), Reg32::Imm(GC_WORD_VAL))),
                    // Write size in HEAP_PTR + 8
                    Instr::Mov(MovArgs::ToMem(mref!(HEAP_PTR + 8), Reg32::Imm(size))),
                ]);

                for i in 0..elems.len() as u32 {
                    self.move_to(
                        Loc::Mem(mref!(HEAP_PTR + %(8 * (i + 2)))),
                        Arg64::Mem(mref!(Rbp - %(8 * (cx.si + i + 1)))),
                    )
                }

                self.emit_instrs([
                    // Add tag to heap ptr and store it in %rax as the result of the expression
                    Instr::Lea(Rax, mref!(HEAP_PTR + 1)),
                    // Bump heap ptr
                    Instr::Lea(HEAP_PTR, mref!(HEAP_PTR + %(8 * (size + 2)))),
                ]);
                self.memset(cx.si, elems.len() as u32, Reg32::Imm(MEM_SET_VAL));
                self.move_to(dst, Arg64::Reg(Rax));
            }
            Expr::VecSet(vec, idx, elem) => {
                let (nextcx1, vec_mem) = cx.next_local();
                let (nextcx2, idx_mem) = nextcx1.next_local();

                self.compile_expr(cx, Loc::Mem(vec_mem), vec);
                self.compile_expr(&nextcx1, Loc::Mem(idx_mem), idx);
                self.compile_expr(&nextcx2, Loc::Reg(Rsi), elem);

                self.emit_instrs([
                    Instr::Mov(MovArgs::ToReg(Rax, Arg64::Mem(vec_mem))),
                    Instr::Mov(MovArgs::ToReg(Rdi, Arg64::Mem(idx_mem))),
                ]);
                self.memset(cx.si, 2, Reg32::Imm(MEM_SET_VAL));
                self.check_is_vec(Rax);
                self.check_is_not_nil(Rax);
                self.check_is_num(Rdi);
                self.emit_instrs([
                    Instr::Mov(MovArgs::ToReg(Rcx, Arg64::Reg(Rax))),
                    Instr::Sub(BinArgs::ToReg(Rcx, Arg32::Imm(1))),
                    Instr::Sar(BinArgs::ToReg(Rdi, Arg32::Imm(1))),
                    Instr::Cmp(BinArgs::ToReg(Rdi, Arg32::Imm(0))),
                    Instr::Jl(INDEX_OUT_OF_BOUNDS.to_string()),
                    Instr::Mov(MovArgs::ToReg(Rdx, Arg64::Mem(mref![Rcx + 8]))),
                    Instr::Cmp(BinArgs::ToReg(Rdi, Arg32::Reg(Rdx))),
                    Instr::Jge(INDEX_OUT_OF_BOUNDS.to_string()),
                    Instr::Mov(MovArgs::ToMem(mref![Rcx + 8 * Rdi + 16], Reg32::Reg(Rsi))),
                ]);
                self.move_to(dst, Arg64::Reg(Rax));
            }
            Expr::VecGet(vec, idx) => {
                let (nextcx, vec_mem) = cx.next_local();

                self.compile_expr(cx, Loc::Mem(vec_mem), vec);
                self.compile_expr(&nextcx, Loc::Reg(Rdi), idx);

                self.emit_instrs([Instr::Mov(MovArgs::ToReg(Rax, Arg64::Mem(vec_mem)))]);
                self.memset(cx.si, 1, Reg32::Imm(MEM_SET_VAL));
                self.check_is_vec(Rax);
                self.check_is_not_nil(Rax);
                self.check_is_num(Rdi);
                self.emit_instrs([
                    Instr::Sub(BinArgs::ToReg(Rax, Arg32::Imm(1))),
                    Instr::Mov(MovArgs::ToReg(Rdx, Arg64::Mem(mref![Rax + 8]))),
                    Instr::Sar(BinArgs::ToReg(Rdi, Arg32::Imm(1))),
                    Instr::Cmp(BinArgs::ToReg(Rdi, Arg32::Imm(0))),
                    Instr::Jl(INDEX_OUT_OF_BOUNDS.to_string()),
                    Instr::Cmp(BinArgs::ToReg(Rdi, Arg32::Reg(Rdx))),
                    Instr::Jge(INDEX_OUT_OF_BOUNDS.to_string()),
                    Instr::Mov(MovArgs::ToReg(Rax, Arg64::Mem(mref![Rax + 8 * Rdi + 16]))),
                ]);
                self.move_to(dst, Arg64::Reg(Rax));
            }
            Expr::VecLen(vec) => {
                self.compile_expr(cx, Loc::Reg(Rax), vec);
                self.check_is_vec(Rax);
                self.check_is_not_nil(Rax);
                self.emit_instrs([
                    Instr::Sub(BinArgs::ToReg(Rax, Arg32::Imm(1))),
                    Instr::Mov(MovArgs::ToReg(Rax, Arg64::Mem(mref![Rax + 8]))),
                    Instr::Sal(BinArgs::ToReg(Rax, Arg32::Imm(1))),
                ]);
                self.move_to(dst, Arg64::Reg(Rax));
            }
            Expr::Gc => {
                self.emit_instrs([
                    Instr::Mov(MovArgs::ToReg(Rdi, Arg64::Reg(HEAP_PTR))),
                    Instr::Mov(MovArgs::ToReg(Rsi, Arg64::Reg(STACK_BASE))),
                    Instr::Mov(MovArgs::ToReg(Rdx, Arg64::Reg(Rbp))),
                    Instr::Mov(MovArgs::ToReg(Rcx, Arg64::Reg(Rsp))),
                    Instr::Call("snek_gc".to_string()),
                    Instr::Mov(MovArgs::ToReg(HEAP_PTR, Arg64::Reg(Rax))),
                ]);
                self.move_to(dst, 0.repr32());
            }
            Expr::PrintStack => {
                self.emit_instrs([
                    Instr::Mov(MovArgs::ToReg(Rdi, Arg64::Reg(STACK_BASE))),
                    Instr::Mov(MovArgs::ToReg(Rsi, Arg64::Reg(Rbp))),
                    Instr::Mov(MovArgs::ToReg(Rdx, Arg64::Reg(Rsp))),
                    Instr::Call("snek_print_stack".to_string()),
                ]);
                self.move_to(dst, 0.repr32());
            }
        }
    }

    fn call(&mut self, fun: Symbol, args: impl IntoIterator<Item = Arg32>) {
        let mut args: Vec<_> = args.into_iter().collect();
        if args.len() % 2 != 0 {
            args.push(Arg32::Imm(MEM_SET_VAL));
        }
        for arg in args.iter().rev() {
            self.emit_instr(Instr::Push(*arg))
        }
        self.emit_instrs([
            Instr::Call(fun_label(fun)),
            Instr::Add(BinArgs::ToReg(Rsp, Arg32::Imm(8 * args.len() as i32))),
        ]);
    }

    fn compile_un_op(&mut self, cx: &Ctxt, dst: Loc, op: Op1, e: &Expr) {
        self.compile_expr(cx, Loc::Reg(Rax), e);
        match op {
            Op1::Add1 => {
                self.check_is_num(Reg::Rax);
                self.emit_instrs([
                    Instr::Add(BinArgs::ToReg(Rax, 1.repr32())),
                    Instr::Jo(OVERFLOW.to_string()),
                ])
            }
            Op1::Sub1 => {
                self.check_is_num(Reg::Rax);
                self.emit_instrs([
                    Instr::Sub(BinArgs::ToReg(Rax, 1.repr32())),
                    Instr::Jo(OVERFLOW.to_string()),
                ])
            }
            Op1::IsNum => {
                self.emit_instrs([
                    Instr::And(BinArgs::ToReg(Rax, Arg32::Imm(0b001))),
                    Instr::Mov(MovArgs::ToReg(Rax, false.repr64())),
                    Instr::Mov(MovArgs::ToReg(Rcx, true.repr64())),
                    Instr::CMov(CMov::Z(Rax, Arg64::Reg(Rcx))),
                ]);
            }
            Op1::IsBool => {
                self.emit_instrs([
                    Instr::And(BinArgs::ToReg(Rax, Arg32::Imm(0b011))),
                    Instr::Cmp(BinArgs::ToReg(Rax, Arg32::Imm(0b011))),
                    Instr::Mov(MovArgs::ToReg(Rax, false.repr64())),
                    Instr::Mov(MovArgs::ToReg(Rcx, true.repr64())),
                    Instr::CMov(CMov::E(Rax, Arg64::Reg(Rcx))),
                ]);
            }
            Op1::IsVec => {
                self.emit_instrs([
                    Instr::Mov(MovArgs::ToReg(Rdx, Arg64::Reg(Rax))),
                    Instr::Mov(MovArgs::ToReg(Rax, true.repr64())),
                    Instr::Mov(MovArgs::ToReg(Rcx, false.repr64())),
                    Instr::Test(BinArgs::ToReg(Rdx, Arg32::Imm(0b01))),
                    Instr::CMov(CMov::Z(Rax, Arg64::Reg(Rcx))),
                    Instr::Test(BinArgs::ToReg(Rdx, Arg32::Imm(0b10))),
                    Instr::CMov(CMov::NZ(Rax, Arg64::Reg(Rcx))),
                ]);
            }
            Op1::Print => self.emit_instrs([
                Instr::Mov(MovArgs::ToReg(Rdi, Arg64::Reg(Rax))),
                Instr::Call("snek_print".to_string()),
            ]),
        }
        self.move_to(dst, Arg32::Reg(Rax));
    }

    fn compile_bin_op(&mut self, cx: &Ctxt, dst: Loc, op: Op2, e1: &Expr, e2: &Expr) {
        let (nextcx, mem) = cx.next_local();
        self.compile_expr(cx, Loc::Mem(mem), e1);
        self.compile_expr(&nextcx, Loc::Reg(Rcx), e2);
        self.emit_instr(Instr::Mov(MovArgs::ToReg(Rax, Arg64::Mem(mem))));
        self.memset(cx.si, 1, Reg32::Imm(MEM_SET_VAL));

        match op {
            Op2::Plus
            | Op2::Minus
            | Op2::Times
            | Op2::Divide
            | Op2::Greater
            | Op2::GreaterEqual
            | Op2::Less
            | Op2::LessEqual => {
                self.check_is_num(Rax);
                self.check_is_num(Rcx);
            }
            Op2::Equal => {
                let tag = self.next_tag();
                let check_eq_finish_lbl = format!("check_eq_finish_{tag}");
                // if (%rax ^ %rcx) & 0b11 == 0 {
                //     jmp check_eq_finish
                // } else if (%rax | %rcx) & 0b01 != 0 {
                //     jmp invalid_arg
                // }
                self.emit_instrs([
                    Instr::Mov(MovArgs::ToReg(Rdx, Arg64::Reg(Rax))),
                    Instr::Xor(BinArgs::ToReg(Rdx, Arg32::Reg(Rcx))),
                    Instr::Test(BinArgs::ToReg(Rdx, Arg32::Imm(0b11))),
                    Instr::Jz(check_eq_finish_lbl.to_string()),
                    Instr::Mov(MovArgs::ToReg(Rdx, Arg64::Reg(Rax))),
                    Instr::Or(BinArgs::ToReg(Rdx, Arg32::Reg(Rcx))),
                    Instr::Test(BinArgs::ToReg(Rdx, Arg32::Imm(0b01))),
                    Instr::Jnz(INVALID_ARG.to_string()),
                    Instr::Label(check_eq_finish_lbl.to_string()),
                ]);
            }
        }

        match op {
            Op2::Plus => {
                self.emit_instrs([
                    Instr::Add(BinArgs::ToReg(Rax, Arg32::Reg(Rcx))),
                    Instr::Jo(OVERFLOW.to_string()),
                ]);
            }
            Op2::Minus => {
                self.emit_instrs([
                    Instr::Sub(BinArgs::ToReg(Rax, Arg32::Reg(Rcx))),
                    Instr::Jo(OVERFLOW.to_string()),
                ]);
            }
            Op2::Times => {
                self.emit_instrs([
                    Instr::Sar(BinArgs::ToReg(Rax, Arg32::Imm(1))),
                    Instr::IMul(BinArgs::ToReg(Rax, Arg32::Reg(Rcx))),
                    Instr::Jo(OVERFLOW.to_string()),
                ]);
            }
            Op2::Divide => {
                self.emit_instrs([
                    Instr::Cqo,
                    Instr::IDiv(Rcx),
                    Instr::Sal(BinArgs::ToReg(Rax, Arg32::Imm(1))),
                    Instr::Jo(OVERFLOW.to_string()),
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
        self.emit_instrs([
            Instr::Cmp(BinArgs::ToReg(Rax, Arg32::Reg(Rcx))),
            Instr::Mov(MovArgs::ToReg(Rax, false.repr64())),
            Instr::Mov(MovArgs::ToReg(Rcx, true.repr64())),
            Instr::CMov(cmp(Rax, Arg64::Reg(Rcx))),
        ]);
    }

    fn move_to(&mut self, dst: Loc, src: impl Into<Arg64>) {
        let src = src.into();
        if dst == src {
            return;
        }
        match (dst, src) {
            (Loc::Reg(reg), _) => self.emit_instr(Instr::Mov(MovArgs::ToReg(reg, src))),
            (Loc::Mem(dst), Arg64::Reg(src)) => {
                self.emit_instr(Instr::Mov(MovArgs::ToMem(dst, Reg32::Reg(src))))
            }
            (Loc::Mem(dst), Arg64::Imm(src)) => {
                if let Ok(src) = src.try_into() {
                    self.emit_instr(Instr::Mov(MovArgs::ToMem(dst, Reg32::Imm(src))))
                } else {
                    self.emit_instrs([
                        Instr::Mov(MovArgs::ToReg(Rdx, Arg64::Imm(src))),
                        Instr::Mov(MovArgs::ToMem(dst, Reg32::Reg(Rdx))),
                    ])
                }
            }
            (Loc::Mem(dst), Arg64::Mem(src)) => self.emit_instrs([
                Instr::Mov(MovArgs::ToReg(Rdx, Arg64::Mem(src))),
                Instr::Mov(MovArgs::ToMem(dst, Reg32::Reg(Rdx))),
            ]),
        }
    }

    fn memset(&mut self, start: u32, count: u32, elem: Reg32) {
        for mem in locals(start, count) {
            self.emit_instr(Instr::Mov(MovArgs::ToMem(mem, elem)));
        }
    }

    fn check_is_num(&mut self, reg: Reg) {
        self.emit_instrs([
            Instr::Test(BinArgs::ToReg(reg, Arg32::Imm(0b001))),
            Instr::Jnz(INVALID_ARG.to_string()),
        ]);
    }

    fn check_is_vec(&mut self, reg: Reg) {
        self.emit_instrs([
            Instr::Test(BinArgs::ToReg(reg, Arg32::Imm(0b001))),
            Instr::Jz(INVALID_ARG.to_string()), // jump if is num
            Instr::Test(BinArgs::ToReg(reg, Arg32::Imm(0b010))),
            Instr::Jnz(INVALID_ARG.to_string()), // jump if is bool
        ]);
    }

    fn check_is_not_nil(&mut self, reg: Reg) {
        self.emit_instrs([
            Instr::Cmp(BinArgs::ToReg(reg, Arg32::Imm(NIL))),
            Instr::Jz(INVALID_ARG.to_string()), // jump if exactly equal to 1
        ]);
    }

    fn emit_instrs(&mut self, instrs: impl IntoIterator<Item = Instr>) {
        self.instrs.extend(instrs);
    }

    fn emit_instr(&mut self, instr: Instr) {
        self.instrs.push(instr)
    }

    fn next_tag(&mut self) -> u32 {
        self.tag = self.tag.checked_add(1).unwrap();
        self.tag - 1
    }
}

fn locals(start: u32, count: u32) -> impl Iterator<Item = MemRef> {
    (start..start + count).map(|i| mref![Rbp - %(8 * (i + 1))])
}

fn frame_size(locals: u32, calle_saved: &[Reg]) -> u32 {
    // #locals + #callee saved + return address
    let n = locals + calle_saved.len() as u32 + 1;
    if n % 2 == 0 {
        locals
    } else {
        locals + 1
    }
}

fn depth(e: &Expr) -> u32 {
    match e {
        Expr::BinOp(_, e1, e2) => depth(e1).max(depth(e2) + 1),
        Expr::Let(bindings, e) => bindings
            .iter()
            .enumerate()
            .map(|(i, (_, e))| depth(e) + (i as u32))
            .max()
            .unwrap_or(0)
            .max(depth(e) + bindings.len() as u32),
        Expr::If(e1, e2, e3) => depth(e1).max(depth(e2)).max(depth(e3)),
        Expr::Block(es) => es.iter().map(depth).max().unwrap_or(0),
        Expr::UnOp(_, e) | Expr::Loop(e) | Expr::Break(e) | Expr::Set(_, e) => depth(e),
        Expr::MakeVec(size, elem) => depth(size).max(depth(elem) + 1).max(2),
        Expr::Call(_, es) | Expr::Vec(es) => es
            .iter()
            .enumerate()
            .map(|(i, e)| depth(e) + (i as u32))
            .max()
            .unwrap_or(0)
            .max(es.len() as u32),
        Expr::VecSet(vec, idx, val) => depth(vec).max(depth(idx) + 1).max(depth(val) + 2).max(2),
        Expr::VecGet(vec, idx) => depth(vec).max(depth(idx) + 1),
        Expr::PrintStack
        | Expr::Gc
        | Expr::VecLen(_)
        | Expr::Input
        | Expr::Nil
        | Expr::Var(_)
        | Expr::Number(_)
        | Expr::Boolean(_) => 0,
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

fn raise_wrong_number_of_args(fun: Symbol, expected: usize, got: usize) {
    panic!("function {fun} takes {expected} arguments but {got} were supplied")
}

fn fun_label(fun: Symbol) -> String {
    format!("snek_fun_{}", fun.replace("-", "_"))
}
