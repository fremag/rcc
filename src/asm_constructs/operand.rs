use crate::asm_constructs::instruction::StackFrame;
use crate::asm_constructs::operand::Operand::Stack;

#[derive(Debug, Clone)]
pub enum Operand {
    Imm { value: i32 },
    Pseudo { identifier: String },
    Register { reg: Reg },
    Stack { offset: usize },
}

#[derive(Debug, Clone)]
pub enum Reg {
    AX,
    R10,
}

impl Operand {
    pub fn to_code(&self) -> String {
        match self {
            Operand::Imm { value } => String::from(format!("${}", value)),
            Operand::Pseudo { identifier } => String::from(identifier.clone()),
            Operand::Register { reg } => match reg {
                Reg::AX => String::from("%eax"),
                Reg::R10 => String::from("%r10d"),
            },
            Operand::Stack { offset } => format!("{}(%rbp)", offset),
        }
    }

    pub fn fix_pseudo_registers(&self, _pseudo_registers: &mut StackFrame) -> Operand {
        match self {
            Operand::Pseudo { identifier } => {
                let offset = _pseudo_registers.get(&identifier);
                Stack { offset }
            }
            _ => self.clone(),
        }
    }
}
