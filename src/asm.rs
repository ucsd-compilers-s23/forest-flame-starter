#![allow(unused)]

use std::fmt::format;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Reg {
    Rax,
    Rbx,
    Rdx,
    Rcx,
    Rsi,
    Rdi,
    Rsp,
    Rbp,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MemRef {
    pub reg: Reg,
    pub offset: Offset,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Offset {
    Constant(i32),
    Computed {
        reg: Reg,
        factor: i32,
        constant: i32,
    },
}

#[macro_export]
macro_rules! mref {
    ($reg:ident + $factor:literal * $idx:ident + $constant:literal) => {{
        MemRef {
            reg: $reg,
            offset: Offset::Computed {
                reg: $idx,
                factor: $factor,
                constant: $constant,
            },
        }
    }};
    ($reg:ident + %($offset:expr)) => {{
        let offset: i32 = $offset.try_into().unwrap();
        MemRef {
            reg: $reg,
            offset: Offset::Constant(offset),
        }
    }};
    ($reg:ident - %($offset:expr)) => {{
        let offset: i32 = $offset.try_into().unwrap();
        MemRef {
            reg: $reg,
            offset: Offset::Constant(-offset),
        }
    }};
    ($reg:ident + $offset:literal) => {
        $crate::mref!($reg + %($offset))
    };
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arg64 {
    Reg(Reg),
    Imm(i64),
    Mem(MemRef),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arg32 {
    Reg(Reg),
    Imm(i32),
    Mem(MemRef),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reg32 {
    Reg(Reg),
    Imm(i32),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MovArgs {
    ToReg(Reg, Arg64),
    ToMem(MemRef, Reg32),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BinArgs {
    ToReg(Reg, Arg32),
    ToMem(MemRef, Reg32),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Loc {
    Reg(Reg),
    Mem(MemRef),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CMov {
    E(Reg, Arg64),
    Z(Reg, Arg64),
    NZ(Reg, Arg64),
    NE(Reg, Arg64),
    G(Reg, Arg64),
    GE(Reg, Arg64),
    L(Reg, Arg64),
    LE(Reg, Arg64),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Instr {
    Mov(MovArgs),
    CMov(CMov),

    Add(BinArgs),
    Sub(BinArgs),
    IMul(BinArgs),
    IDiv(Reg),
    And(BinArgs),
    Or(BinArgs),
    Xor(BinArgs),
    Shr(BinArgs),
    Sar(BinArgs),
    Sal(BinArgs),
    Shl(BinArgs),
    Cmp(BinArgs),
    Not(Loc),
    Test(BinArgs),

    Push(Arg32),
    Pop(Loc),

    Label(String),

    Call(String),
    Ret,

    Jmp(String),
    Je(String),
    Jne(String),
    Jl(String),
    Jle(String),
    Jg(String),
    Jge(String),

    Js(String),  // jump if msb is 1
    Jz(String),  // jump if result was 0
    Jnz(String), // jump if result was not 0

    Jo(String),  // jump if last arith operation overflowed
    Jno(String), // jump if last arith operation didn't overflow

    Lea(Reg, MemRef),
    Rep(StrOp),
    Cqo,

    Comment(String),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StrOp {
    Stosq,
}

pub fn reg_to_string(r: Reg) -> String {
    match r {
        Reg::Rax => String::from("rax"),
        Reg::Rbx => String::from("rbx"),
        Reg::Rcx => String::from("rcx"),
        Reg::Rdx => String::from("rdx"),
        Reg::Rsi => String::from("rsi"),
        Reg::Rdi => String::from("rdi"),
        Reg::Rsp => String::from("rsp"),
        Reg::Rbp => String::from("rbp"),
        Reg::R8 => String::from("r8"),
        Reg::R9 => String::from("r9"),
        Reg::R10 => String::from("r10"),
        Reg::R11 => String::from("r11"),
        Reg::R12 => String::from("r12"),
        Reg::R13 => String::from("r13"),
        Reg::R14 => String::from("r14"),
        Reg::R15 => String::from("r15"),
    }
}

impl PartialEq<Loc> for Arg64 {
    fn eq(&self, other: &Loc) -> bool {
        match (self, other) {
            (Arg64::Reg(r1), Loc::Reg(r2)) => r1 == r2,
            (Arg64::Mem(m1), Loc::Mem(m2)) => m1 == m2,
            (Arg64::Imm(_), Loc::Reg(_))
            | (Arg64::Imm(_), Loc::Mem(_))
            | (Arg64::Mem(_), Loc::Reg(_))
            | (Arg64::Reg(_), Loc::Mem(_)) => false,
        }
    }
}

impl PartialEq<Arg64> for Loc {
    fn eq(&self, other: &Arg64) -> bool {
        PartialEq::eq(other, self)
    }
}

impl From<Loc> for Arg32 {
    fn from(loc: Loc) -> Self {
        match loc {
            Loc::Reg(r) => Arg32::Reg(r),
            Loc::Mem(m) => Arg32::Mem(m),
        }
    }
}

impl From<Arg32> for Arg64 {
    fn from(arg: Arg32) -> Self {
        match arg {
            Arg32::Reg(r) => Arg64::Reg(r),
            Arg32::Imm(i) => Arg64::Imm(i as i64),
            Arg32::Mem(m) => Arg64::Mem(m),
        }
    }
}

pub fn imm32_to_string(i: i32) -> String {
    i.to_string()
}

pub fn mem_ref_to_string(m: MemRef) -> String {
    format!(
        "QWORD [{} {}]",
        reg_to_string(m.reg),
        offset_to_string(m.offset)
    )
}

fn offset_to_string(off: Offset) -> String {
    match off {
        Offset::Constant(n) => {
            if n < 0 {
                format!("- {}", n.unsigned_abs())
            } else {
                format!("+ {}", n)
            }
        }
        Offset::Computed {
            reg,
            factor,
            constant,
        } => {
            format!("+ {factor} * {} + {constant}", reg_to_string(reg))
        }
    }
}

pub fn reg32_to_string(r_or_i: Reg32) -> String {
    match r_or_i {
        Reg32::Reg(r) => reg_to_string(r),
        Reg32::Imm(i) => imm32_to_string(i),
    }
}

pub fn arg32_to_string(arg: Arg32) -> String {
    match arg {
        Arg32::Reg(r) => reg_to_string(r),
        Arg32::Imm(i) => imm32_to_string(i),
        Arg32::Mem(m) => mem_ref_to_string(m),
    }
}

pub fn arg64_to_string(arg: &Arg64) -> String {
    match arg {
        Arg64::Reg(r) => reg_to_string(*r),
        Arg64::Imm(i) => i.to_string(),
        Arg64::Mem(m) => mem_ref_to_string(*m),
    }
}

pub fn mov_args_to_string(args: &MovArgs) -> String {
    match args {
        MovArgs::ToReg(r, arg) => {
            format!("{}, {}", reg_to_string(*r), arg64_to_string(arg))
        }
        MovArgs::ToMem(mem, arg) => {
            format!("{}, {}", mem_ref_to_string(*mem), reg32_to_string(*arg))
        }
    }
}

pub fn bin_args_to_string(args: BinArgs) -> String {
    match args {
        BinArgs::ToReg(r, arg) => {
            format!("{}, {}", reg_to_string(r), arg32_to_string(arg))
        }
        BinArgs::ToMem(mem, arg) => {
            format!("{}, {}", mem_ref_to_string(mem), reg32_to_string(arg))
        }
    }
}

pub fn loc_to_string(loc: Loc) -> String {
    match loc {
        Loc::Reg(r) => reg_to_string(r),
        Loc::Mem(m) => mem_ref_to_string(m),
    }
}

pub fn instr_to_string(i: &Instr) -> String {
    match i {
        Instr::Comment(s) => format!(";; {}", s),
        Instr::Mov(args) => format!("  mov {}", mov_args_to_string(args)),
        Instr::Add(args) => format!("  add {}", bin_args_to_string(*args)),
        Instr::Sub(args) => format!("  sub {}", bin_args_to_string(*args)),
        Instr::IMul(args) => format!("  imul {}", bin_args_to_string(*args)),
        Instr::IDiv(reg) => format!("  idiv {}", reg_to_string(*reg)),
        Instr::And(args) => format!("  and {}", bin_args_to_string(*args)),
        Instr::Or(args) => format!("  or {}", bin_args_to_string(*args)),
        Instr::Not(loc) => format!("  not {}", loc_to_string(*loc)),
        Instr::Xor(args) => format!("  xor {}", bin_args_to_string(*args)),
        Instr::Shr(args) => format!("  shr {}", bin_args_to_string(*args)),
        Instr::Shl(args) => format!("  shl {}", bin_args_to_string(*args)),
        Instr::Sar(args) => format!("  sar {}", bin_args_to_string(*args)),
        Instr::Sal(args) => format!("  sal {}", bin_args_to_string(*args)),
        Instr::Cmp(args) => format!("  cmp {}", bin_args_to_string(*args)),
        Instr::Test(args) => format!("  test {}", bin_args_to_string(*args)),
        Instr::Push(arg) => format!("  push {}", arg32_to_string(*arg)),
        Instr::Pop(loc) => format!("  pop {}", loc_to_string(*loc)),
        Instr::Label(s) => format!("{}:", s),

        Instr::Call(s) => format!("  call {s}"),
        Instr::Ret => format!("  ret"),
        Instr::Jmp(s) => format!("  jmp {s}"),
        Instr::Je(s) => format!("  je {s}"),
        Instr::Jne(s) => format!("  jne {s}"),
        Instr::Jle(s) => format!("  jle {s}"),
        Instr::Jl(s) => format!("  jl {s}"),
        Instr::Jg(s) => format!("  jg {s}"),
        Instr::Jge(s) => format!("  jge {s}"),
        Instr::Js(s) => format!("  js {s}"),
        Instr::Jz(s) => format!("  jz {s}"),
        Instr::Jnz(s) => format!("  jnz {s}"),
        Instr::Jo(s) => format!("  jo {s}"),
        Instr::Jno(s) => format!("  jno {s}"),
        Instr::CMov(cmov) => match cmov {
            CMov::E(reg, arg) => {
                format!("  cmove {}, {}", reg_to_string(*reg), arg64_to_string(arg))
            }
            CMov::Z(reg, arg) => {
                format!("  cmovz {}, {}", reg_to_string(*reg), arg64_to_string(arg))
            }
            CMov::NZ(reg, arg) => {
                format!("  cmovnz {}, {}", reg_to_string(*reg), arg64_to_string(arg))
            }
            CMov::NE(reg, arg) => {
                format!("  cmovne {}, {}", reg_to_string(*reg), arg64_to_string(arg))
            }
            CMov::G(reg, arg) => {
                format!("  cmovg {}, {}", reg_to_string(*reg), arg64_to_string(arg))
            }
            CMov::GE(reg, arg) => {
                format!("  cmovge {}, {}", reg_to_string(*reg), arg64_to_string(arg))
            }
            CMov::L(reg, arg) => {
                format!("  cmovl {}, {}", reg_to_string(*reg), arg64_to_string(arg))
            }
            CMov::LE(reg, arg) => {
                format!("  cmovle {}, {}", reg_to_string(*reg), arg64_to_string(arg))
            }
        },
        Instr::Lea(reg, mem) => {
            format!("  lea {}, {}", reg_to_string(*reg), mem_ref_to_string(*mem))
        }
        Instr::Rep(op) => format!("  rep {}", str_op_to_string(*op)),
        Instr::Cqo => format!("  cqo"),
    }
}

fn str_op_to_string(op: StrOp) -> String {
    match op {
        StrOp::Stosq => format!("stosq"),
    }
}

pub fn instrs_to_string(is: &[Instr]) -> String {
    let mut buf = String::new();
    for i in is {
        buf.push_str(&instr_to_string(i));
        buf.push('\n');
    }
    buf
}
