use crate::asm_constructs::instruction::Instruction;

#[derive(Debug)]
pub struct Ret {
}

impl Instruction for Ret {
    fn to_code(&self) -> String {
        String::from("ret")
    }
}
