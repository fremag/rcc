use crate::ast_model::expression::AstExpression;
use crate::ast_model::function::AstBlock;

#[derive(Debug, Clone)]
pub enum AstStatement {
    Return{expression: AstExpression},
    Expression{ expression: AstExpression},
    If{expression: AstExpression, then_statement : Box<AstStatement>, else_statement: Option<Box<AstStatement>>},
    Compound{block: AstBlock},
    Null,
    Label { label: String, statement: Box<AstStatement> },
    Goto { target: String },
}
