use std::fmt::Debug;
use crate::asm_constructs::instruction::Instruction;

#[derive(Debug)]
pub struct AllocateStack {
    size: i32
}

impl Instruction for AllocateStack {
    fn to_code(&self) -> String {
        String::from(format!("sub rsp, {}", self.size))
    }
}