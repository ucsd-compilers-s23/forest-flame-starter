use std::{collections::HashSet, env};

type SnekVal = u64;

#[repr(u8)]
enum ErrCode {
    InvalidArgument = 1,
    Overflow = 2,
    IndexOutOfBounds = 3,
    NegativeSize = 4,
    OutOfMemory = 5,
}

const TRUE: u64 = 7;
const FALSE: u64 = 3;

#[link(name = "our_code")]
extern "C" {
    // The \x01 here is an undocumented feature of LLVM that ensures
    // it does not add an underscore in front of the name.
    // Courtesy of Max New (https://maxsnew.com/teaching/eecs-483-fa22/hw_adder_assignment.html)
    #[link_name = "\x01our_code_starts_here"]
    fn our_code_starts_here(input: u64, heap_start: *mut u64, heap_end: *mut u64) -> u64;
}

#[export_name = "\x01snek_error"]
pub extern "C" fn snek_error(errcode: i32) {
    if errcode == ErrCode::InvalidArgument as i32 {
        eprintln!("invalid argument");
    } else if errcode == ErrCode::Overflow as i32 {
        eprintln!("overflow");
    } else if errcode == ErrCode::IndexOutOfBounds as i32 {
        eprintln!("index out of bounds");
    } else if errcode == ErrCode::NegativeSize as i32 {
        eprintln!("vector size must be non-negative");
    } else {
        eprintln!("an error ocurred {errcode}");
    }
    std::process::exit(errcode);
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

unsafe fn try_gc(
    heap_start: *mut u64,
    heap_end: *mut u64,
    heap_ptr: *mut u64,
    count: usize,
) -> *mut u64 {
    eprintln!("out of memory");
    std::process::exit(ErrCode::OutOfMemory as i32)
}

#[export_name = "\x01snek_alloc_vec"]
pub unsafe extern "C" fn snek_alloc_vec(
    heap_start: *mut u64,
    heap_end: *mut u64,
    heap_ptr: *mut u64,
    count: usize,
    elem: SnekVal,
) -> *mut u64 {
    let heap_ptr = if heap_ptr.offset_from(heap_start) >= count as isize + 2 {
        heap_ptr.sub(count + 2)
    } else {
        try_gc(heap_start, heap_end, heap_ptr, count)
    };
    heap_ptr.add(1).write(count as u64);
    for i in 0..count {
        heap_ptr.add(2 + i).write(elem);
    }
    heap_ptr
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

    let mut heap: Vec<u64> = Vec::with_capacity(heap_size);
    let heap_start = heap.as_mut_ptr();
    let heap_end = unsafe { heap_start.offset(heap_size as isize) };

    let i: u64 = unsafe { our_code_starts_here(input, heap_start, heap_end) };
    unsafe { snek_print(i) };
}
