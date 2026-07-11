use crate::asm_constructs::operand::Operand;

#[derive(Debug)]
pub struct Pseudo {
    pub identifier : String
}

impl Operand for Pseudo {
    fn to_code(&self) -> String {
        self.identifier.clone()
    }
}