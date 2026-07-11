use crate::asm_constructs::instruction::Instruction;
use crate::ast_model::ast_return::AstReturn;

#[derive(Debug)]
pub struct AstStatement {
    pub(crate) return_exp: AstReturn
}

impl AstStatement {
    pub(crate) fn to_asm(&self) -> Vec<Box<dyn Instruction>> {
        let mut instructions = Vec::new();
        let exp_instructions = self.return_exp.to_asm();
        for instruction in exp_instructions {
            instructions.push(instruction);
        }
        instructions
    }
}