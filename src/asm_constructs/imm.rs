use crate::asm_constructs::instruction::StackFrame;
use crate::asm_constructs::operand::Operand;

#[derive(Debug)]
pub struct Imm {
    pub(crate) value: i32
}

impl Operand for Imm {
    fn to_code(&self) -> String {
        String::from(format!("${}", self.value))
    }

    fn fix_pseudo_registers(&mut self, _pseudo_registers: &mut StackFrame) -> Option<Box<dyn Operand>> {
        None
    }
}
