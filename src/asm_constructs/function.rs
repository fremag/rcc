use std::fmt;
use std::fmt::Formatter;
use crate::asm_constructs::instruction::Instruction;

pub struct FunctionDefinition {
    pub(crate) identifier: String,
    pub(crate) instructions : Vec<Box<dyn Instruction>>
}

impl FunctionDefinition {
    pub(crate) fn to_code(&self) -> String {
        let mut asm_code = format!("\t.globl {}\n{}:\n", self.identifier, self.identifier);
        for instruction in &self.instructions {
            let inst_code = instruction.to_code();
            asm_code += format!("\t{}\n", inst_code).as_str();
        }

        asm_code
    }
}

impl fmt::Debug for FunctionDefinition {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("FunctionDefinition")
            .field("identifier", &self.identifier)
            .field("instructions", &self.instructions)
            .finish()
    }
}