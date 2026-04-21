use std::{fs::{self, File}, io::Write};
use tiny_assembler::assemble_line;

fn main() {
    let input = fs::read_to_string("input.s").unwrap();

    let mut file = File::create("program.bin").unwrap();
    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let instr = assemble_line(line);
        // println!("{:#010x}", instr); 
        
        // i love rust straight to little endian bytes hehe :))
        let bytes = instr.to_le_bytes();
        file.write_all(&bytes).unwrap();
    }
}




