
#[derive(Debug)]
pub struct Program {
    pub(crate) function: Function
}

#[derive(Debug)]
pub struct Function {
    pub(crate) identifier: String,
    pub(crate) body : Statement
}
#[derive(Debug)]
pub struct Statement {
    pub(crate) return_exp: Return
}

#[derive(Debug)]
pub struct Return {
    pub(crate) expression: Expression
}

#[derive(Debug)]
pub struct Expression {
    pub(crate) constant: Constant
}
#[derive(Debug)]
pub struct Constant {
    pub(crate) value: i32
}
