pub(crate) mod tacky_emit;

pub struct TackyProgram {
    pub(crate) function_def: TackyFunction,
}

pub struct TackyFunction {
    pub(crate) identifier: String,
    pub(crate) body: Vec<TackyInstruction>,
}

pub enum TackyInstruction {
    Return(TackyVal),
    Unary(
        TackyUnaryOp,
        TackyVal, /* src */
        TackyVal, /* dst */
    ),
    Binary(
        TackyBinaryOp,
        TackyVal, /* src 1 */
        TackyVal, /* src 2 */
        TackyVal, /* dst */
    )
}

#[derive(Debug, Clone, PartialEq)]
pub enum TackyVal {
    Constant(i32),
    Var(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TackyUnaryOp {
    Complement,
    Negate,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TackyBinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
}
