use std::fmt;
use std::fmt::Formatter;
use crate::asm_constructs::operand::Operand;

pub struct Imm {
    pub(crate) value: i32
}

impl Operand for Imm {
    fn to_code(&self) -> String {
        String::from(format!("${}", self.value))
    }
}

impl fmt::Debug for Imm {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Imm")
            .field("value", &self.value)
            .finish()
    }
}