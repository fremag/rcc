use crate::ast_model::function::AstFunction;

#[derive(Debug)]
pub struct AstProgram {
    pub(crate) function: AstFunction
}