pub(crate) mod tacky_emit;

#[derive(Debug, Clone, PartialEq)]
pub struct TackyProgram {
    pub(crate) function_def: TackyFunction,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TackyFunction {
    pub(crate) identifier: String,
    pub(crate) body: Vec<TackyInstruction>,
}

#[derive(Debug, Clone, PartialEq)]
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
    ),
    Copy{src: TackyVal, dst: TackyVal},
    Label{identifier: String},
    Jump{target : String},
    JumpIfZero{condition: TackyVal, target : String},
    JumpIfNotZero{condition: TackyVal, target : String}
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
    Not
}

#[derive(Debug, Clone, PartialEq)]
pub enum TackyBinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equal, NotEqual, LessThan, LessOrEqual, GreaterThan, GreaterOrEqual
}
