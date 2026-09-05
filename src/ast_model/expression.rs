use crate::ast_model::constant::AstConstant;

#[derive(Debug, Clone)]
pub enum AstExpression {
    Binary {
        left: Box<AstExpression>,
        binop: AstBinaryOp,
        right: Box<AstExpression>,
    },
    Constant {
        constant: AstConstant,
    },
    Unary {
        unary_op: AstUnaryOp,
        factor: Box<AstExpression>,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AstBinaryOp {Add, Sub, Mul, Div, Mod, And, Or, Equal, NotEqual, LessThan, LessThanEqual, GreaterThan, GreaterThanEqual }

#[derive(Debug, Clone, PartialEq)]
pub enum AstUnaryOp {
    Negate,
    BitwiseComplement,
    Not
}
