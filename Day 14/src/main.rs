mod bitmask;
mod instruction;
mod memory_address_decoder;
mod parse_input;

use bitmask::Bitmask;
use instruction::{Instruction, Instruction::*};
use memory_address_decoder::MemoryAddressDecoder;
use parse_input::parse_input;
use std::collections::HashMap;

fn main() {
    let instructions = parse_input();

    println!(
        "Part 1 - sum of memory values = {}",
        execute_program_v1(&instructions)
    );

    println!(
        "Part 2 - sum of memory values = {}",
        execute_program_v2(&instructions)
    );
}

fn execute_program_v1(instructions: &Vec<Instruction>) -> u64 {
    let mut bitmask = Bitmask::default();
    let mut memory = HashMap::new();

    for instruction in instructions {
        match instruction {
            UpdateBitmask { mask } => bitmask = Bitmask::from_str(&mask),
            WriteValueToMemory { address, value } => {
                memory.insert(address, bitmask.apply(*value));
            }
        }
    }

    memory.values().sum()
}

fn execute_program_v2(instructions: &Vec<Instruction>) -> u64 {
    let mut memory_address_decoder = MemoryAddressDecoder::default();
    let mut memory = HashMap::new();

    for instruction in instructions {
        match instruction {
            UpdateBitmask { mask } => {
                memory_address_decoder = MemoryAddressDecoder::from_str(&mask)
            }
            WriteValueToMemory { address, value } => {
                let decoded_addresses = memory_address_decoder.decode(*address as u64);
                for decoded_address in decoded_addresses {
                    memory.insert(decoded_address, *value);
                }
            }
        }
    }

    memory.values().sum()
}
