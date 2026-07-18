use crate::asm_constructs;
use crate::asm_constructs::allocate_stack::AllocateStack;
use crate::asm_constructs::function::FunctionDefinition;
use crate::asm_constructs::imm::Imm;
use crate::asm_constructs::instruction::{Instruction, StackFrame};
use crate::asm_constructs::mov::Mov;
use crate::asm_constructs::operand::Operand;
use crate::asm_constructs::program::AsmProgram;
use crate::asm_constructs::pseudo::Pseudo;
use crate::asm_constructs::register::{Reg, Register};
use crate::asm_constructs::ret::Ret;
use crate::asm_constructs::unary::UnaryOperator;
use crate::ast_model::ast_return::AstReturn;
use crate::ast_model::expression::AstExpression;
use crate::ast_model::function::AstFunction;
use crate::ast_model::program::AstProgram;
use crate::ast_model::unary::AstUnaryOp;
use crate::tacky::{TackyFunction, TackyInstruction, TackyProgram, TackyUnaryOp, TackyVal};
use crate::tacky::TackyVal::Constant;

pub struct TackyEmit {
    tmp_var_count : i32
}

impl TackyEmit {
    pub(crate) fn new() -> Self {
        Self { tmp_var_count: 0 }
    }

    pub fn emit_expression(&mut self, expression: &AstExpression, instructions: &mut Vec<TackyInstruction>) -> TackyVal {
        match expression {
            AstExpression::Constant { constant: cst } => {
                TackyVal::Constant(cst.value)
            }
            AstExpression::Unary { unary_op: op, expression: inner } => {
                let exp = inner.as_ref().clone();
                let src = self.emit_expression(&exp, instructions);
                let dst_name = self.make_temporary();
                let dst = TackyVal::Var(dst_name);
                let tacky_op = TackyEmit::convert_unop(op);
                let tacky_inst = TackyInstruction::Unary(tacky_op, src, dst.clone());
                instructions.push(tacky_inst);
                dst
            }
        }
    }

    fn convert_unop(ast_unary_op: &AstUnaryOp) -> TackyUnaryOp {
        match ast_unary_op {
            AstUnaryOp::Negate => TackyUnaryOp::Negate,
            AstUnaryOp::BitwiseComplement => TackyUnaryOp::Complement
        }
    }

    fn convert_asm_unop(ast_unary_op: &TackyUnaryOp) -> UnaryOperator {
        match ast_unary_op {
            TackyUnaryOp::Negate => UnaryOperator::Neg,
            TackyUnaryOp::Complement => UnaryOperator::Not
        }
    }

    fn make_temporary(&mut self) -> String {
        let tmp = String::from("tmp.") + &self.tmp_var_count.to_string();
        self.tmp_var_count+=1;
        tmp
    }

    pub fn emit_return(&mut self, ast_return: &AstReturn, instructions: &mut Vec<TackyInstruction>) {
        let exp = self.emit_expression(&ast_return.expression, instructions);
        instructions.push(TackyInstruction::Return(exp));
    }

    pub fn emit_program(&mut self, program: &AstProgram) -> TackyProgram {
        TackyProgram{
            function_def: self.emit_function(&program.function)
        }
    }

    pub fn emit_function(&mut self, function: &AstFunction) -> TackyFunction {
        let mut instructions : Vec<TackyInstruction> = Vec::new();
        let _ = self.emit_return(&function.body.return_exp, &mut instructions);
        TackyFunction {
            identifier: function.identifier.clone(),
            body: instructions
        }
    }

    pub fn to_asm(&mut self, program: &TackyProgram) -> AsmProgram {
        let function_definition = self.function_to_asm(&program.function_def);

        AsmProgram {
            function_definition,
        }
    }

    pub fn function_to_asm(&mut self, function: &TackyFunction) -> FunctionDefinition {
        let mut instructions : Vec<Box< dyn Instruction>> = Vec::new();
        for tacky_instruction in &function.body {
            if let TackyInstruction::Return(val) = tacky_instruction {
                let src= self.value_to_asm(&val);
                let dest = Register {reg : Reg::AX };
                let mov = Mov {src, dest: Box::new(dest)};
                instructions.push(Box::new(mov));
                instructions.push(Box::new(Ret{}));
            } else if let TackyInstruction::Unary(op, src, dst) = tacky_instruction {
                let src= self.value_to_asm(&src);
                let dest= self.value_to_asm(&dst);
                let mov = Mov {src, dest};
                instructions.push(Box::new(mov));

                let unary_operator = Self::convert_asm_unop(op);
                let dest2= self.value_to_asm(&dst);
                let unary = asm_constructs::unary::Unary::new(unary_operator, dest2);
                instructions.push(Box::new(unary));

            } else {
                unreachable!()
            };

        }

        let stack_frame = replace_pseudo_registers(&mut instructions);
        instructions.insert(0, Box::new(AllocateStack::new(stack_frame.len()*4)));

        FunctionDefinition {
            identifier: function.identifier.clone(),
            instructions
        }
    }

    fn value_to_asm(&self, tacky_val: &TackyVal) -> Box<dyn Operand> {
        if let Constant(value) = tacky_val {
            Box::new(Imm { value: *value })
        } else if let TackyVal::Var(name) = tacky_val {
            Box::new(Pseudo { identifier: name.clone() })
        } else {
            unreachable!();
        }
    }
}

fn replace_pseudo_registers(instructions: &mut Vec<Box<dyn Instruction>>) -> StackFrame{
    let mut stack_frame = StackFrame::new();

    instructions.into_iter().for_each(|instruction| {
        let inst = instruction.as_mut();
        inst.fix_pseudo_registers(&mut stack_frame);
    });

    stack_frame
}

#[cfg(test)]
mod tests {
    use crate::ast_model::constant::AstConstant;
    use crate::ast_model::statement::AstStatement;
    use crate::ast_model::unary::AstUnaryOp::{BitwiseComplement, Negate};
    use super::*;

    #[test]
    pub fn test_emit_expression_constant() {
        let mut emit = TackyEmit::new();
        let ast_exp = AstExpression::Constant {
            constant: AstConstant { value: 3},
        };
        let mut instructions : Vec<TackyInstruction> = Vec::new();
        let result = emit.emit_expression(&ast_exp, &mut instructions);

        assert_eq!(result, TackyVal::Constant(3));
        assert_eq!(instructions.len(), 0);
    }

    #[test]
    pub fn test_emit_expression_unary() {
        let mut emit = TackyEmit::new();

        let ast_exp = AstExpression::Unary {
            unary_op: Negate,
            expression: Box::new(AstExpression::Constant {
                constant: AstConstant { value: 3},
            })
        };
        let mut instructions : Vec<TackyInstruction> = Vec::new();
        let result = emit.emit_expression(&ast_exp, &mut instructions);

        assert_eq!(result, TackyVal::Var(String::from("tmp.0")));
        assert_eq!(instructions.len(), 1);
        let instruction = instructions.get(0).unwrap();
        if let TackyInstruction::Unary(op, src, dst) = instruction {
            assert_eq!(op, &TackyUnaryOp::Negate);
            assert_eq!(src, &TackyVal::Constant(3));
            assert_eq!(dst, &TackyVal::Var(String::from("tmp.0")));
        } else {
            panic!();
        }
    }

    #[test]
    pub fn test_emit_return() {
        let mut emit = TackyEmit::new();

        let ast_return = AstReturn {
            expression: AstExpression::Constant {
                constant: AstConstant { value: 3},
            }
        };
        let mut instructions : Vec<TackyInstruction> = Vec::new();
        emit.emit_return(&ast_return, &mut instructions);
        assert_eq!(instructions.len(), 1);
        let instruction = instructions.get(0).unwrap();
        if let TackyInstruction::Return(val) = instruction {
            assert_eq!(val, &TackyVal::Constant(3));
        } else {
            panic!();
        }
    }

    #[test]
    pub fn test_emit_return_double_unary() {
        let mut emit = TackyEmit::new();

        let ast_return = AstReturn {
            expression: AstExpression::Unary {
                unary_op: Negate,
                expression: Box::new(AstExpression::Unary {
                    unary_op: BitwiseComplement,
                    expression: Box::new(AstExpression::Constant {
                        constant: AstConstant { value: 3},
                    })
                })
            }
        };
        let mut instructions : Vec<TackyInstruction> = Vec::new();
        emit.emit_return(&ast_return, &mut instructions);
        assert_eq!(instructions.len(), 3);
        let instruction = instructions.get(0).unwrap();
        if let TackyInstruction::Unary(op, src, dst) = instruction {
            assert_eq!(op, &TackyUnaryOp::Complement);
            assert_eq!(src, &TackyVal::Constant(3));
            assert_eq!(dst, &TackyVal::Var(String::from("tmp.0")));
        }
        let instruction = instructions.get(1).unwrap();
        if let TackyInstruction::Unary(op, src, dst) = instruction {
            assert_eq!(op, &TackyUnaryOp::Negate);
            assert_eq!(src, &TackyVal::Var(String::from("tmp.0")));
            assert_eq!(dst, &TackyVal::Var(String::from("tmp.1")));
        } else {
            panic!();
        }
        let instruction = instructions.get(2).unwrap();
        if let TackyInstruction::Return(val) = instruction {
            assert_eq!(val, &TackyVal::Var(String::from("tmp.1")));
        } else {
            panic!();
        }
    }

    #[test]
    pub fn test_make_temporary() {
        let mut emit = TackyEmit::new();

        let result = emit.make_temporary();
        assert_eq!(result, String::from("tmp.0"));
        let result2 = emit.make_temporary();
        assert_eq!(result2, String::from("tmp.1"));
    }

    #[test]
    pub fn test_convert_unop() {
        let result = TackyEmit::convert_unop(&Negate);
        assert_eq!(result, TackyUnaryOp::Negate);

        let result2 = TackyEmit::convert_unop(&BitwiseComplement);
        assert_eq!(result2, TackyUnaryOp::Complement);
    }

    #[test]
    pub fn test_emit_function() {
        let mut emit = TackyEmit::new();

        let function = AstFunction {
            identifier: "main".to_string(),
            body: AstStatement {
                return_exp: AstReturn {
                    expression: AstExpression::Constant {
                        constant: AstConstant { value: 3},
                    }
                }
            },
        };

        let result = emit.emit_function(&function);
        assert_eq!(result.identifier, "main");
        assert_eq!(result.body.len(), 1);
        let instruction = result.body.get(0).unwrap();
        if let TackyInstruction::Return(val) = instruction {
            assert_eq!(val, &TackyVal::Constant(3));
        } else {
            panic!();
        }
    }

    #[test]
    pub fn test_emit_program() {
        let mut emit = TackyEmit::new();
        let program = AstProgram {
            function: AstFunction {
                identifier: "main".to_string(),
                body: AstStatement {
                    return_exp: AstReturn {
                        expression: AstExpression::Constant {
                            constant: AstConstant { value: 3},
                        }
                    }
                }
            }
        };

        let result = emit.emit_program(&program);
        assert_eq!(result.function_def.identifier, "main");
        assert_eq!(result.function_def.body.len(), 1);

        let instruction = result.function_def.body.get(0).unwrap();
        if let TackyInstruction::Return(val) = instruction {
            assert_eq!(val, &Constant(3));
        } else {
            panic!();
        }
    }
}
