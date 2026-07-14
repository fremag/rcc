use crate::asm_constructs::instruction::StackFrame;

pub trait Operand: std::fmt::Debug {
    fn to_code(&self) -> String;
    fn fix_pseudo_registers(&mut self, _pseudo_registers: &mut StackFrame) -> Option<Box<dyn Operand>>;
}