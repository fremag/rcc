use crate::ast_model::{AstConstant, AstExpression, AstFunction, AstProgram, AstReturn, AstStatement, AstUnaryOp};
use crate::lexer::Lexer;

pub struct Parser {

    regex : regex::Regex
}

impl Parser {
    pub fn new() -> Self {
        Self { regex: Lexer::identifier_regex() }
    }
    pub fn parse_program(&self, tokens: &mut Vec<String>) -> Result<AstProgram, String> {
        if let Ok(function) = self.parse_function(tokens) {
            Ok( AstProgram {function})
        } else {
            Err("Invalid program".to_string())
        }
    }

    pub fn parse_constant(&self, tokens: &mut Vec<String>) -> Result<AstConstant, String> {
        if tokens.len() == 0 {
            Err("Empty token list".to_string())
        } else {
            let token = tokens.get(0).unwrap();
            if let Ok(value) = token.parse::<i32>() {
                tokens.remove(0);
                Ok(AstConstant {value})
            } else {
                Err("Invalid constant".to_string())
            }
        }
    }

    pub(crate) fn parse_expression(&self, tokens: &mut Vec<String>) -> Result<AstExpression, String> {
        if let Ok(constant) = self.parse_constant(tokens) {
            Ok(AstExpression::Constant(constant))
        } else if tokens[0] == "~" || tokens[0] == "-" {
            if let Ok(op) = self.parse_unop(tokens) {
                if let Ok(exp) = self.parse_expression(tokens) {
                    Ok(AstExpression::Unary(op, Box::new(exp)))
                } else {
                    Err("Invalid unary operator".to_string())
                }
            } else {
                Err("Invalid expression".to_string())
            }
        } else if tokens[0] == "(" {
            tokens.remove(0);
            if let Ok(inner_exp) = self.parse_expression(tokens) {
                let token = tokens.remove(0);
                if token != ")" {
                    Err("Invalid expression".to_string())
                } else {
                    Ok(inner_exp)
                }
            } else {
                Err("Invalid expression".to_string())
            }
        } else {
            Err("Invalid expression".to_string())
        }
    }

    pub(crate) fn parse_unop(&self, tokens: &mut Vec<String>) -> Result<AstUnaryOp, String> {
        let token = tokens.remove(0);
        match token.as_str() {
            "~" => Ok(AstUnaryOp::BitwiseComplement),
            "-" => Ok(AstUnaryOp::Negate),
            _ => Err(format!("Invalid unary operator: {}", &token))
        }
    }

    pub(crate) fn parse_return(&self, tokens: &mut Vec<String>) -> Result<AstReturn, String> {
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
            Ok(AstReturn {expression})

        } else {
            Err("Invalid expression".to_string())
        }
    }

    pub(crate) fn parse_statement(&self, tokens: &mut Vec<String>) -> Result<AstStatement, String> {
        let result = self.parse_return(tokens);
        if let Ok(return_exp) = result {
            Ok(AstStatement {return_exp})

        } else {
            Err("Invalid expression".to_string())
        }
    }

    // <function> ::= "int" <identifier> "(" "void" ")" "{" <statement> "}"
    pub(crate) fn parse_function(&self, tokens: &mut Vec<String>) -> Result<AstFunction, String> {
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
        Ok(AstFunction {identifier, body})
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
    fn test_constant_expression_parser() {
        let parser = Parser::new();
        let mut tokens = vec![ "123".to_string()];

        let expression = parser.parse_expression(&mut tokens);
        assert_eq!(expression.is_ok(), true);
        match expression.unwrap() {
            AstExpression::Constant(cst) => {
                assert_eq!(cst.value, 123);
                return;
            }
            _ => panic!("Invalid expression")
        }
    }

    #[test]
    fn test_unary_expression_parser() {
        let parser = Parser::new();
        let mut tokens = vec![ "~".to_string(), "123".to_string()];

        let expression = parser.parse_expression(&mut tokens);
        assert_eq!(expression.is_ok(), true);
        if let AstExpression::Unary(op, exp) = expression.unwrap() {
            assert_eq!(op, AstUnaryOp::BitwiseComplement);
            match exp.as_ref() {
                AstExpression::Constant(cst) => {
                    assert_eq!(cst.value, 123);
                },
                _ => panic!("Invalid expression")
            }

            return;
        } else {
            panic!("Invalid expression")
        }
    }

    #[test]
    fn test_unary_negate_parser() {
        let parser = Parser::new();
        let mut tokens = vec![ "-".to_string(), "123".to_string()];

        let expression = parser.parse_expression(&mut tokens);
        assert_eq!(expression.is_ok(), true);
        if let AstExpression::Unary(op, exp) = expression.unwrap() {
            assert_eq!(op, AstUnaryOp::Negate);
            match exp.as_ref() {
                AstExpression::Constant(cst) => {
                    assert_eq!(cst.value, 123);
                },
                _ => panic!("Invalid expression")
            }

            return;
        } else {
            panic!("Invalid expression")
        }
    }

    #[test]
    fn test_multi_unary_negate_parser() {
        let parser = Parser::new();
        let mut tokens = vec![ "-".to_string(),"(".to_string(),"~".to_string(), "123".to_string(), ")".to_string()];

        let expression = parser.parse_expression(&mut tokens);
        if let Ok(exp1) = expression
            && let AstExpression::Unary(negate1, sub_exp) = exp1
            && let AstExpression::Unary(bitwise_complement, sub_exp2) = sub_exp.as_ref()
            && let AstExpression::Constant(cst) = sub_exp2.as_ref()
        {
            assert_eq!(negate1, AstUnaryOp::Negate);
            assert_eq!(*bitwise_complement, AstUnaryOp::BitwiseComplement);
            assert_eq!(cst.value, 123);

            assert_eq!(cst.value, 123);
            return;
        }
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
        match result.unwrap().expression {
            AstExpression::Constant(cst) => {
                assert_eq!(cst.value, 123);
                return;
            }
            _ => panic!("Invalid expression")
        }
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
        match result.unwrap().return_exp.expression {
            AstExpression::Constant(cst) => {
                assert_eq!(cst.value, 123);
                return;
            }
            _ => panic!("Invalid expression")
        }
    }

    #[test]
    fn test_function_parser() {
        let parser = Parser::new();
        let mut tokens = vec![ "int".to_string(), "main".to_string(), "(".to_string(), "void".to_string(), ")".to_string(), "{".to_string(),
                           "return".to_string(), "2".to_string(), ";".to_string(), "}".to_string()];

        let result = parser.parse_function(&mut tokens);
        assert_eq!(result.is_ok(), true);
        let function = result.unwrap();
        let expression = function.body.return_exp.expression;
        match expression {
            AstExpression::Constant(cst) => {
                assert_eq!(cst.value, 2);
            }
            _ => panic!("Invalid expression")
        }
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
        let expression = function.function.body.return_exp.expression;
        match expression {
            AstExpression::Constant(cst) => {
                assert_eq!(cst.value, 2);
            },
            AstExpression::Unary(_, _) => todo!()
        }

        assert_eq!(function.function.identifier, "main".to_string());
    }
}