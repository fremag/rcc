use crate::asm_constructs::operand::Operand;

#[derive(Debug)]
pub struct Stack {
    pub offset: i32
}

impl Operand for Stack {
    fn to_code(&self) -> String {
        todo!()
    }
}