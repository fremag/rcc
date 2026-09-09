use crate::ast_model::constant::AstConstant;

#[derive(Debug, Clone)]
pub enum AstExpression {
    Constant {
        constant: AstConstant,
    },
    Var{ identifier: String },
    Unary {
        unary_op: AstUnaryOp,
        factor: Box<AstExpression>,
    },
    Binary {
        left: Box<AstExpression>,
        binop: AstBinaryOp,
        right: Box<AstExpression>,
    },
    Assignment { 
        left: Box<AstExpression>, 
        right: Box<AstExpression> 
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
