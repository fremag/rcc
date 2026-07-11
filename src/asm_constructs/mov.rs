use std::fmt;
use std::fmt::Formatter;
use crate::asm_constructs::operand::Operand;

pub struct Mov {
    pub(crate) src : Box<dyn Operand>,
    pub(crate) dest : Box<dyn Operand>
}

impl fmt::Debug for Mov {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Mov")
            .field("src", &self.src)
            .field("dest", &self.dest)
            .finish()
    }
}