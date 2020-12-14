use crate::instruction::Instruction;
use regex::Regex;

const INPUT: &str = include_str!("input.txt");

pub fn parse_input() -> Vec<Instruction> {
    let mask_pattern = Regex::new(r"mask = (?P<mask>[X10]{36})").unwrap();
    let write_value_pattern = Regex::new(r"mem\[(?P<address>\d+)\] = (?P<value>\d+)").unwrap();

    INPUT
        .split("\n")
        .map(|line| {
            if let Some(captures) = mask_pattern.captures(line) {
                Instruction::UpdateBitmask {
                    mask: captures.name("mask").unwrap().as_str().to_string(),
                }
            } else {
                let captures = write_value_pattern.captures(line).unwrap();
                Instruction::WriteValueToMemory {
                    address: captures.name("address").unwrap().as_str().parse().unwrap(),
                    value: captures.name("value").unwrap().as_str().parse().unwrap(),
                }
            }
        })
        .collect()
}
