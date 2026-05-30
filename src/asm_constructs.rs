
pub struct AsmProgram {
    pub(crate) function_definition: FunctionDefinition
}

pub struct  FunctionDefinition {
    pub(crate) identifier: String,
    pub(crate) instructions : Vec<Box<dyn Instruction>>
}

pub trait Instruction {

}

pub struct AsmReturn {
}

impl Instruction for AsmReturn {}

pub struct Mov {
    pub(crate) src : Box<dyn Operand>,
    pub(crate) dest : Box<dyn Operand>
}

impl Instruction for Mov {}

pub trait Operand {
}

pub struct Imm {
    pub(crate) value: i32
}

impl Operand for Imm {}

pub struct Register {
}

impl Operand for Register {}
