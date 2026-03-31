use std::fs;
use core::panic;

fn main() {
    let input = fs::read_to_string("input.s").expect("failed to read file.");

    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let instr = assemble_line(line);
        println!("{:010x}", instr); 
    }
}

fn assemble_line(line: &str) -> u32 {
    let clean = line.replace(",", "");

    let tokens: Vec<&str> = clean.split_whitespace().collect();

    let op = tokens[0];
    match op {
        "add" => encode_r(
            0b0000000,
            reg(tokens[3]),
            reg(tokens[2]),
            0b000,
            reg(tokens[1]),
            0b0110011,
        ),
        "sub" => encode_r(
            0b0100000,
            reg(tokens[3]),
            reg(tokens[2]),
            0b000,
            reg(tokens[1]),
            0b0110011,
        ),
        _ => panic!("unknown instruction"),
    }
}

fn reg(r: &str) -> u32 {
    if let Some(num) = r.strip_prefix('x') {
        num.parse::<u32>().unwrap()
    } else {
        panic!("invalid register");
    }
}

fn encode_r(
    func7: u32,
    rs2: u32,
    rs1: u32,
    func3: u32,
    rd: u32,
    opcode: u32,
    ) -> u32 {
    (func7 << 25)
        | (rs2 << 20)
        | (rs1 << 15)
        | (func3 << 12)
        | (rd << 7)
        | opcode
}

