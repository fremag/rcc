
pub struct Program {
    function: Function
}

pub struct Function {
    identifier: String,
    body : Statement
}
pub struct Statement {
    return_exp: Return
}

pub struct Return {
    expression: Expression
}

pub struct Expression {
    constant: Constant
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