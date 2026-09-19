use crate::ast_model::expression::AstExpression;

#[derive(Debug, Clone)]
pub enum AstStatement {
    Return{expression: AstExpression},
    Expression{ expression: AstExpression},
    If{expression: AstExpression, then_statement : Box<AstStatement>, else_statement: Option<Box<AstStatement>>},
    Null
}
