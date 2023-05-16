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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JmpArg {
    Label(String),
    Reg(Reg),
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
    And(BinArgs),
    Or(BinArgs),
    Xor(BinArgs),
    Shr(BinArgs),
    Sar(BinArgs),
    Shl(BinArgs),
    Cmp(BinArgs),
    Test(BinArgs),

    Push(Arg32),
    Pop(Loc),

    Label(String),

    Call(JmpArg),
    Ret,

    Jmp(JmpArg),
    Je(JmpArg),
    Jne(JmpArg),
    Jl(JmpArg),
    Jle(JmpArg),
    Jg(JmpArg),
    Jge(JmpArg),

    Js(JmpArg),  // jump if msb is 1
    Jz(JmpArg),  // jump if result was 0
    Jnz(JmpArg), // jump if result was not 0

    Jo(JmpArg),  // jump if last arith operation overflowed
    Jno(JmpArg), // jump if last arith operation didn't overflow

    Lea(Reg, MemRef),

    Comment(String),
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
        "QWORD [{} + {}]",
        reg_to_string(m.reg),
        offset_to_string(m.offset)
    )
}

fn offset_to_string(off: Offset) -> String {
    match off {
        Offset::Constant(n) => format!("{}", n),
        Offset::Computed {
            reg,
            factor,
            constant,
        } => format!("{} * {} + {}", reg_to_string(reg), factor, constant),
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

pub fn jmp_arg_to_string(arg: &JmpArg) -> String {
    match arg {
        JmpArg::Label(s) => s.clone(),
        JmpArg::Reg(r) => reg_to_string(*r),
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
        Instr::Comment(s) => format!(";; {}\n", s),
        Instr::Mov(args) => format!("  mov {}", mov_args_to_string(args)),
        Instr::Add(args) => format!("  add {}", bin_args_to_string(*args)),
        Instr::Sub(args) => format!("  sub {}", bin_args_to_string(*args)),
        Instr::IMul(args) => format!("  imul {}", bin_args_to_string(*args)),
        Instr::And(args) => format!("  and {}", bin_args_to_string(*args)),
        Instr::Or(args) => format!("  or {}", bin_args_to_string(*args)),
        Instr::Xor(args) => format!("  xor {}", bin_args_to_string(*args)),
        Instr::Shr(args) => format!("  shr {}", bin_args_to_string(*args)),
        Instr::Shl(args) => format!("  shl {}", bin_args_to_string(*args)),
        Instr::Sar(args) => format!("  sar {}", bin_args_to_string(*args)),
        Instr::Cmp(args) => format!("  cmp {}", bin_args_to_string(*args)),
        Instr::Test(args) => format!("  test {}", bin_args_to_string(*args)),
        Instr::Push(arg) => format!("  push {}", arg32_to_string(*arg)),
        Instr::Pop(loc) => format!("  pop {}", loc_to_string(*loc)),
        Instr::Label(s) => format!("{}:", s),

        Instr::Call(s) => format!("  call {}", jmp_arg_to_string(s)),
        Instr::Ret => format!("  ret"),
        Instr::Jmp(s) => format!("  jmp {}", jmp_arg_to_string(s)),
        Instr::Je(s) => format!("  je {}", jmp_arg_to_string(s)),
        Instr::Jne(s) => format!("  jne {}", jmp_arg_to_string(s)),
        Instr::Jle(s) => format!("  jle {}", jmp_arg_to_string(s)),
        Instr::Jl(s) => format!("  jl {}", jmp_arg_to_string(s)),
        Instr::Jg(s) => format!("  jg {}", jmp_arg_to_string(s)),
        Instr::Jge(s) => format!("  jge {}", jmp_arg_to_string(s)),
        Instr::Js(s) => format!("  js {}", jmp_arg_to_string(s)),
        Instr::Jz(s) => format!("  jz {}", jmp_arg_to_string(s)),
        Instr::Jnz(s) => format!("  jnz {}", jmp_arg_to_string(s)),
        Instr::Jo(s) => format!("  jo {}", jmp_arg_to_string(s)),
        Instr::Jno(s) => format!("  jno {}", jmp_arg_to_string(s)),
        Instr::CMov(cmov) => match cmov {
            CMov::E(reg, arg) => {
                format!("  cmove {}, {}", reg_to_string(*reg), arg64_to_string(arg))
            }
            CMov::Z(reg, arg) => {
                format!("  cmovz {}, {}", reg_to_string(*reg), arg64_to_string(arg))
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
