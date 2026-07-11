use std::fmt;
use crate::asm_constructs::function::FunctionDefinition;

pub struct AsmProgram {
    pub(crate) function_definition: FunctionDefinition
}

impl AsmProgram {
    pub(crate) fn to_code(&self) -> String {
        let mut asm_code = self.function_definition.to_code();
        asm_code += "\t.section .note.GNU-stack,\"\",@progbits\n";
        asm_code
    }
}

impl fmt::Debug for AsmProgram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Program")
            .field("function_definition", &self.function_definition)
            .finish()
    }
}