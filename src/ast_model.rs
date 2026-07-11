use crate::asm_constructs::{AsmProgram, AsmReturn, FunctionDefinition, Imm, Instruction, Mov, Operand, Register};

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

#[derive(Debug)]
pub struct AstStatement {
    pub(crate) return_exp: AstReturn
}

impl AstStatement {
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
pub struct AstReturn {
    pub(crate) expression: AstExpression
}

impl AstReturn {
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

#[derive(Debug, Clone)]
pub enum AstExpression {
    Constant(AstConstant),
    Unary(AstUnaryOp, Box<AstExpression>)
}

#[derive(Debug)]
struct AstUnaryOperand {
    pub(crate) op: AstUnaryOp,
    pub(crate) exp: Box<AstExpression>
}

impl Operand for AstUnaryOperand {
    fn to_code(&self) -> String {
        todo!()
    }
}

impl AstExpression {
    pub fn to_asm(&self) -> Box<dyn Operand> {
        match self {
            AstExpression::Constant(cst) => Box::new(Imm { value: cst.value }),
            AstExpression::Unary(op, exp) => {
                Box::new(AstUnaryOperand {
                    op: op.clone(),
                    exp: exp.clone(),
                })
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AstUnaryOp {
    Negate, BitwiseComplement
}

#[derive(Debug, Clone)]
pub struct AstConstant {
    pub(crate) value: i32
}
