use std::env;

#[link(name = "our_code")]
extern "C" {
    // The \x01 here is an undocumented feature of LLVM that ensures
    // it does not add an underscore in front of the name.
    // Courtesy of Max New (https://maxsnew.com/teaching/eecs-483-fa22/hw_adder_assignment.html)
    #[link_name = "\x01our_code_starts_here"]
    fn our_code_starts_here(input: u64) -> u64;
}

#[export_name = "\x01snek_error"]
pub extern "C" fn snek_error(errcode: i32) {
    if errcode == 1 {
        eprintln!("invalid argument");
    } else if errcode == 2 {
        eprintln!("overflow");
    } else {
        eprintln!("an error ocurred {errcode}");
    }
    std::process::exit(errcode);
}

#[export_name = "\x01snek_print"]
pub extern "C" fn snek_print(i: u64) -> u64 {
    if i == 3 {
        println!("true");
    } else if i == 1 {
        println!("false");
    } else if i & 1 == 0 {
        println!("{}", (i as i64) >> 1);
    } else {
        println!("Unknown value: {i}")
    }
    i
}

fn parse_input(input: &str) -> u64 {
    match input {
        "true" => 3,
        "false" => 1,
        _ => (input.parse::<i64>().unwrap() << 1) as u64,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() == 2 { &args[1] } else { "false" };
    let input = parse_input(&input);

    let i: u64 = unsafe { our_code_starts_here(input) };
    snek_print(i);
}
