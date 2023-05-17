use std::{collections::HashSet, env};

type SnekVal = u64;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(i64)]
pub enum ErrCode {
    InvalidArgument = 1,
    Overflow = 2,
    IndexOutOfBounds = 3,
    InvalidVecSize = 4,
    OutOfMemory = 5,
}

const TRUE: u64 = 7;
const FALSE: u64 = 3;

static mut HEAP_START: *const u64 = std::ptr::null();
static mut HEAP_END: *const u64 = std::ptr::null();
static mut HEAP_PTR: *const u64 = std::ptr::null();

#[link(name = "our_code")]
extern "C" {
    // The \x01 here is an undocumented feature of LLVM that ensures
    // it does not add an underscore in front of the name.
    // Courtesy of Max New (https://maxsnew.com/teaching/eecs-483-fa22/hw_adder_assignment.html)
    #[link_name = "\x01our_code_starts_here"]
    fn our_code_starts_here(input: u64) -> u64;
}

#[export_name = "\x01snek_error"]
pub extern "C" fn snek_error(errcode: ErrCode) {
    if errcode == ErrCode::InvalidArgument {
        eprintln!("invalid argument");
    } else if errcode == ErrCode::Overflow {
        eprintln!("overflow");
    } else if errcode == ErrCode::IndexOutOfBounds {
        eprintln!("index out of bounds");
    } else if errcode == ErrCode::InvalidVecSize {
        eprintln!("vector size must be non-negative");
    } else {
        eprintln!("an error ocurred {}", errcode as i32);
    }
    std::process::exit(errcode as i32);
}

unsafe fn snek_str(val: SnekVal, seen: &mut HashSet<SnekVal>) -> String {
    if val == TRUE {
        format!("true")
    } else if val == FALSE {
        format!("false")
    } else if val & 1 == 0 {
        format!("{}", (val as i64) >> 1)
    } else if val & 1 == 1 {
        if !seen.insert(val) {
            return "[...]".to_string();
        }
        let addr = (val - 1) as *const u64;
        let size = addr.add(1).read() as usize;
        let mut res = "[".to_string();
        for i in 0..size {
            let elem = addr.add(2 + i).read();
            res = res + &snek_str(elem, seen);
            if i < size - 1 {
                res = res + ", ";
            }
        }
        res + "]"
    } else {
        format!("unknown value: {val}")
    }
}

#[export_name = "\x01snek_print"]
pub unsafe extern "C" fn snek_print(val: SnekVal) -> SnekVal {
    println!("{}", snek_str(val, &mut HashSet::new()));
    val
}

unsafe fn try_gc(count: usize, stack_start: *const u64, stack_end: *const u64) -> *const u64 {
    eprintln!("out of memory");
    std::process::exit(ErrCode::OutOfMemory as i32)
}

unsafe fn alloc(count: usize, stack_start: *const u64, stack_end: *const u64) -> *mut u64 {
    // Allocate 1 extra word to store GC metadata
    HEAP_PTR = if HEAP_PTR.offset_from(HEAP_START) >= count as isize + 1 {
        HEAP_PTR.sub(count + 1)
    } else {
        try_gc(count, stack_start, stack_end)
    };
    let ptr = HEAP_PTR as *mut u64;

    // Write GC metadata
    ptr.write(0);

    ptr
}

#[export_name = "\x01snek_alloc_vec"]
pub unsafe extern "C" fn snek_alloc_vec(
    size: SnekVal,
    elem: SnekVal,
    stack_start: *const u64,
    stack_end: *const u64,
) -> SnekVal {
    // Check the size is a number
    if size & 1 != 0 {
        snek_error(ErrCode::InvalidArgument);
    }
    // Check the size is non-negative
    if size < 0 {
        snek_error(ErrCode::InvalidVecSize);
    }
    let size = (size >> 1) as usize;

    // Allocate `size + 1` 8 byte words to account for size of the vector
    let ptr = alloc(size + 1, stack_start, stack_end) as *mut u64;

    // Write size of the vector and fill it with the given element
    ptr.add(1).write(size as u64);
    for i in 0..size {
        ptr.add(2 + i).write(elem);
    }
    (ptr as u64) ^ 1
}

#[export_name = "\x01snek_print_stack"]
unsafe fn snek_print_stack(stack_start: *const u64, stack_end: *const u64) {
    let mut ptr = stack_start;
    println!("-----------------------------------------");
    while ptr >= stack_end {
        let val = *ptr;
        println!("{ptr:?}: {:#0x}", val);
        ptr = ptr.sub(1);
    }
    println!("-----------------------------------------");
}

fn parse_input(input: &str) -> u64 {
    match input {
        "true" => TRUE,
        "false" => FALSE,
        _ => (input.parse::<i64>().unwrap() << 1) as u64,
    }
}

fn parse_heap_size(input: &str) -> usize {
    input.parse::<usize>().unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() >= 2 { &args[1] } else { "false" };
    let heap_size = if args.len() >= 3 { &args[2] } else { "10000" };
    let input = parse_input(&input);
    let heap_size = parse_heap_size(&heap_size);

    // Initialize heap
    let mut heap: Vec<u64> = Vec::with_capacity(heap_size);
    unsafe {
        HEAP_START = heap.as_mut_ptr();
        HEAP_END = HEAP_START.add(heap_size);
        HEAP_PTR = HEAP_END;
    }

    let i: u64 = unsafe { our_code_starts_here(input) };
    unsafe { snek_print(i) };
}
