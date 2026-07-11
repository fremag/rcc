pub trait Instruction: std::fmt::Debug {
    fn to_code(&self) -> String;
}