use crate::asm_constructs::asm_return::AsmReturn;
use crate::asm_constructs::instruction::Instruction;
use crate::asm_constructs::mov::Mov;
use crate::asm_constructs::register::Register;
use crate::ast_model::expression::AstExpression;

#[derive(Debug)]
pub struct AstReturn {
    pub(crate) expression: AstExpression
}

impl AstReturn {
    pub(crate) fn to_asm(&self) -> Vec<Box<dyn Instruction>> {
        let mut instructions : Vec<Box<dyn Instruction>> = Vec::new();
        let mov = Mov{
            src: self.expression.to_asm(),
            dest: Box::new(Register{})
        };

        instructions.push(Box::new(mov));
        instructions.push(Box::new(AsmReturn{}));
        instructions
    }
}