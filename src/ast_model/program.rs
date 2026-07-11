use crate::asm_constructs::program::AsmProgram;
use crate::ast_model::function::AstFunction;

#[derive(Debug)]
pub struct AstProgram {
    pub(crate) function: AstFunction
}

impl AstProgram {
    pub fn to_asm(&self) -> AsmProgram {
        AsmProgram{
            function_definition: self.function.to_asm(),
        }
    }
}