use std::fs;

fn main() {
    println!("Reading file", file_path);
    //Read BF file to string prog
    let prog = fs::read(file_path).expect("Unable to read file");
    //print string length
    println!("{}", file_path.len());

    let memory = Vec::new();
    for _ in 0..40000 {
        memory.push(0);
    }
    let pc = 0;
    let dp = 0;
    let memLength = memory.len();

    loop {
        if pc >=  prog.len() {
            break;
        }
       let instr = prog[pc];
       match instr  {
        '>' => { dp += 1; }
        '<' => { dp -= 1; }
        '+' => { memory[dp] += 1; }
        '-' => { memory[dp] -= 1; }
        '[' => {
           
            }
        ']' => {
               
            }

        '.' => {
                println!(memory[dp])
            }
        ',' => {
               
            }
         _   => {
            //throw error
         }
       }
    }
}


