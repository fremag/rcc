use crate::ast_modele::{Constant, Expression, Function, Program, Return, Statement};
use crate::lexer::Lexer;

pub struct Parser {

    regex : regex::Regex
}

impl Parser {
    pub fn new() -> Self {
        Self { regex: Lexer::identifier_regex() }
    }
    pub fn parse_program(&self, tokens: &mut Vec<String>) -> Result<Program, String> {
        if let Ok(function) = self.parse_function(tokens) {
            Ok( Program{function})
        } else {
            Err("Invalid program".to_string())
        }
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
            let _ = tokens.remove(0);
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

    // <function> ::= "int" <identifier> "(" "void" ")" "{" <statement> "}"
    pub(crate) fn parse_function(&self, tokens: &mut Vec<String>) -> Result<Function, String> {
        if ! Self::check_token(tokens, "int") {
            return Err("nope".to_string());
        }
        let _ = tokens.remove(0);

        let identifier = tokens.remove(0);
        if ! self.check_identifier(&identifier) {
            return Err("Invalid identifier".to_string());
        }

        if ! Self::check_token(tokens, "(") {
            return Err("nope".to_string());
        }
        let _ = tokens.remove(0);

        if ! Self::check_token(tokens, "void") {
            return Err("nope".to_string());
        }
        let _ = tokens.remove(0);

        if ! Self::check_token(tokens, ")") {
            return Err("nope".to_string());
        }
        let _ = tokens.remove(0);

        if ! Self::check_token(tokens, "{") {
            return Err("nope".to_string());
        }
        let _ = tokens.remove(0);

        let result = self.parse_statement(tokens);
        if result.is_err() {
            return Err("nope".to_string());
        }

        if ! Self::check_token(tokens, "}") {
            return Err("nope".to_string());
        }
        let _ = tokens.remove(0);

        let body = result.unwrap();
        Ok(Function{identifier, body})
    }

    fn check_token(tokens: &mut Vec<String>, token: &str) -> bool {
        if tokens.len() == 0 {
            return false;
        }
        tokens[0] == token
    }

    fn check_identifier(&self, token : &String) -> bool {
        if token.len() == 0 {
            return false;
        }
        self.regex.is_match(&token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constant_parser() {
        let parser = Parser::new();
        let mut tokens = vec![ "123".to_string()];

        let constant = parser.parse_constant(&mut tokens);
        assert_eq!(constant.is_ok(), true);
        assert_eq!(constant.unwrap().value, 123);
    }

    #[test]
    fn test_constant_parser_fail_empty() {
        let parser = Parser::new();
        let mut tokens = vec![];
        let constant = parser.parse_constant(&mut tokens);
        assert_eq!(constant.is_err(), true);
    }

    #[test]
    fn test_expression_parser() {
        let parser = Parser::new();
        let mut tokens = vec![ "123".to_string()];

        let expression = parser.parse_expression(&mut tokens);
        assert_eq!(expression.is_ok(), true);
        assert_eq!(expression.unwrap().constant.value, 123);
    }

    #[test]
    fn test_expression_parser_error() {
        let parser = Parser::new();
        let mut tokens = vec![ "return".to_string()];
        let expression = parser.parse_expression(&mut tokens);
        assert_eq!(expression.is_err(), true);
    }

    #[test]
    fn test_return_parser() {
        let parser = Parser::new();
        let mut tokens = vec![ "return".to_string(), "123".to_string(), ";".to_string()];
        let result = parser.parse_return(&mut tokens);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap().expression.constant.value, 123);
    }

    #[test]
    fn test_return_parser_error() {
        let parser = Parser::new();
        let mut tokens = vec![ "123".to_string(), ";".to_string()];
        let result = parser.parse_return(&mut tokens);
        assert_eq!(result.is_ok(), false);
    }

    #[test]
    fn test_return_parser_error_2() {
        let parser = Parser::new();
        let mut tokens = vec!["return".to_string(),  ";".to_string()];
        let result = parser.parse_return(&mut tokens);
        assert_eq!(result.is_ok(), false);
    }

    #[test]
    fn test_return_parser_error_3() {
        let parser = Parser::new();
        let mut tokens = vec!["return".to_string(),  "132".to_string()];
        let result = parser.parse_return(&mut tokens);
        assert_eq!(result.is_ok(), false);
    }

    #[test]
    fn test_statement_parser() {
        let parser = Parser::new();
        let mut tokens = vec![ "return".to_string(), "123".to_string(), ";".to_string()];
        let result = parser.parse_statement(&mut tokens);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap().return_exp.expression.constant.value, 123);
    }

    #[test]
    fn test_function_parser() {
        let parser = Parser::new();
        let mut tokens = vec![ "int".to_string(), "main".to_string(), "(".to_string(), "void".to_string(), ")".to_string(), "{".to_string(),
                           "return".to_string(), "2".to_string(), ";".to_string(), "}".to_string()];

        let result = parser.parse_function(&mut tokens);
        assert_eq!(result.is_ok(), true);
        let function = result.unwrap();
        assert_eq!(function.body.return_exp.expression.constant.value, 2);
        assert_eq!(function.identifier, "main".to_string());
    }
    
    #[test]
    fn test_program_parser() {
        let parser = Parser::new();
        let mut tokens = vec![ "int".to_string(), "main".to_string(), "(".to_string(), "void".to_string(), ")".to_string(), "{".to_string(),
                           "return".to_string(), "2".to_string(), ";".to_string(), "}".to_string()];

        let result = parser.parse_program(&mut tokens);
        assert_eq!(result.is_ok(), true);
        let function = result.unwrap();
        assert_eq!(function.function.body.return_exp.expression.constant.value, 2);
        assert_eq!(function.function.identifier, "main".to_string());
    }
}