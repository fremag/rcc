use crate::asm_constructs::instruction::StackFrame;
use crate::asm_constructs::operand::Operand;
use crate::asm_constructs::stack::Stack;

#[derive(Debug)]
pub struct Pseudo {
    pub identifier : String
}

impl Operand for Pseudo {
    fn to_code(&self) -> String {
        self.identifier.clone()
    }

    fn fix_pseudo_registers(&mut self, _pseudo_registers: &mut StackFrame) -> Option<Box<dyn Operand>> {
        let offset = _pseudo_registers.get(&self.identifier);
        let stack = Stack { offset };
        Some(Box::new(stack))
    }
}