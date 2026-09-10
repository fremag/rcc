use crate::ast_model::expression::AstExpression;
use crate::ast_model::statement::AstStatement;

#[derive(Debug, Clone)]
pub struct AstFunction {
    pub(crate) identifier: String,
    pub(crate) body: Vec<AstBlockItem>,
}

#[derive(Debug, Clone)]
pub enum AstBlockItem {
    Statement(AstStatement),
    Declaration(AstDeclaration)
}

#[derive(Debug, Clone)]
pub struct AstDeclaration {
    pub(crate) identifier: String,
    pub(crate) init : Option<AstExpression>
}