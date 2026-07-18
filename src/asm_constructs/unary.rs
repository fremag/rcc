use std::fmt::{Debug };
use crate::asm_constructs::instruction::{Instruction, StackFrame};
use crate::asm_constructs::operand::Operand;

#[derive(Debug)]
pub enum UnaryOperator {
    Neg, Not
}

#[derive(Debug)]
pub struct Unary {
    unary_operator: UnaryOperator,
    operand: Operand
}

impl Unary {
    pub fn new(unary_operator: UnaryOperator, operand: Operand) -> Self {
        Self { unary_operator, operand }
    }
}

impl Instruction for Unary {
    fn to_code(&self) -> String {
        let unary = match self.unary_operator {
            UnaryOperator::Neg => "neg1",
            UnaryOperator::Not => "not1"
        };
        let operand = self.operand.to_code();
        String::from(format!("{} {}\n", unary, operand))
    }

    fn fix_pseudo_registers(&mut self, _pseudo_registers: &mut StackFrame) {
        let new_operand = self.operand.fix_pseudo_registers(_pseudo_registers);
        if let Some(new_operand) = new_operand {
            self.operand = new_operand;
        }
    }
}