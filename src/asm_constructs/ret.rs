use crate::asm_constructs::instruction::{Instruction, StackFrame};

#[derive(Debug)]
pub struct Ret {
}

impl Instruction for Ret {
    fn to_code(&self) -> String {
        String::from("
    movq %rbp, %rsp
    popq %rbp
    ret")
    }

    fn fix_pseudo_registers(&mut self, _pseudo_registers: &mut StackFrame) {
        // nothing to do
    }
}
