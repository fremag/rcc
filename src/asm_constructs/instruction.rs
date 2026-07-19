use crate::asm_constructs::operand::Operand;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum UnaryOperator {
    Neg,
    Not,
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
}

impl Instruction {
    pub fn to_code(&self) -> String {
        match self {
            Instruction::AllocateStack { size } => format!("sub rsp, {}\n", size),
            Instruction::Mov { src, dest } => {
                format!("movl {}, {}\n", src.to_code(), dest.to_code())
            }
            Instruction::Unary {
                unary_operator,
                operand,
            } => {
                let unary = match unary_operator {
                    UnaryOperator::Neg => "neg1",
                    UnaryOperator::Not => "not1",
                };
                format!("{} {}\n", unary, operand.to_code())
            }
            Instruction::Ret => String::from("movq %rbp, %rsp\n\tpopq %rbp\n\tret\n"),
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
            } => Instruction::Unary {
                unary_operator: unary_operator.clone(),
                operand: operand.clone(),
            },
            Instruction::AllocateStack { size } => Instruction::AllocateStack { size: *size },
            Instruction::Ret => Instruction::Ret,
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
            Some(value) => *value,
            None => {
                let n = self.items.len() + 1;
                self.items.insert(key.to_string(), n);
                n
            }
        }
    }
}
