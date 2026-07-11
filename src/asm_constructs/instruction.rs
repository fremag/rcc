use crate::asm_constructs::mov::Mov;
use crate::asm_constructs::asm_return::AsmReturn;

pub trait Instruction: std::fmt::Debug {
    fn to_code(&self) -> String;
}

impl Instruction for AsmReturn {
    fn to_code(&self) -> String {
        String::from("ret")
    }
}

impl Instruction for Mov {
    fn to_code(&self) -> String {
        format!("movl {}, {}", self.src.to_code(), self.dest.to_code())
    }
}