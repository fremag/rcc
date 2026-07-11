use crate::asm_constructs::instruction::Instruction;
use crate::asm_constructs::operand::Operand;

#[derive(Debug)]
pub struct Mov {
    pub(crate) src : Box<dyn Operand>,
    pub(crate) dest : Box<dyn Operand>
}

impl Instruction for Mov {
    fn to_code(&self) -> String {
        format!("movl {}, {}", self.src.to_code(), self.dest.to_code())
    }
}