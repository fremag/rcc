use crate::ast_model::expression::AstExpression;

#[derive(Debug, Clone)]
pub enum AstStatement {
    Return{expression: AstExpression},
    Expression{ expression: AstExpression},
    Null
}
