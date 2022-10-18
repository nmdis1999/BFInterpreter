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

    // One pass to find bracket pairs.
    let brackets: Vec<usize> = {
        let mut m = vec![0; prog.len()];
        let mut stack = Vec::new();
        for (idx, ch) in prog.chars().enumerate() {
            match ch {
                '[' => {
                    stack.push(idx);
                }
                ']' => {
                    let s_elem = stack.pop().unwrap();
                    m[s_elem] = idx;
                    m[idx] = s_elem;
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
                    pc = brackets[pc];
                }
            }
            ']' => {
                if memory[dp] != 0 {
                    pc = brackets[pc];
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
