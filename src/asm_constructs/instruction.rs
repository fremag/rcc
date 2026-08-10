use crate::asm_constructs::operand::{Operand, Reg};
use std::collections::HashMap;
use Operand::Register;
use crate::asm_constructs::instruction::Instruction::{Binary, Idiv, Mov};
use crate::asm_constructs::operand::Operand::Stack;

#[derive(Debug, Clone)]
pub enum UnaryOperator {
    Neg,
    Not,
}

#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
}

#[derive(Debug, Clone)]
pub enum Instruction {
    AllocateStack {
        size: usize,
    },
    Mov {
        src: Operand,
        dest: Operand,
    },
    Unary {
        unary_operator: UnaryOperator,
        operand: Operand,
    },
    Ret,
    Binary {
        binary_operator : BinaryOperator,
        left: Operand,
        right: Operand
    },
    Idiv {
        src: Operand,
    },
    Cdq
}

impl Instruction {
    pub fn to_code(&self) -> String {
        match self {
            Instruction::AllocateStack { size } => format!("subq ${}, %rsp", size),
            Mov { src, dest } => {
                format!("movl {}, {}", src.to_code(), dest.to_code())
            }
            Instruction::Unary {
                unary_operator,
                operand,
            } => {
                let unary = match unary_operator {
                    UnaryOperator::Neg => "negl",
                    UnaryOperator::Not => "notl",
                };
                format!("{} {}", unary, operand.to_code())
            }
            Instruction::Ret => String::from("movq %rbp, %rsp\n\tpopq %rbp\n\tret"),
            Binary {binary_operator, left, right} => {
                let binary = match binary_operator {
                    BinaryOperator::Add => "addl",
                    BinaryOperator::Sub => "subl",
                    BinaryOperator::Mul => "mull",
                };
                format!("{} {}, {}, ", binary, left.to_code(), right.to_code())
            }

            Idiv { src } => {format!("idivl {}", src.to_code())}
            Instruction::Cdq => {String::from("cdq")}
        }
    }

    pub fn fix_pseudo_registers(&self, pseudo_registers: &mut StackFrame) -> Instruction {
        match self {
            Instruction::Mov { src, dest } => {
                let new_src = src.fix_pseudo_registers(pseudo_registers);
                let new_dest = dest.fix_pseudo_registers(pseudo_registers);
                Instruction::Mov {
                    src: new_src,
                    dest: new_dest,
                }
            }

            Instruction::Unary {
                unary_operator,
                operand,
            } => {
                let new_operand = operand.fix_pseudo_registers(pseudo_registers);
                Instruction::Unary {
                    unary_operator: unary_operator.clone(),
                    operand: new_operand,
                }
            },
            Instruction::AllocateStack { size } => Instruction::AllocateStack { size: *size },
            Instruction::Ret => Instruction::Ret,
            Instruction::Binary { binary_operator, left, right } => {
                let new_left = left.fix_pseudo_registers(pseudo_registers);
                let new_right = right.fix_pseudo_registers(pseudo_registers);
                Instruction::Binary { binary_operator: binary_operator.clone(), right: new_right, left: new_left}
            }
            Instruction::Idiv { src } => {
                let new_src = src.fix_pseudo_registers(pseudo_registers);
                Instruction::Idiv { src: new_src } 
            }
            Instruction::Cdq => Instruction::Cdq
        }
    }

    pub(crate) fn fix_instruction(&self) -> Option<Vec<Instruction>> {
        match self {
            Mov {src, dest} => {
                match (src, dest) {
                    (Stack {offset : offset_src }, Stack {offset: offset_dest}) => {
                        Some(vec![
                            Mov {src: Stack {offset: *offset_src}, dest: Register {reg: Reg::R10}},
                            Mov {src: Register {reg: Reg::R10}, dest: Stack {offset: *offset_dest}},
                        ])
                    }
                    (_, _) => None
                }
            },
            Idiv { src } => {
                match src {
                    Operand::Imm {value} => {
                        Some(vec![ 
                            Mov { src: Operand::Imm{ value: *value }, dest: Register {reg: Reg::R10} },
                            Idiv {src: Register {reg: Reg::R10}},
                        ])
                    },
                    _ => None

                }
            },
            Binary {binary_operator, left, right} => {
                match binary_operator {
                    BinaryOperator::Add => {
                        match (left, right) {
                            (Stack { offset: offset_src }, Stack { offset: offset_dest }) => {
                                Some(vec![
                                    Mov { src: Stack { offset: *offset_src }, dest: Register { reg: Reg::R10 } },
                                    Binary {binary_operator: BinaryOperator::Add, left: Register { reg: Reg::R10 }, right: Stack { offset: *offset_dest } },
                                ])
                            },
                            (_, _) => None
                        }
                    },
                    BinaryOperator::Sub => {
                        match (left, right) {
                            (Stack { offset: offset_src }, Stack { offset: offset_dest }) => {
                                Some(vec![
                                    Mov { src: Stack { offset: *offset_src }, dest: Register { reg: Reg::R10 } },
                                    Binary {binary_operator: BinaryOperator::Sub, left: Register { reg: Reg::R10 }, right: Stack { offset: *offset_dest } },
                                ])
                            },
                            (_, _) => None
                        }
                    },
                    BinaryOperator::Mul => {
                        match (left, right) {
                            (operand, Stack { offset: offset_dest }) => {
                                Some(vec![
                                    Mov { src: Stack {offset: *offset_dest}, dest: Register { reg: Reg::R11 } },
                                    Binary {binary_operator: BinaryOperator::Mul, left: operand.clone(), right: Register { reg: Reg::R11 }},
                                    Mov { src: Register { reg: Reg::R11 }, dest: Stack {offset: *offset_dest}},
                                ])
                            },
                            (_, _) => None
                        }
                    },
                }
            },
            _ => None
        }
    }

}

pub struct StackFrame {
    items: HashMap<String, usize>,
}

impl StackFrame {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.items.len()
    }

    pub fn get(&mut self, key: &str) -> usize {
        match self.items.get(key) {
            Some(value) => *value * 4,
            None => {
                let n = self.items.len() + 1;
                self.items.insert(key.to_string(), n);
                4*n
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allocate_stack_to_code() {
        let instr = Instruction::AllocateStack { size: 16 };
        assert_eq!(instr.to_code(), "subq $16, %rsp");
    }

    #[test]
    fn test_mov_to_code() {
        let instr = Instruction::Mov {
            src: Operand::Imm { value: 42 },
            dest: Operand::Register { reg: Reg::AX },
        };
        assert_eq!(instr.to_code(), "movl $42, %eax");
    }

    #[test]
    fn test_unary_neg_to_code() {
        let instr = Instruction::Unary {
            unary_operator: UnaryOperator::Neg,
            operand: Operand::Register { reg: Reg::AX },
        };
        assert_eq!(instr.to_code(), "negl %eax");
    }

    #[test]
    fn test_unary_not_to_code() {
        let instr = Instruction::Unary {
            unary_operator: UnaryOperator::Not,
            operand: Operand::Register { reg: Reg::R10 },
        };
        assert_eq!(instr.to_code(), "notl %r10d");
    }

    #[test]
    fn test_ret_to_code() {
        let instr = Instruction::Ret;
        assert_eq!(instr.to_code(), "movq %rbp, %rsp\n\tpopq %rbp\n\tret");
    }

    #[test]
    fn test_fix_pseudo_registers_mov() {
        let mut stack_frame = StackFrame::new();
        let instr = Instruction::Mov {
            src: Operand::Pseudo { identifier: "var1".to_string() },
            dest: Operand::Register { reg: Reg::AX },
        };
        let fixed = instr.fix_pseudo_registers(&mut stack_frame);

        match fixed {
            Instruction::Mov { src, dest } => {
                match src {
                    Operand::Stack { offset } => assert_eq!(offset, 4),
                    _ => panic!("Expected Stack operand"),
                }
                match dest {
                    Operand::Register { reg } => assert!(matches!(reg, Reg::AX)),
                    _ => panic!("Expected Register operand"),
                }
            }
            _ => panic!("Expected Mov instruction"),
        }
    }

    #[test]
    fn test_fix_instruction_register_to_register() {
        let instr = Instruction::Mov {
            src: Operand::Register { reg: Reg::AX },
            dest: Operand::Register { reg: Reg::AX },
        };
        let fixed = instr.fix_instruction();

        assert!(fixed.is_none());
    }

    #[test]
    fn test_fix_instruction_stack_to_stack() {
        let instr = Instruction::Mov {
            src: Operand::Stack { offset: 8 },
            dest: Operand::Stack { offset: 12 },
        };
        let fixed = instr.fix_instruction();

        assert!(fixed.is_some());
        let two_instructions = fixed.unwrap();
        assert_eq!(two_instructions[0].to_code(), "movl -8(%rbp), %r10d");
        assert_eq!(two_instructions[1].to_code(), "movl %r10d, -12(%rbp)");
    }

    #[test]
    fn test_fix_instruction_non_register_mov() {
        let instr = Instruction::Mov {
            src: Operand::Imm { value: 42 },
            dest: Operand::Register { reg: Reg::AX },
        };
        let fixed = instr.fix_instruction();
        assert!(fixed.is_none());
    }

    #[test]
    fn test_stack_frame_new() {
        let stack_frame = StackFrame::new();
        assert_eq!(stack_frame.len(), 0);
    }

    #[test]
    fn test_stack_frame_get_same_key() {
        let mut stack_frame = StackFrame::new();
        let offset1 = stack_frame.get("tmp.1");
        let offset2 = stack_frame.get("tmp.1");
        assert_eq!(offset1, offset2);
        assert_eq!(offset1, 4);
    }

    #[test]
    fn test_stack_frame_get_different_keys() {
        let mut stack_frame = StackFrame::new();
        let offset1 = stack_frame.get("tmp.1");
        let offset2 = stack_frame.get("tmp.2");
        assert_ne!(offset1, offset2);
        assert_eq!(offset1, 4);
        assert_eq!(offset2, 8);
    }

    #[test]
    fn test_stack_frame_len() {
        let mut stack_frame = StackFrame::new();
        assert_eq!(stack_frame.len(), 0);
        stack_frame.get("tmp.1");
        assert_eq!(stack_frame.len(), 1);
        stack_frame.get("tmp.2");
        assert_eq!(stack_frame.len(), 2);
        stack_frame.get("tmp.1"); // Same key, shouldn't increase length
        assert_eq!(stack_frame.len(), 2);
    }
}