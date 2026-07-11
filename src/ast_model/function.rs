use crate::asm_constructs::function::FunctionDefinition;
use crate::ast_model::statement::AstStatement;

#[derive(Debug)]
pub struct AstFunction {
    pub(crate) identifier: String,
    pub(crate) body : AstStatement
}
impl AstFunction {
    pub(crate) fn to_asm(&self) -> FunctionDefinition {
        FunctionDefinition {
            identifier: self.identifier.clone(),
            instructions: self.body.to_asm(),
        }
    }
}
