use crate::ast_model::constant::AstConstant;
use crate::ast_model::unary::AstUnaryOp;

#[derive(Debug, Clone)]
pub enum AstFactor {
    Constant {
        constant: AstConstant,
    },
    Unary {
        unary_op: AstUnaryOp,
        factor: Box<AstFactor>,
    },
    Nested(Box<AstExpression>),
}

#[derive(Debug, Clone)]
pub enum AstExpression {
    Factor(AstFactor),
    Binary {
        left: Box<AstExpression>,
        binop: BinaryOp,
        right: Box<AstExpression>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinaryOp {Add, Sub, Mul, Div, Modulo}