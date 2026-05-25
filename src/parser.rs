use crate::ast_modele::{Constant, Expression, Program, Return, Statement};

pub struct Parser {
    
}

impl Parser {
    pub fn parse_program(&self, tokens: &Vec<String>) -> Result<Program, String> {
        panic!("Not implemented");
    }

    pub fn parse_constant(&self, tokens: &mut Vec<String>) -> Result<Constant, String> {
        if tokens.len() == 0 {
            Err("Empty token list".to_string())
        } else {
            let cst = tokens.remove(0);
            if let Ok(value) = cst.parse::<i32>() {
                Ok(Constant{value})
            } else {
                Err("Invalid constant".to_string())
            }
        }
    }

    pub(crate) fn parse_expression(&self, tokens: &mut Vec<String>) -> Result<Expression, String> {
        if let Ok(constant) = self.parse_constant(tokens) {
            Ok(Expression {constant})
        } else {
            Err("Invalid expression".to_string())
        }
    }

    pub(crate) fn parse_return(&self, tokens: &mut Vec<String>) -> Result<Return, String> {
        if tokens.len() == 0 {
            return Err("Invalid expression".to_string());
        }

        if tokens[0] != "return" {
            return Err("Invalid expression".to_string());
        }

        let _ = tokens.remove(0);

        let result = self.parse_expression(tokens);
        if let Ok(expression) = result {
            if tokens.len() == 0 || tokens[0] != ";" {
                return Err("Invalid expression".to_string());
            }
            Ok(Return{expression})

        } else {
            Err("Invalid expression".to_string())
        }
    }

    pub(crate) fn parse_statement(&self, tokens: &mut Vec<String>) -> Result<Statement, String> {
        let result = self.parse_return(tokens);
        if let Ok(return_exp) = result {
            Ok(Statement{return_exp})

        } else {
            Err("Invalid expression".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_program_parser() {
        let parser = Parser {};
        let tokens = vec![ "int".to_string(), "main".to_string(), "(".to_string(), "void".to_string(), ")".to_string(), "{".to_string(),
                           "return".to_string(), "2".to_string(), ";".to_string(), "}".to_string()];

        let program = parser.parse_program(&tokens);
        assert_eq!(program.is_ok(), false);
    }

    #[test]
    fn test_constant_parser() {
        let parser = Parser {};
        let mut tokens = vec![ "123".to_string()];

        let constant = parser.parse_constant(&mut tokens);
        assert_eq!(constant.is_ok(), true);
        assert_eq!(constant.unwrap().value, 123);
    }

    #[test]
    fn test_constant_parser_fail_empty() {
        let parser = Parser {};
        let mut tokens = vec![];
        let constant = parser.parse_constant(&mut tokens);
        assert_eq!(constant.is_err(), true);
    }

    #[test]
    fn test_expression_parser() {
        let parser = Parser {};
        let mut tokens = vec![ "123".to_string()];

        let expression = parser.parse_expression(&mut tokens);
        assert_eq!(expression.is_ok(), true);
        assert_eq!(expression.unwrap().constant.value, 123);
    }

    #[test]
    fn test_expression_parser_error() {
        let parser = Parser {};
        let mut tokens = vec![ "return".to_string()];
        let expression = parser.parse_expression(&mut tokens);
        assert_eq!(expression.is_err(), true);
    }

    #[test]
    fn test_return_parser() {
        let parser = Parser {};
        let mut tokens = vec![ "return".to_string(), "123".to_string(), ";".to_string()];
        let result = parser.parse_return(&mut tokens);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap().expression.constant.value, 123);
    }

    #[test]
    fn test_return_parser_error() {
        let parser = Parser {};
        let mut tokens = vec![ "123".to_string(), ";".to_string()];
        let result = parser.parse_return(&mut tokens);
        assert_eq!(result.is_ok(), false);
    }

    #[test]
    fn test_return_parser_error_2() {
        let parser = Parser {};
        let mut tokens = vec!["return".to_string(),  ";".to_string()];
        let result = parser.parse_return(&mut tokens);
        assert_eq!(result.is_ok(), false);
    }

    #[test]
    fn test_return_parser_error_3() {
        let parser = Parser {};
        let mut tokens = vec!["return".to_string(),  "132".to_string()];
        let result = parser.parse_return(&mut tokens);
        assert_eq!(result.is_ok(), false);
    }

    #[test]
    fn test_statement_parser() {
        let parser = Parser {};
        let mut tokens = vec![ "return".to_string(), "123".to_string(), ";".to_string()];
        let result = parser.parse_statement(&mut tokens);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap().return_exp.expression.constant.value, 123);
    }

}