use std::fs;
use std::process;

fn main() {
    let file_path = "hello_world.bf";
    println!("Reading file ...{}", file_path);
    //Read BF file to string prog
    let prog = fs::read_to_string(file_path).expect("Unable to read file");
    // print string length
    // println!("{}", file_path.len());

    let mut memory = Vec::new(); // without mut rust throw warning that vector is not mutable -> will read on it
    for _ in 0..40000 {
        memory.push(0);
    }
    let mut pc = 0;
    let mut dp = 0;

    let mut stack: Vec<usize> = Vec::new();

    loop {
        if pc >= prog.len() {
            break;
        }
        let instr: u8 = prog.as_bytes()[pc];
        let instrchar: char = instr as char;

        match instrchar {
            '>' => {
                dp += 1;
                if dp >= memory.len() {
                    memory.push(0); //growing memory if needed
                }
            }
            '<' => {
                if dp == 0 {
                    eprintln!("pointer was at 0");
                    process::exit(1);
                }
                dp -= 1;
            }
            '+' => {
                memory[dp] += 1;
            }
            '-' => {
                memory[dp] -= 1;
            }
            '[' => {
                if memory[dp] == 0 {
                    // find ']'
                    while prog.as_bytes()[pc] as char != ']' {
                        pc += 1;
                        if pc >= prog.len() {
                            eprintln!("no matching ']' for '['");
                            process::exit(1);
                        }
                    }
                } else {
                    stack.push(pc);
                }
            }
            ']' => {
                // jump back to start of loop  if memory[dp] is not zero
                if memory[dp] != 0 {
                    pc = stack.pop().unwrap_or_else(|| {
                        eprintln!("cannot find balancing opening bracket");
                        process::exit(1);
                    }) - 1; // sutract 1 so to re-process the '[' and add to stack
                } else {
                    // balanced so remove prev bracket
                    stack.pop();
                }
            }
            '.' => {
                let c = char::from_u32(memory[dp]).unwrap();
                print!("{}", c);
            }
            // ',' => {}
            _ => {
                eprintln!("unexpected character in bf program");
                process::exit(1);
            }
        }
        pc += 1;
    }
}
