use crate::ast_model::program::AstProgram;

pub struct Resolver {}

impl Resolver {
    pub(crate) fn resolve(&self, ast_program: &AstProgram) -> AstProgram {
        ast_program.clone()
    }
}

impl Resolver {
    pub(crate) fn new() -> Self {
        Self {}
    }
}
