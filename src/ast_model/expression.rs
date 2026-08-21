use crate::ast_model::constant::AstConstant;

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
        binop: AstBinaryOp,
        right: Box<AstExpression>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AstBinaryOp {Add, Sub, Mul, Div, Mod, And, Or, Equal, NotEqual, LessThan, LessThanEqual, GreaterThan, GreaterThanEqual }

#[derive(Debug, Clone, PartialEq)]
pub enum AstUnaryOp {
    Negate,
    BitwiseComplement,
    Not
}
