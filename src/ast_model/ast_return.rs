use crate::ast_model::expression::AstExpression;

#[derive(Debug)]
pub struct AstReturn {
    pub(crate) expression: AstExpression
}