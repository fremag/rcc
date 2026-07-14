use std::fmt::Debug;
use crate::asm_constructs::instruction::{Instruction, StackFrame};

#[derive(Debug)]
pub struct AllocateStack {
    size: i32
}

impl Instruction for AllocateStack {
    fn to_code(&self) -> String {
        String::from(format!("sub rsp, {}", self.size))
    }

    fn fix_pseudo_registers(&mut self, _pseudo_registers: &mut StackFrame) {
        // nothing to do
    }
}