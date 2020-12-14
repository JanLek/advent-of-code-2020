#[derive(Debug)]
pub enum Instruction {
    UpdateBitmask { mask: String },
    WriteValueToMemory { address: usize, value: u64 },
}
