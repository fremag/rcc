use crate::ast_model::expression::AstExpression;
use crate::ast_model::statement::AstStatement;

#[derive(Debug)]
pub struct AstFunction {
    pub(crate) identifier: String,
    pub(crate) body: Vec<AstStatement>,
}

#[derive(Debug)]
pub enum AstBlockItem {
    Statement(AstStatement),
    Declaration(AstDeclaration)
}

#[derive(Debug)]
pub struct AstDeclaration {
    pub(crate) identifier: String,
    pub(crate) init : Option<AstExpression>
}