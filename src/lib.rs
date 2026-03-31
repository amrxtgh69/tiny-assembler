pub fn assemble_line(line: &str) -> u32 {
    let clean = line.replace(",", "");

    let tokens: Vec<&str> = clean.split_whitespace().collect();
    if tokens.len() != 4 {
        panic!("invalid instruction format");
    }

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
        "and" => encode_r(
            0b0000000,
            reg(tokens[3]),
            reg(tokens[2]),
            0b111,
            reg(tokens[1]),
            0b0110011,
        ),
        "or" => encode_r(
            0b0000000,
            reg(tokens[3]),
            reg(tokens[2]),
            0b110,
            reg(tokens[1]),
            0b0110011,
        ),
        _ => panic!("unknown instruction"),
    }
}

fn encode_r(func7: u32, rs2: u32, rs1: u32, func3: u32, rd: u32, opcode: u32) -> u32 {
    (func7 << 25) | (rs2 << 20) | (rs1 << 15) | (func3 << 12) | (rd << 7) | opcode
}
fn reg(r: &str) -> u32 {
    if let Some(num) = r.strip_prefix('x') {
        num.parse::<u32>().unwrap()
    } else {
        panic!("invalid register");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(assemble_line("add x1, x2, x3"), 0x003100b3);
    }

    #[test]
    fn test_sub() {
        assert_eq!(assemble_line("sub x4, x5, x6"), 0x40628233);
    }

    #[test]
    fn test_and() {
        assert_eq!(assemble_line("and x7, x8, x9"), 0x009473b3);
    }

    #[test]
    fn test_or() {
        assert_eq!(assemble_line("or x10, x11, x12"), 0x00c5e533);
    }
}
