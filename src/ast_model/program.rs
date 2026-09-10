use crate::ast_model::function::AstFunction;

#[derive(Debug, Clone)]
pub struct AstProgram {
    pub(crate) function: AstFunction,
}
