use crate::asm_constructs::instruction::{Instruction, StackFrame};
use crate::asm_constructs::operand::Operand;

#[derive(Debug)]
pub struct Mov {
    pub(crate) src : Operand,
    pub(crate) dest : Operand
}

impl Instruction for Mov {
    fn to_code(&self) -> String {
        format!("movl {}, {}", self.src.to_code(), self.dest.to_code())
    }

    fn fix_pseudo_registers(&mut self, _pseudo_registers: &mut StackFrame) {
        let new_src = self.src.fix_pseudo_registers(_pseudo_registers);
        if let Some(new_src) = new_src {
            self.src = new_src;
        }

        let new_dest = self.src.fix_pseudo_registers(_pseudo_registers);
        if let Some(new_dest) = new_dest {
            self.dest = new_dest;
        }
    }
}