use crate::asm_constructs::imm::Imm;
use crate::asm_constructs::operand::Operand;
use crate::ast_model::unary::AstUnaryOp;
use crate::ast_model::constant::AstConstant;
use crate::ast_model::unary::AstUnaryOperand;

#[derive(Debug, Clone)]
pub enum AstExpression {
    Constant { constant: AstConstant },
    Unary { unary_op: AstUnaryOp, expression: Box<AstExpression> }
}

impl AstExpression {
    pub fn to_asm(&self) -> Box<dyn Operand> {
        match self {
            AstExpression::Constant { constant: cst } => Box::new(Imm { value: cst.value }),
            AstExpression::Unary { unary_op: op, expression: exp } => {
                Box::new(AstUnaryOperand {
                    op: op.clone(),
                    exp: exp.clone(),
                })
            }
        }
    }
}