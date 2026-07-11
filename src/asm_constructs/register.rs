#[derive(Debug)]
pub enum Reg {
    AX, R10
}

#[derive(Debug)]
pub struct Register {
    pub(crate) reg: Reg
}