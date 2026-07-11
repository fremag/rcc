use crate::asm_constructs::register::Register;

pub trait Operand: std::fmt::Debug {
    fn to_code(&self) -> String;
}

impl Operand for Register {
    fn to_code(&self) -> String {
        String::from("%eax")
    }
}