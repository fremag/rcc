use std::fmt;
use std::fmt::Formatter;

pub struct AsmProgram {
    pub(crate) function_definition: FunctionDefinition
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

impl fmt::Debug for FunctionDefinition {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("FunctionDefinition")
            .field("identifier", &self.identifier)
            .field("instructions", &self.instructions)
            .finish()
    }
}

pub trait Instruction: std::fmt::Debug {}

pub struct AsmReturn {
}

impl Instruction for AsmReturn {}
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

impl Instruction for Mov {}
impl fmt::Debug for Mov {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Mov")
            .field("src", &self.src)
            .field("dest", &self.dest)
            .finish()
    }
}

pub trait Operand: std::fmt::Debug {}

pub struct Imm {
    pub(crate) value: i32
}

impl Operand for Imm {}
impl fmt::Debug for Imm {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Imm")
            .field("value", &self.value)
            .finish()
    }
}

pub struct Register {
}

impl Operand for Register {}
impl fmt::Debug for Register {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Register")
            .finish()
    }
}
