use crate::asm_constructs::function::FunctionDefinition;
use crate::asm_constructs::instruction::{BinaryOperator, CondCode, Instruction, StackFrame, UnaryOperator};
use crate::asm_constructs::operand::{Operand, Reg};
use crate::asm_constructs::operand::Operand::Register;
use crate::asm_constructs::program::AsmProgram;
use crate::ast_model::ast_return::AstReturn;
use crate::ast_model::expression::{AstExpression, AstFactor, AstBinaryOp};
use crate::ast_model::function::AstFunction;
use crate::ast_model::program::AstProgram;
use crate::ast_model::expression::AstUnaryOp;
use crate::tacky::TackyVal::Constant;
use crate::tacky::{TackyBinaryOp, TackyFunction, TackyInstruction, TackyProgram, TackyUnaryOp, TackyVal};

pub struct TackyEmit {
    tmp_var_count: i32,
    tmp_label_and_count: i32,
    tmp_label_or_count: i32,
    tmp_label_end_count: i32,
}

impl TackyEmit {
    pub(crate) fn new() -> Self {
        Self { tmp_var_count: 0, tmp_label_and_count: 0, tmp_label_or_count: 0, tmp_label_end_count: 0 }
    }

    pub fn emit_factor(&mut self, factor: &AstFactor, instructions: &mut Vec<TackyInstruction>) -> TackyVal{
        match factor {
            AstFactor::Constant { constant } => Constant(constant.value),
            AstFactor::Unary { unary_op, factor } => {
                let inner_factor = factor.as_ref().clone();
                let src = self.emit_factor(&inner_factor, instructions);
                let dst_name = self.make_temporary();
                let dst = TackyVal::Var(dst_name);
                let tacky_op = TackyEmit::convert_unop(unary_op);
                let tacky_inst = TackyInstruction::Unary(tacky_op, src, dst.clone());
                instructions.push(tacky_inst);
                dst
            }
            AstFactor::Nested(exp) => {
                self.emit_expression(exp, instructions)
            }
        }
    }

    pub fn emit_expression(
        &mut self,
        expression: &AstExpression,
        instructions: &mut Vec<TackyInstruction>,
    ) -> TackyVal {
        match expression {
            AstExpression::Factor(factor) => {
                self.emit_factor(factor, instructions)
            },
            AstExpression::Binary { binop : AstBinaryOp::Or, left, right } => {
                let left_exp = left.as_ref().clone();
                let right_exp = right.as_ref().clone();
                let v1 = self.emit_expression(&left_exp, instructions);
                let label_or_true = self.make_label_or_true();
                let jump_if_not_zero_v1 = TackyInstruction::JumpIfNotZero {condition: v1, target: label_or_true.clone()};
                let v2 = self.emit_expression(&right_exp, instructions);
                let jump_if_not_zero_v2 = TackyInstruction::JumpIfNotZero {condition: v2, target: label_or_true.clone()};

                let result = TackyVal::Var(self.make_temporary());
                let copy_result1 = TackyInstruction::Copy {src: TackyVal::Constant(1), dst: result.clone()};
                let label_end = self.make_label_end();
                let jump_end = TackyInstruction::Jump {target: label_end.clone()};
                let copy_result0 = TackyInstruction::Copy {src: TackyVal::Constant(0), dst: result.clone()};

                instructions.push(jump_if_not_zero_v1);
                instructions.push(jump_if_not_zero_v2);
                instructions.push(copy_result0);
                instructions.push(jump_end);
                instructions.push(TackyInstruction::Label { identifier: label_or_true.clone() });
                instructions.push(copy_result1);
                instructions.push(TackyInstruction::Label { identifier: label_end.clone() });

                result
            }
            AstExpression::Binary { binop : AstBinaryOp::And, left, right } => {
                let left_exp = left.as_ref().clone();
                let right_exp = right.as_ref().clone();
                let v1 = self.emit_expression(&left_exp, instructions);
                let label_and_false = self.make_label_and_false();
                let jump_if_zero_v1 = TackyInstruction::JumpIfZero {condition: v1, target: label_and_false.clone()};
                let v2 = self.emit_expression(&right_exp, instructions);
                let jump_if_zero_v2 = TackyInstruction::JumpIfZero {condition: v2, target: label_and_false.clone()};

                let result = TackyVal::Var(self.make_temporary());
                let copy_result1 = TackyInstruction::Copy {src: TackyVal::Constant(1), dst: result.clone()};
                let label_end = self.make_label_end();
                let jump_end = TackyInstruction::Jump {target: label_end.clone()};
                let copy_result0 = TackyInstruction::Copy {src: TackyVal::Constant(0), dst: result.clone()};

                instructions.push(jump_if_zero_v1);
                instructions.push(jump_if_zero_v2);
                instructions.push(copy_result1);
                instructions.push(jump_end);
                instructions.push(TackyInstruction::Label { identifier: label_and_false.clone() });
                instructions.push(copy_result0);
                instructions.push(TackyInstruction::Label { identifier: label_end.clone() });

                result
            }
            AstExpression::Binary { binop, left, right } => {
                let left_exp = left.as_ref().clone();
                let right_exp = right.as_ref().clone();
                let v1 = self.emit_expression(&left_exp, instructions);
                let v2 = self.emit_expression(&right_exp, instructions);

                let dst_name = self.make_temporary();
                let dst = TackyVal::Var(dst_name);
                let tacky_op = TackyEmit::convert_binop(binop);
                let tacky_inst = TackyInstruction::Binary(tacky_op, v1, v2, dst.clone());
                instructions.push(tacky_inst);
                dst
            }
        }
    }

    fn convert_unop(ast_unary_op: &AstUnaryOp) -> TackyUnaryOp {
        match ast_unary_op {
            AstUnaryOp::Negate => TackyUnaryOp::Negate,
            AstUnaryOp::BitwiseComplement => TackyUnaryOp::Complement,
            AstUnaryOp::Not => TackyUnaryOp::Not,
        }
    }

    fn convert_binop(ast_bin_op: &AstBinaryOp) -> TackyBinaryOp {
        match ast_bin_op {
            AstBinaryOp::Add => TackyBinaryOp::Add,
            AstBinaryOp::Sub =>  TackyBinaryOp::Subtract,
            AstBinaryOp::Mul =>  TackyBinaryOp::Multiply,
            AstBinaryOp::Div =>  TackyBinaryOp::Divide,
            AstBinaryOp::Mod =>  TackyBinaryOp::Modulo,
            AstBinaryOp::And => panic!("Invalid binary operator"),
            AstBinaryOp::Or => panic!("Invalid binary operator"),
            AstBinaryOp::Equal => TackyBinaryOp::Equal,
            AstBinaryOp::NotEqual => TackyBinaryOp::NotEqual,
            AstBinaryOp::LessThan => TackyBinaryOp::LessThan,
            AstBinaryOp::LessThanEqual => TackyBinaryOp::LessOrEqual,
            AstBinaryOp::GreaterThan => TackyBinaryOp::GreaterThan,
            AstBinaryOp::GreaterThanEqual => TackyBinaryOp::GreaterOrEqual,
        }
    }

    fn convert_asm_unop(ast_unary_op: &TackyUnaryOp) -> UnaryOperator {
        match ast_unary_op {
            TackyUnaryOp::Negate => UnaryOperator::Neg,
            TackyUnaryOp::Complement => UnaryOperator::Not,
            TackyUnaryOp::Not => todo!(),
            
        }
    }

    fn convert_asm_binop(ast_binary_op: &TackyBinaryOp) -> BinaryOperator {
        match ast_binary_op {
            TackyBinaryOp::Add => BinaryOperator::Add,
            TackyBinaryOp::Subtract =>  BinaryOperator::Sub,
            TackyBinaryOp::Multiply =>  BinaryOperator::Mul,
            TackyBinaryOp::NotEqual => todo!(),
            TackyBinaryOp::Equal => todo!(),
            TackyBinaryOp::LessThan  => todo!(),
            TackyBinaryOp::LessOrEqual => todo!(),
            TackyBinaryOp::GreaterThan  => todo!(),
            TackyBinaryOp::GreaterOrEqual => todo!(),
            _ => panic!("invalid binary operator"),
        }
    }

    fn make_temporary(&mut self) -> String {
        let tmp = String::from("tmp.") + &self.tmp_var_count.to_string();
        self.tmp_var_count += 1;
        tmp
    }

    fn make_label_and_false(&mut self) -> String {
        let tmp = String::from("label_and_false_") + &self.tmp_label_and_count.to_string();
        self.tmp_label_and_count += 1;
        tmp
    }

    fn make_label_or_true(&mut self) -> String {
        let tmp = String::from("label_or_true_") + &self.tmp_label_or_count.to_string();
        self.tmp_label_or_count += 1;
        tmp
    }

    fn make_label_end(&mut self) -> String {
        let tmp = String::from("label_end_") + &self.tmp_label_end_count.to_string();
        self.tmp_label_end_count += 1;
        tmp
    }

    pub fn emit_return(
        &mut self,
        ast_return: &AstReturn,
        instructions: &mut Vec<TackyInstruction>,
    ) {
        let exp = self.emit_expression(&ast_return.expression, instructions);
        instructions.push(TackyInstruction::Return(exp));
    }

    pub fn emit_program(&mut self, program: &AstProgram) -> TackyProgram {
        TackyProgram {
            function_def: self.emit_function(&program.function),
        }
    }

    pub fn emit_function(&mut self, function: &AstFunction) -> TackyFunction {
        let mut instructions: Vec<TackyInstruction> = Vec::new();
        let _ = self.emit_return(&function.body.return_exp, &mut instructions);
        TackyFunction {
            identifier: function.identifier.clone(),
            body: instructions,
        }
    }

    pub fn convert_asm(&mut self, program: &TackyProgram) -> AsmProgram {
        let function_definition = self.function_to_asm(&program.function_def);

        AsmProgram {
            function_definition,
        }
    }

    pub fn function_to_asm(&mut self, function: &TackyFunction) -> FunctionDefinition {
        let mut instructions: Vec<Instruction> = Vec::new();
        for tacky_instruction in &function.body {
            if let TackyInstruction::Return(val) = tacky_instruction {
                let src = self.value_to_asm(&val);
                let dest = Operand::Register { reg: Reg::AX };
                let mov = Instruction::Mov { src, dest };
                instructions.push(mov);
                instructions.push(Instruction::Ret {});
            } else if let TackyInstruction::Unary(TackyUnaryOp::Not , src, dst) = tacky_instruction {
                let src = self.value_to_asm(&src);
                let dest = self.value_to_asm(&dst);
                let cmp = Instruction::Cmp {left: Operand::Imm {value: 0}, right: src };
                let mov = Instruction::Mov { src: Operand::Imm {value: 0}, dest: dest.clone() };
                let set_cc = Instruction::SetCC {cond_code: CondCode::E, operand: dest.clone()};
                instructions.push(cmp);
                instructions.push(mov);
                instructions.push(set_cc);
            } else if let TackyInstruction::Unary(op, src, dst) = tacky_instruction {
                let src = self.value_to_asm(&src);
                let dest = self.value_to_asm(&dst);
                let mov = Instruction::Mov { src, dest };
                instructions.push(mov);

                let unary_operator = Self::convert_asm_unop(op);
                let dest2 = self.value_to_asm(&dst);
                let unary = Instruction::Unary {
                    unary_operator,
                    operand: dest2,
                };
                instructions.push(unary);
            } else if let TackyInstruction::Binary(op, src1, src2, dst) = tacky_instruction {
                let src1 = self.value_to_asm(&src1);
                let src2 = self.value_to_asm(&src2);
                let dest = self.value_to_asm(&dst);

                match op {
                    TackyBinaryOp::Add | TackyBinaryOp::Subtract | TackyBinaryOp::Multiply => {
                        let mov = Instruction::Mov { src: src1, dest: dest.clone() };
                        let binop = Self::convert_asm_binop(op);
                        let bin = Instruction::Binary {binary_operator: binop, left: src2, right: dest };
                        instructions.push(mov);
                        instructions.push(bin);
                    }
                    TackyBinaryOp::Divide => {
                        let mov1 = Instruction::Mov { src: src1, dest: Register { reg: Reg::AX } };
                        let cdq = Instruction::Cdq;
                        let idiv = Instruction::Idiv { src: src2 };
                        let mov2 = Instruction::Mov { src: Register { reg: Reg::AX }, dest };
                        instructions.push(mov1);
                        instructions.push(cdq);
                        instructions.push(idiv);
                        instructions.push(mov2);
                    }
                    TackyBinaryOp::Modulo => {
                        let mov1 = Instruction::Mov { src: src1, dest: Register { reg: Reg::AX } };
                        let cdq = Instruction::Cdq;
                        let idiv = Instruction::Idiv { src: src2 };
                        let mov2 = Instruction::Mov { src: Register { reg: Reg::DX }, dest };
                        instructions.push(mov1);
                        instructions.push(cdq);
                        instructions.push(idiv);
                        instructions.push(mov2);
                    }
                    TackyBinaryOp::Equal  => { add_relational_operator_instructions(&mut instructions, src1, src2, dest, CondCode::E); }
                    TackyBinaryOp::NotEqual  => { add_relational_operator_instructions(&mut instructions, src1, src2, dest, CondCode::NE); }
                    TackyBinaryOp::GreaterThan => { add_relational_operator_instructions(&mut instructions, src1, src2, dest, CondCode::G); }
                    TackyBinaryOp::GreaterOrEqual  => { add_relational_operator_instructions(&mut instructions, src1, src2, dest, CondCode::GE); }
                    TackyBinaryOp::LessThan   => { add_relational_operator_instructions(&mut instructions, src1, src2, dest, CondCode::L); }
                    TackyBinaryOp::LessOrEqual => { add_relational_operator_instructions(&mut instructions, src1, src2, dest, CondCode::LE); }
                }
            }
            else if let TackyInstruction::JumpIfZero {condition, target} = tacky_instruction {
                let left = Operand::Imm {value: 0};
                let right = self.value_to_asm(condition);
                let cmp = Instruction::Cmp {left, right};
                let jmp_cc = Instruction::JmpCC {cond_code: CondCode::E, identifier: target.clone()};

                instructions.push(cmp);
                instructions.push(jmp_cc);
            }
            else if let TackyInstruction::JumpIfNotZero {condition, target} = tacky_instruction {
                let left = Operand::Imm {value: 0};
                let right = self.value_to_asm(condition);
                let cmp = Instruction::Cmp {left, right};
                let jmp_cc = Instruction::JmpCC {cond_code: CondCode::NE, identifier: target.clone()};

                instructions.push(cmp);
                instructions.push(jmp_cc);
            }
            else if let TackyInstruction::Jump {target} = tacky_instruction {
            instructions.push(Instruction::Jmp{identifier: target.clone() });
            }
            else if let TackyInstruction::Copy {src, dst} = tacky_instruction {
                let srv_val = self.value_to_asm(src);
                let dst_val = self.value_to_asm(dst);
            instructions.push(Instruction::Mov {src: srv_val, dest: dst_val});
            }
            else if let TackyInstruction::Label {identifier} = tacky_instruction {
            instructions.push(Instruction::Label { identifier: identifier.clone() });
            }
            else {
                unreachable!()
            };
        }

        let (new_instructions, stack_frame) = replace_pseudo_registers(&instructions);
        let mut fixed_instructions = fix_instructions(&new_instructions);

        fixed_instructions.insert(
            0,
            Instruction::AllocateStack {
                size: stack_frame.len() * 4,
            },
        );

        FunctionDefinition {
            identifier: function.identifier.clone(),
            instructions: fixed_instructions,
        }
    }

    fn value_to_asm(&self, tacky_val: &TackyVal) -> Operand {
        if let Constant(value) = tacky_val {
            Operand::Imm { value: *value }
        } else if let TackyVal::Var(name) = tacky_val {
            Operand::Pseudo {
                identifier: name.clone(),
            }
        } else {
            unreachable!();
        }
    }
}

fn add_relational_operator_instructions(instructions: &mut Vec<Instruction>, src1: Operand, src2: Operand, dest: Operand, cond_code : CondCode) {
    let cmp = Instruction::Cmp {left: src2, right: src1 };
    let mov = Instruction::Mov { src: Operand::Imm {value: 0}, dest: dest.clone() };
    let set_cc = Instruction::SetCC {cond_code, operand: dest.clone()};
    instructions.push(cmp);
    instructions.push(mov);
    instructions.push(set_cc);
}

fn fix_instructions(instructions: &Vec<Instruction>) -> Vec::<Instruction> {
    let mut new_instructions = Vec::<Instruction>::new();

    instructions.into_iter().for_each(|instruction| {
        let result = instruction.fix_instruction();
        if result.is_some() {
            let result = result.unwrap();
            result.into_iter().for_each(|instruction| {
                new_instructions.push(instruction);
            });
        } else {
            new_instructions.push(instruction.clone());
        }
    });

    new_instructions
}

fn replace_pseudo_registers(instructions: &Vec<Instruction>) -> (Vec<Instruction>, StackFrame) {
    let mut stack_frame = StackFrame::new();
    let mut new_instructions = Vec::<Instruction>::new();

    instructions.into_iter().for_each(|instruction| {
        let result = instruction.replace_pseudo_registers(&mut stack_frame);
        new_instructions.push(result);
    });

    (new_instructions, stack_frame)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;
    use crate::ast_model::constant::AstConstant;
    use crate::ast_model::statement::AstStatement;
    use crate::ast_model::expression::AstUnaryOp::{BitwiseComplement, Negate};
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    #[test]
    pub fn test_emit_expression_constant() {
        let mut emit = TackyEmit::new();
        let ast_exp = AstExpression::Factor(AstFactor::Constant {
            constant: AstConstant { value: 3 },
        });
        let mut instructions: Vec<TackyInstruction> = Vec::new();
        let result = emit.emit_expression(&ast_exp, &mut instructions);

        assert_eq!(result, Constant(3));
        assert_eq!(instructions.len(), 0);
    }

    #[test]
    pub fn test_emit_expression_unary() {
        let mut emit = TackyEmit::new();

        let ast_exp = AstExpression::Factor(AstFactor::Unary {
            unary_op: Negate,
            factor: Box::new(AstFactor::Constant {
                constant: AstConstant { value: 3 },
            }),
        });
        let mut instructions: Vec<TackyInstruction> = Vec::new();
        let result = emit.emit_expression(&ast_exp, &mut instructions);

        assert_eq!(result, TackyVal::Var(String::from("tmp.0")));
        assert_eq!(instructions.len(), 1);
        let instruction = instructions.get(0).unwrap();
        if let TackyInstruction::Unary(op, src, dst) = instruction {
            assert_eq!(op, &TackyUnaryOp::Negate);
            assert_eq!(src, &Constant(3));
            assert_eq!(dst, &TackyVal::Var(String::from("tmp.0")));
        } else {
            panic!();
        }
    }

    #[test]
    pub fn test_emit_return() {
        let mut emit = TackyEmit::new();

        let ast_return = AstReturn {
            expression: AstExpression::Factor(AstFactor::Constant {
                constant: AstConstant { value: 3 }
            })
        };
        let mut instructions: Vec<TackyInstruction> = Vec::new();
        emit.emit_return(&ast_return, &mut instructions);
        assert_eq!(instructions.len(), 1);
        let instruction = instructions.get(0).unwrap();
        if let TackyInstruction::Return(val) = instruction {
            assert_eq!(val, &Constant(3));
        } else {
            panic!();
        }
    }

    #[test]
    pub fn test_emit_return_double_unary() {
        let mut emit = TackyEmit::new();

        let ast_return = AstReturn {
            expression: AstExpression::Factor(AstFactor::Unary {
                unary_op: Negate,
                factor: Box::new(AstFactor::Unary {
                    unary_op: BitwiseComplement,
                    factor: Box::new(AstFactor::Constant {
                        constant: AstConstant { value: 3 },
                    }),
                }),
            }),
        };
        let mut instructions: Vec<TackyInstruction> = Vec::new();
        emit.emit_return(&ast_return, &mut instructions);
        assert_eq!(instructions.len(), 3);
        let instruction = instructions.get(0).unwrap();
        if let TackyInstruction::Unary(op, src, dst) = instruction {
            assert_eq!(op, &TackyUnaryOp::Complement);
            assert_eq!(src, &Constant(3));
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
                    expression: AstExpression::Factor(AstFactor::Constant {
                        constant: AstConstant { value: 3 },
                    }),
                },
            },
        };

        let result = emit.emit_function(&function);
        assert_eq!(result.identifier, "main");
        assert_eq!(result.body.len(), 1);
        let instruction = result.body.get(0).unwrap();
        if let TackyInstruction::Return(val) = instruction {
            assert_eq!(val, &Constant(3));
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
                        expression: AstExpression::Factor(AstFactor::Constant {
                            constant: AstConstant { value: 3 },
                        }),
                    },
                },
            },
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


    #[test]
    pub fn test_bin_and() {
        let tacky = tacky("1 && 0".to_string());
        assert_eq!(tacky.len(), 8);

        assert_eq!(tacky[0], "JumpIfZero { condition: Constant(1), target: \"label_and_false_0\" }");
        assert_eq!(tacky[1], "JumpIfZero { condition: Constant(0), target: \"label_and_false_0\" }");
        assert_eq!(tacky[2], "Copy { src: Constant(1), dst: Var(\"tmp.0\") }");
        assert_eq!(tacky[3], "Jump { target: \"label_end_0\" }");
        assert_eq!(tacky[4], "Label { identifier: \"label_and_false_0\" }");
        assert_eq!(tacky[5], "Copy { src: Constant(0), dst: Var(\"tmp.0\") }");
        assert_eq!(tacky[6], "Label { identifier: \"label_end_0\" }");
        assert_eq!(tacky[7], "Return(Var(\"tmp.0\"))");
    }

    #[test]
    pub fn test_bin_or() {
        let tacky = tacky("1 || 0".to_string());
        assert_eq!(tacky.len(), 8);

        assert_eq!(tacky[0], "JumpIfNotZero { condition: Constant(1), target: \"label_or_true_0\" }");
        assert_eq!(tacky[1], "JumpIfNotZero { condition: Constant(0), target: \"label_or_true_0\" }");
        assert_eq!(tacky[2], "Copy { src: Constant(0), dst: Var(\"tmp.0\") }");
        assert_eq!(tacky[3], "Jump { target: \"label_end_0\" }");
        assert_eq!(tacky[4], "Label { identifier: \"label_or_true_0\" }");
        assert_eq!(tacky[5], "Copy { src: Constant(1), dst: Var(\"tmp.0\") }");
        assert_eq!(tacky[6], "Label { identifier: \"label_end_0\" }");
        assert_eq!(tacky[7], "Return(Var(\"tmp.0\"))");
    }

    #[test_case("1 + 0", "Binary(Add, Constant(1), Constant(0), Var(\"tmp.0\"))")]
    #[test_case("1 - 0", "Binary(Subtract, Constant(1), Constant(0), Var(\"tmp.0\"))")]
    #[test_case("1 / 0", "Binary(Divide, Constant(1), Constant(0), Var(\"tmp.0\"))")]
    #[test_case("1 * 0", "Binary(Multiply, Constant(1), Constant(0), Var(\"tmp.0\"))")]
    #[test_case("1 % 0", "Binary(Modulo, Constant(1), Constant(0), Var(\"tmp.0\"))")]
    #[test_case("1 == 0", "Binary(Equal, Constant(1), Constant(0), Var(\"tmp.0\"))")]
    #[test_case("1 != 0", "Binary(NotEqual, Constant(1), Constant(0), Var(\"tmp.0\"))")]
    #[test_case("1 < 0", "Binary(LessThan, Constant(1), Constant(0), Var(\"tmp.0\"))")]
    #[test_case("1 <= 0", "Binary(LessOrEqual, Constant(1), Constant(0), Var(\"tmp.0\"))")]
    #[test_case("1 > 0", "Binary(GreaterThan, Constant(1), Constant(0), Var(\"tmp.0\"))")]
    #[test_case("1 >= 0", "Binary(GreaterOrEqual, Constant(1), Constant(0), Var(\"tmp.0\"))")]
    pub fn test_bin_op(code: &str, expected: &str) {
        let tacky = tacky(code.to_string());
        assert_eq!(tacky.len(), 2);
        assert_eq!(tacky[0], expected);
    }

    pub fn tacky(code : String) -> Vec<String> {
        let program : String = "int main(void) { return ".to_string() + code.as_str() + "; }";
        let lexer = Lexer::new(program);
        let mut tokens = lexer.tokenize().unwrap();
        let parser = Parser::new();
        let program_result = parser.parse_program(&mut tokens).unwrap();

        let mut emit = crate::tacky::tacky_emit::TackyEmit::new();
        let tacky_program = emit.emit_program(&program_result);
        let tacky = tacky_program.function_def.body
            .iter()
            .map(|tacky_inst| format!("{:?}", tacky_inst))
            .collect();
        tacky
    }
}
