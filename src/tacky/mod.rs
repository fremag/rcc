pub(crate) mod tacky_emit;

pub struct TackyProgram {
    pub(crate) function_def : TackyFunction
}

pub struct TackyFunction {
    pub(crate) identifier : String,
    pub(crate) body : Vec<TackyInstruction>
} 

pub enum TackyInstruction {
    Return(TackyVal),
    Unary(TackyUnaryOp, TackyVal /* src */, TackyVal /* dst */)
}

#[derive(Debug, Clone, PartialEq)]
pub enum TackyVal {
    Constant(i32),
    Var(String)
}

#[derive(Debug, Clone, PartialEq)]
pub enum TackyUnaryOp {
    Complement,
    Negate
}