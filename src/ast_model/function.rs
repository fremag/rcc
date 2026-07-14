use crate::ast_model::statement::AstStatement;

#[derive(Debug)]
pub struct AstFunction {
    pub(crate) identifier: String,
    pub(crate) body : AstStatement
}
