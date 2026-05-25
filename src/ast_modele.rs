
pub struct Program {
    pub(crate) function: Function
}

pub struct Function {
    pub(crate) identifier: String,
    pub(crate) body : Statement
}
pub struct Statement {
    pub(crate) return_exp: Return
}

pub struct Return {
    pub(crate) expression: Expression
}

pub struct Expression {
    pub(crate) constant: Constant
}
pub struct Constant {
    pub(crate) value: i32
}

#[cfg(test)]
mod tests {
    #[test]
    pub fn test_ast_node() {

    }
}