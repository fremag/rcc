use crate::asm_constructs::instruction::StackFrame;

#[derive(Debug)]
pub enum Operand {
    Imm {value: i32},
    Pseudo {identifier: String},
    Register {reg: Reg},
    Stack {offset: i32}
}

#[derive(Debug)]
pub enum Reg {
    AX, R10
}


impl Operand {
    pub fn to_code(&self) -> String {
        match self {
            Operand::Imm { value } =>             String::from(format!("${}", value)),
            Operand::Pseudo { identifier } => String::from(identifier.clone()),
            Operand::Register { reg } => {
                match reg {
                    Reg::AX => String::from("%eax"),
                    Reg::R10 => String::from("%r10d")
                }
            }
            Operand::Stack { offset } => format!("{}(%rbp)", offset)
        }
    }

    pub fn fix_pseudo_registers(&mut self, _pseudo_registers: &mut StackFrame) -> Option<Operand> {
        None
    }


    // Stack
    // fn fix_pseudo_registers(&mut self, _pseudo_registers: &mut StackFrame) -> Option<Box<dyn Operand>> {
    //     let offset = _pseudo_registers.get(&self.identifier);
    //     let stack = Stack { offset };
    //     Some(Box::new(stack))
    // }
}