use crate::asm_constructs::imm::Imm;
use crate::asm_constructs::operand::Operand;
use crate::ast_model::unary::AstUnaryOp;
use crate::ast_model::constant::AstConstant;
use crate::ast_model::unary::AstUnaryOperand;

#[derive(Debug, Clone)]
pub enum AstExpression {
    Constant(AstConstant),
    Unary(AstUnaryOp, Box<AstExpression>)
}

impl AstExpression {
    pub fn to_asm(&self) -> Box<dyn Operand> {
        match self {
            AstExpression::Constant(cst) => Box::new(Imm { value: cst.value }),
            AstExpression::Unary(op, exp) => {
                Box::new(AstUnaryOperand {
                    op: op.clone(),
                    exp: exp.clone(),
                })
            }
        }
    }
}