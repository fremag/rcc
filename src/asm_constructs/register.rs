use crate::asm_constructs::instruction::StackFrame;
use crate::asm_constructs::operand::Operand;

#[derive(Debug)]
pub enum Reg {
    AX, R10
}

#[derive(Debug)]
pub struct Register {
    pub(crate) reg: Reg
}

impl Operand for Register {
    fn to_code(&self) -> String {
        match self.reg {
            Reg::AX => String::from("%eax"),
            Reg::R10 => String::from("%r10d")
        }
    }

    fn fix_pseudo_registers(&mut self, _pseudo_registers: &mut StackFrame) -> Option<Box<dyn Operand>> {
        None
    }
}