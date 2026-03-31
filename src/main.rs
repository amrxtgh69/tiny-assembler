use std::fs;
use tiny_assembler::assemble_line;

fn main() {
    let input = fs::read_to_string("input.s").unwrap();

    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let instr = assemble_line(line);
        println!("{:#010x}", instr); 
    }
}




