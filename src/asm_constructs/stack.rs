use crate::asm_constructs::instruction::StackFrame;
use crate::asm_constructs::operand::Operand;

#[derive(Debug)]
pub struct Stack {
    pub offset: usize
}

impl Operand for Stack {
    fn to_code(&self) -> String {
        todo!()
    }

    fn fix_pseudo_registers(&mut self, _pseudo_registers: &mut StackFrame) -> Option<Box<dyn Operand>> {
        None
    }
}