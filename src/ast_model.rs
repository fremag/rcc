use crate::asm_constructs::{AsmProgram, AsmReturn, FunctionDefinition, Imm, Instruction, Mov, Operand, Register};

#[derive(Debug)]
pub struct Program {
    pub(crate) function: Function
}

impl Program {
    pub fn to_asm(&self) -> AsmProgram {
        AsmProgram{
            function_definition: self.function.to_asm(),
        }
    }
}
#[derive(Debug)]
pub struct Function {
    pub(crate) identifier: String,
    pub(crate) body : Statement
}

impl Function {
    pub(crate) fn to_asm(&self) -> FunctionDefinition {
        FunctionDefinition {
            identifier: self.identifier.clone(),
            instructions: self.body.to_asm(),
        }
    }
}

#[derive(Debug)]
pub struct Statement {
    pub(crate) return_exp: Return
}

impl Statement {
    pub(crate) fn to_asm(&self) -> Vec<Box<dyn Instruction>> {
        let mut instructions = Vec::new();
        let exp_instructions = self.return_exp.to_asm();
        for instruction in exp_instructions {
            instructions.push(instruction);
        }
        instructions
    }
}

#[derive(Debug)]
pub struct Return {
    pub(crate) expression: Expression
}

impl Return {
    pub(crate) fn to_asm(&self) -> Vec<Box<dyn Instruction>> {
        let mut instructions : Vec<Box<dyn Instruction>> = Vec::new();
        let mov = Mov{
            src: self.expression.to_asm(),
            dest: Box::new(Register{})
        };

        instructions.push(Box::new(mov));
        instructions.push(Box::new(AsmReturn{}));
        instructions
    }
}

#[derive(Debug)]
pub enum Expression {
    Constant(Constant)
}

impl Expression {
    pub fn to_asm(&self) -> Box<dyn Operand> {
        match self {
            Expression::Constant(cst) => Box::new(Imm { value: cst.value })
        }
    }
}

#[derive(Debug)]
pub struct Constant {
    pub(crate) value: i32
}
