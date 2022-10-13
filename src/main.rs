use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;
use std::io::Read;

static MAX_LEN: u32 = 40000;
fn zeros(size: u32) -> Vec<u32> {
    vec![0; size as usize]
}
fn main() {
    let file_path = env::args().nth(1).expect("Please pass *.bf input");
    println!("Reading file ...{}", file_path);
    //Read BF file to string prog
    let prog = fs::read_to_string(file_path).expect("Unable to read file");
    // without mut rust throw warning that vector is not mutable -> will read on it
    let mut memory = zeros(MAX_LEN);
    let mut pc = 0;
    let mut dp = 0;

    let mut stack: Vec<usize> = Vec::new();

    // One pass to find bracket pairs.
    let brackets: HashMap<usize, usize> = {
        let mut m = HashMap::new();
        let mut scope_stack = Vec::new();
        for (idx, ch) in prog.chars().enumerate() {
            match ch {
                '[' => {
                    scope_stack.push(idx);
                }
                ']' => {
                    m.insert(scope_stack.pop().unwrap(), idx);
                }
                _ => { /* ignore */ }
            }
        }

        m
    };

    loop {
        if pc >= prog.len() {
            break;
        }
        let instr: u8 = prog.as_bytes()[pc];
        let instrchar: char = instr as char;

        match instrchar {
            '>' => {
                if dp >= memory.len() {
                    memory.push(0); //growing memory if needed
                }
                dp += 1;
            }
            '<' => {
                if dp > 0 {
                    dp -= 1;
                }
            }
            '+' => {
                memory[dp] = memory[dp].wrapping_add(1);
                if memory[dp] > 255 {
                    memory[dp] = 0;
                }
            }
            '-' => {
                memory[dp] = memory[dp].wrapping_sub(1);
            }
            '[' => {
                if memory[dp] == 0 {
                    pc = brackets[&pc];
                } else {
                    stack.push(pc);
                }
            }
            ']' => {
                let matching_bracket = stack.pop().unwrap();
                if memory[dp] != 0 {
                    pc = matching_bracket - 1;
                }
            }
            '.' => {
                let c = char::from_u32(memory[dp]).unwrap();
                print!("{}", c);
            }
            ',' => {
                let mut input = [0];
                io::stdin()
                    .read_exact(&mut input)
                    .expect("error reading user input");
                memory[dp] = input[0] as u32;
            }
            _ => {
                //ignore
            }
        }
        pc += 1;
    }
}
