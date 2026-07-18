use crate::asm_constructs::instruction::Instruction;

#[derive(Debug)]
pub struct FunctionDefinition {
    pub(crate) identifier: String,
    pub(crate) instructions : Vec<Box<dyn Instruction>>
}

impl FunctionDefinition {
    pub(crate) fn to_code(&self) -> String {
        let mut asm_code = format!("\t.globl {}\n{}:\n\tpushq %rbp\nmovq {}%rsp, %rbp\n", self.identifier, self.identifier, "{@}");
        for instruction in &self.instructions {
            let inst_code = instruction.to_code();
            asm_code += format!("\t{}\n", inst_code).as_str();
        }

        asm_code
    }
}