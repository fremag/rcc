use crate::ast_model::expression::AstExpression;

#[derive(Debug, Clone, PartialEq)]
pub enum AstUnaryOp {
    Negate, BitwiseComplement
}

#[derive(Debug)]
pub struct AstUnaryOperand {
    pub(crate) op: AstUnaryOp,
    pub(crate) exp: Box<AstExpression>
}