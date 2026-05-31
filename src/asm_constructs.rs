use std::fmt;
use std::fmt::Formatter;

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

pub trait Instruction: std::fmt::Debug {
    fn to_code(&self) -> String;
}

pub struct AsmReturn {
}

impl Instruction for AsmReturn {
    fn to_code(&self) -> String {
        String::from("ret")
    }
}
impl fmt::Debug for AsmReturn {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("AsmReturn")
            .finish()
    }
}

pub struct Mov {
    pub(crate) src : Box<dyn Operand>,
    pub(crate) dest : Box<dyn Operand>
}

impl Instruction for Mov {
    fn to_code(&self) -> String {
        format!("movl {}, {}", self.src.to_code(), self.dest.to_code())
    }
}
impl fmt::Debug for Mov {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Mov")
            .field("src", &self.src)
            .field("dest", &self.dest)
            .finish()
    }
}

pub trait Operand: std::fmt::Debug {
    fn to_code(&self) -> String;
}

pub struct Imm {
    pub(crate) value: i32
}

impl Operand for Imm {
    fn to_code(&self) -> String {
        String::from(format!("${}", self.value))
    }
}
impl fmt::Debug for Imm {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Imm")
            .field("value", &self.value)
            .finish()
    }
}

pub struct Register {
}

impl Operand for Register {
    fn to_code(&self) -> String {
        String::from("%eax")
    }
}
impl fmt::Debug for Register {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Register")
            .finish()
    }
}
