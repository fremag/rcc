use std::fmt::{Debug };
use crate::asm_constructs::instruction::Instruction;
use crate::asm_constructs::operand::Operand;

#[derive(Debug)]
pub enum UnaryOperator {
    Neg, Not
}

#[derive(Debug)]
struct Unary {
    unary_operator: UnaryOperator,
    operand: Box<dyn Operand>
}

impl Instruction for Unary {
    fn to_code(&self) -> String {
        String::from("")
    }
}