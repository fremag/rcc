use crate::ast_model::constant::AstConstant;
use crate::ast_model::unary::AstUnaryOp;

#[derive(Debug, Clone)]
pub enum AstExpression {
    Constant { constant: AstConstant },
    Unary { unary_op: AstUnaryOp, expression: Box<AstExpression> }
}