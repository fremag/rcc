use AstStatement::Return;
use crate::ast_model::constant::AstConstant;
use crate::ast_model::expression::{AstExpression, AstBinaryOp, AstUnaryOp};
use crate::ast_model::function::{AstBlockItem, AstDeclaration, AstFunction};
use crate::ast_model::program::AstProgram;
use crate::ast_model::statement::{AstStatement};
use crate::lexer::Lexer;

pub struct Parser {
    regex: regex::Regex,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            regex: Lexer::identifier_regex(),
        }
    }
    pub fn parse_program(&self, tokens: &mut Vec<String>) -> Result<AstProgram, String> {
        if let Ok(function) = self.parse_function_definition(tokens) {
            Ok(AstProgram { function })
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
                Ok(AstConstant { value })
            } else {
                Err("Invalid constant".to_string())
            }
        }
    }

    pub(crate) fn parse_factor(&self, tokens: &mut Vec<String>) -> Result<AstExpression, String> {
        if tokens.len() == 0 {
            return Err("Empty token list".to_string())
        }
        
        if let Ok(constant) = self.parse_constant(tokens) {
            let f = AstExpression::Constant { constant };
            Ok(f)
        } else if Self::check_token(tokens, "~") 
            || Self::check_token(tokens, "-") 
            || Self::check_token(tokens, "!") {
            if let Ok(op) = self.parse_unop(tokens) {
                if let Ok(inner_exp) = self.parse_factor(tokens) {
                    Ok(AstExpression::Unary {
                        unary_op: op,
                        factor: Box::new(inner_exp),
                    })
                } else {
                    Err("Invalid unary operator".to_string())
                }
            } else {
                Err("Invalid expression".to_string())
            }
        } else if Self::check_token(tokens,"(") {
            tokens.remove(0);
            if let Ok(inner_exp) = self.parse_expression(tokens, 0) {
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
            let identifier = tokens.remove(0);
            Ok(AstExpression::Var {identifier })
        }
    }

    pub(crate) fn parse_expression(
        &self,
        tokens: &mut Vec<String>,
        min_prec: i32
    ) -> Result<AstExpression, String> {
        let factor = self.parse_factor(tokens);
        if let Ok(left_factor) = factor {
            let mut left = left_factor;
            let mut next_token = Self::peek_token(tokens);
            while Self::is_binary_op(&next_token) && Self::precedence(&next_token) >= min_prec {
                if next_token == "=" {
                   let _ = tokens.remove(0);
                    let next_token_precedence = Self::precedence(&next_token) ;
                    let right_exp = self.parse_expression(tokens, next_token_precedence);
                    if let Ok(right) = right_exp {
                        left = AstExpression::Assignment { left: Box::new(left), right: Box::new(right) };
                    } else {
                        return Err("Invalid expression".to_string());
                    }
                } else {
                    let binop = self.parse_binop(tokens);
                    let next_token_precedence = Self::precedence(&next_token) + 1;
                    let right_exp = self.parse_expression(tokens, next_token_precedence);
                    if let Ok(right) = right_exp {
                        left = AstExpression::Binary { binop, left: Box::new(left), right: Box::new(right) };
                    } else {
                        return Err("Invalid expression".to_string());
                    }
                }
                next_token = Self::peek_token(tokens);
            }
            Ok(left)
        } else if let Err(msg) = factor {
            Err(format!("Invalid expression: '{msg}'"))
        } else {
            Err("Invalid expression".to_string())
        }
    }

    pub(crate) fn parse_unop(&self, tokens: &mut Vec<String>) -> Result<AstUnaryOp, String> {
        let token = tokens.remove(0);
        match token.as_str() {
            "~" => Ok(AstUnaryOp::BitwiseComplement),
            "-" => Ok(AstUnaryOp::Negate),
            "!" => Ok(AstUnaryOp::Not),
            _ => Err(format!("Invalid unary operator: {}", &token)),
        }
    }

    pub(crate) fn parse_return(&self, tokens: &mut Vec<String>) -> Result<AstStatement, String> {
        if tokens.len() == 0 {
            return Err("Invalid expression".to_string());
        }

        if tokens[0] != "return" {
            return Err("Invalid expression".to_string());
        }

        let _ = tokens.remove(0);

        let result = self.parse_expression(tokens, 0);
        if let Ok(expression) = result {
            if tokens.len() == 0 || tokens[0] != ";" {
                return Err("Invalid expression".to_string());
            }
            let _ = tokens.remove(0);
            Ok(Return { expression })
        } else {
            Err("Invalid expression".to_string())
        }
    }

    pub(crate) fn parse_statement(&self, tokens: &mut Vec<String>) -> Result<AstStatement, String> {
        let result = self.parse_return(tokens);
        if let Ok(AstStatement::Return {expression})  = result {
            Ok(Return { expression})
        } else {
            Err("Invalid expression".to_string())
        }
    }

    // <function> ::= "int" <identifier> "(" "void" ")" "{" <statement> "}"
    pub(crate) fn parse_function_definition(&self, tokens: &mut Vec<String>) -> Result<AstFunction, String> {
        if !Self::check_token(tokens, "int") {
            return Err("nope".to_string());
        }
        let _ = tokens.remove(0);

        let identifier = tokens.remove(0);
        if !self.check_identifier(&identifier) {
            return Err("Invalid identifier".to_string());
        }

        if !Self::check_token(tokens, "(") {
            return Err("nope".to_string());
        }
        let _ = tokens.remove(0);

        if !Self::check_token(tokens, "void") {
            return Err("nope".to_string());
        }
        let _ = tokens.remove(0);

        if !Self::check_token(tokens, ")") {
            return Err("nope".to_string());
        }
        let _ = tokens.remove(0);

        if !Self::check_token(tokens, "{") {
            return Err("nope".to_string());
        }
        let _ = tokens.remove(0);

        let result = self.parse_function_body(tokens);
        if result.is_err() {
            return Err("nope".to_string());
        }

        if !Self::check_token(tokens, "}") {
            return Err("nope".to_string());
        }
        let _ = tokens.remove(0);

        let body = result.unwrap();
        Ok(AstFunction { identifier, body })
    }

    fn check_token(tokens: &mut Vec<String>, token: &str) -> bool {
        if tokens.len() == 0 {
            return false;
        }
        tokens[0] == token
    }

    fn check_identifier(&self, token: &String) -> bool {
        if token.len() == 0 {
            return false;
        }
        self.regex.is_match(&token)
    }

    fn parse_binop(&self, tokens: &mut Vec<String>) -> AstBinaryOp {
        let token =        tokens.remove(0);
        match token.as_str() {
            "+" => AstBinaryOp::Add,
            "-" => AstBinaryOp::Sub,
            "*" => AstBinaryOp::Mul,
            "/" => AstBinaryOp::Div,
            "%" => AstBinaryOp::Mod,
            ">" => AstBinaryOp::GreaterThan,
            ">=" => AstBinaryOp::GreaterThanEqual,
            "<" => AstBinaryOp::LessThan,
            "<=" => AstBinaryOp::LessThanEqual,
            "==" => AstBinaryOp::Equal,
            "!=" => AstBinaryOp::NotEqual,
            "&&" => AstBinaryOp::And,
            "||" => AstBinaryOp::Or,
            _ => panic!("Invalid binary operator ! {}", token.as_str())
        }
    }

    fn is_binary_op(token: &String) -> bool {
        token == "+" || token == "-" || token == "*" || token == "/" || token == "%" || 
        token == "<" || token == "<=" || token == ">" || token == ">=" ||
        token == "==" || token == "!=" || token == "&&" || token == "||" 
    }

    fn precedence(token: &String) -> i32 {
        match token.as_str() {
            "*" | "/" | "%" => 50,
            "+" | "-" => 45,
            "<" | "<=" | ">" | ">=" => 35,
            "==" | "!=" => 30,
            "&&" => 10,
            "||" => 5,
            "=" => 1,
            _ => panic!("Unknown precedence ! ({})", token)
        }
    }

    fn peek_token(tokens: &Vec<String>) -> String {
        if tokens.len() > 0 { tokens[0].clone()} else { "".to_string() }
    }

    fn parse_function_body(&self, tokens: &mut Vec<String>) -> Result<Vec<AstBlockItem>, String> {
        let mut block_items : Vec<AstBlockItem>= vec![];
        while ! Self::check_token(tokens, "}") {
            if let Ok(next_block_item) = self.parse_block_item(tokens) {
                block_items.push(next_block_item);
            } else {
                return Err("Invalid block".to_string());
            }
        }

        Ok(block_items)
    }

    fn parse_block_item(&self, tokens: &mut Vec<String>) -> Result<AstBlockItem, String> {
        let block_item;
        if Self::check_token(tokens, "int") {
            let _ = tokens.remove(0);
            let identifier = tokens.remove(0);
            if Self::check_token(tokens, "=") {
                let _ = tokens.remove(0);
                let init_exp = self.parse_expression(tokens, 0);
                if let Ok(expression) = init_exp {
                    block_item = AstBlockItem::Declaration(AstDeclaration { identifier, init: Some(expression)})
                } else {
                    return Err("Invalid expression".to_string());
                }
            } else {
                block_item = AstBlockItem::Declaration(AstDeclaration { identifier, init: None })
            }
        } else {
            let statement = self.parse_statement(tokens);
            if let Ok(statement) = statement {
                block_item = AstBlockItem::Statement(statement)
            } else {
                return Err("Invalid statement".to_string());
            }
        }

        Ok(block_item)
    }
}

#[cfg(test)]
mod tests {
    use crate::ast_model::expression::AstBinaryOp;
    use super::*;

    #[test]
    fn test_constant_parser() {
        let parser = Parser::new();
        let mut tokens = vec!["123".to_string()];

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
    fn test_constant_factor_parser() {
        let parser = Parser::new();
        let mut tokens = vec!["123".to_string()];

        let factor = parser.parse_factor(&mut tokens);
        assert_eq!(factor.is_ok(), true);
        match factor.unwrap() {
            AstExpression::Constant { constant: cst } => {
                assert_eq!(cst.value, 123);
                return;
            }
            _ => panic!("Invalid expression"),
        }
    }

    #[test]
    fn test_unary_factor_parser() {
        let parser = Parser::new();
        let mut tokens = vec!["~".to_string(), "123".to_string()];

        let factor = parser.parse_factor(&mut tokens);
        assert_eq!(factor.is_ok(), true);
        if let AstExpression::Unary {
            unary_op,
            factor,
        } = factor.unwrap()
        {
            assert_eq!(unary_op, AstUnaryOp::BitwiseComplement);
            match factor.as_ref() {
                AstExpression::Constant { constant: cst } => {
                    assert_eq!(cst.value, 123);
                }
                _ => panic!("Invalid expression"),
            }

            return;
        } else {
            panic!("Invalid expression")
        }
    }

    #[test]
    fn test_unary_negate_parser() {
        let parser = Parser::new();
        let mut tokens = vec!["-".to_string(), "123".to_string()];

        let factor = parser.parse_factor(&mut tokens);
        assert_eq!(factor.is_ok(), true);
        if  let AstExpression::Unary {
            unary_op,
            factor,
        } = factor.unwrap()
        {
            assert_eq!(unary_op, AstUnaryOp::Negate);
            match factor.as_ref() {
                AstExpression::Constant { constant: cst } => {
                    assert_eq!(cst.value, 123);
                }
                _ => panic!("Invalid expression"),
            }

            return;
        } else {
            panic!("Invalid expression")
        }
    }

    #[test]
    fn test_multi_unary_negate_parser() {
        let parser = Parser::new();
        let mut tokens = vec![
            "-".to_string(),
            "(".to_string(),
            "~".to_string(),
            "123".to_string(),
            ")".to_string(),
        ];

        let factor = parser.parse_factor(&mut tokens);
        if let Ok(exp1) = factor
            && let AstExpression::Unary {
                unary_op: negate1,
                factor: factor1,
            } = exp1
            && let AstExpression::Unary {
                unary_op: bitwise_complement,
                factor: sub_factor2,
            } = factor1.as_ref()
            && let AstExpression::Constant { constant: cst } = sub_factor2.as_ref()
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
        let mut tokens = vec!["return".to_string()];
        let expression = parser.parse_expression(&mut tokens, 0);
        assert_eq!(expression.is_err(), true);
    }

    #[test]
    fn test_return_parser() {
        let parser = Parser::new();
        let mut tokens = vec!["return".to_string(), "123".to_string(), ";".to_string()];
        let result = parser.parse_return(&mut tokens);
        assert_eq!(result.is_ok(), true);
        if let Return { expression : AstExpression::Constant{constant : cst} } = result.unwrap() {
            assert_eq!(cst.value, 123);
        } else {
            panic!("Invalid expression")
        }
    }


    #[test]
    fn test_return_wrong_order_parser_error() {
        // based on writing-a-c-compiler-tests/tests/chapter_2/invalid_parse/wrong_order.c
        let parser = Parser::new();
        let mut tokens = vec!["return", "4", "-", ";"].iter().map(|s| s.to_string()).collect();
        let result = parser.parse_return(&mut tokens);
        assert_eq!(result.is_ok(), false);
    }

    #[test]
    fn test_return_parser_error() {
        let parser = Parser::new();
        let mut tokens = vec!["123".to_string(), ";".to_string()];
        let result = parser.parse_return(&mut tokens);
        assert_eq!(result.is_ok(), false);
    }

    #[test]
    fn test_return_parser_error_2() {
        let parser = Parser::new();
        let mut tokens = vec!["return".to_string(), ";".to_string()];
        let result = parser.parse_return(&mut tokens);
        assert_eq!(result.is_ok(), false);
    }

    #[test]
    fn test_return_parser_error_3() {
        let parser = Parser::new();
        let mut tokens = vec!["return".to_string(), "132".to_string()];
        let result = parser.parse_return(&mut tokens);
        assert_eq!(result.is_ok(), false);
    }

    #[test]
    fn test_statement_parser() {
        let parser = Parser::new();
        let mut tokens = vec!["return".to_string(), "123".to_string(), ";".to_string()];
        let result = parser.parse_statement(&mut tokens);
        assert_eq!(result.is_ok(), true);
        let statement = result.unwrap();
        let expression = match statement  {
            AstStatement::Return { expression } => expression,
            _ => panic!("Invalid statement"),
        };
        match expression {
                AstExpression::Constant { constant: cst } => {
                    assert_eq!(cst.value, 123);
                }
            _ => panic!("Invalid expression"),
        }
    }
    #[test]
    fn test_function_parser() {
        let parser = Parser::new();
        let mut tokens = vec![
            "int",
            "main",
            "(",
            "void",
            ")",
            "{",
            "return",
            "2",
            ";",
            "}"
        ].iter().map(|s| s.to_string()).collect();

        let result = parser.parse_function_definition(&mut tokens);
        assert_eq!(result.is_ok(), true);
        let function = result.unwrap();
        let ast_statement = function.body.get(0).unwrap();
        let expression = match ast_statement {
            AstBlockItem::Statement(AstStatement::Return { expression }) => expression,
            _ => panic!("Invalid statement"),
        };
        if let crate::ast_model::expression::AstExpression::Constant{constant: cst} = expression  {
            assert_eq!(cst.value, 2);
        } else {
            panic!("Invalid expression")
        }
        assert_eq!(function.identifier, "main".to_string());
    }

    #[test]
    fn test_program_parser() {
        let parser = Parser::new();
        let mut tokens = vec![
            "int".to_string(),
            "main".to_string(),
            "(".to_string(),
            "void".to_string(),
            ")".to_string(),
            "{".to_string(),
            "return".to_string(),
            "2".to_string(),
            ";".to_string(),
            "}".to_string(),
        ];

        let result = parser.parse_program(&mut tokens);
        assert_eq!(result.is_ok(), true);
        let program = result.unwrap();
        let expression = match program.function.body.get(0).unwrap() {
            AstBlockItem::Statement(AstStatement::Return { expression }) => expression,
            _ => panic!("Invalid statement"),
        };
        if let AstExpression::Constant { constant: cst } = expression  {
            assert_eq!(cst.value, 2);
        } else {
            panic!("Invalid expression")
        }

        assert_eq!(program.function.identifier, "main".to_string());
    }

    #[test]
    fn test_parse_exp_binary_operator() {
        let parser = Parser::new();
        let mut tokens = vec!["1", "+", "2"].iter().map(|s| s.to_string()).collect();
        let result= parser.parse_expression(&mut tokens, 0);
        assert_eq!(result.is_ok(), true);
        if let AstExpression::Binary {binop, left, right} = result.unwrap()
        && let right_factor = right.as_ref()
        && let left_factor = left.as_ref()
            && let AstExpression::Constant{constant: right_cst} = right_factor
            && let AstExpression::Constant{constant: left_cst} = left_factor
        {
            assert_eq!(binop, AstBinaryOp::Add);
            assert_eq!(left_cst.value, 1);
            assert_eq!(right_cst.value, 2);
        } else {
            panic!("Something failed !")
        }
    }

    #[test]
    fn test_parse_factor_binary_operator_parentheses() {
        let parser = Parser::new();
        let mut tokens = vec!["(", "1", "+", "2", ")"].iter().map(|s| s.to_string()).collect();
        let result= parser.parse_factor(&mut tokens);
        assert_eq!(result.is_ok(), true);
        if  let AstExpression::Binary {binop, left, right} = result.unwrap()
            && let AstExpression::Constant{constant: right_cst} = right.as_ref()
            && let AstExpression::Constant{constant: left_cst} = left.as_ref()
        {
            assert_eq!(binop, AstBinaryOp::Add);
            assert_eq!(left_cst.value, 1);
            assert_eq!(right_cst.value, 2);
        } else {
            panic!("Something failed !")
        }
    }

    #[test]
    fn test_parse_exp_binary_operator_nested_expression() {
        let parser = Parser::new();
        let mut tokens = vec!["1", "+", "(", "2", "-", "3", ")"].iter().map(|s| s.to_string()).collect();
        let result= parser.parse_expression(&mut tokens, 0);
        assert_eq!(result.is_ok(), true);
        if let AstExpression::Binary {binop: binop1, left, right} = result.unwrap()
            && let AstExpression::Constant{constant: left_cst} = left.as_ref()
            && let AstExpression::Binary {binop: binop2, left : nested_left, right: nested_right} = right.as_ref()
            && let AstExpression::Constant{constant: left_nested_cst} = nested_left.as_ref()
            && let AstExpression::Constant{constant: right_nested_cst} = nested_right.as_ref()
        {
            assert_eq!(left_cst.value, 1);
            assert_eq!(left_nested_cst.value, 2);
            assert_eq!(right_nested_cst.value, 3);
            assert_eq!(binop1.clone(), AstBinaryOp::Add);
            assert_eq!(binop2.clone(), AstBinaryOp::Sub);
        } else {
            panic!("Something failed !")
        }
    }

    #[test]
    fn test_parse_exp_binary_operator_precedence() {
        let parser = Parser::new();
        let mut tokens = vec!["1", "*", "2", "+", "3"].iter().map(|s| s.to_string()).collect();
        let result= parser.parse_expression(&mut tokens, 0);
        assert_eq!(result.is_ok(), true);
        let result_expression = result.unwrap();
        if let AstExpression::Binary {binop: binop1, left: left1, right: right1} = result_expression
            && let AstExpression::Binary{binop: binop2, left: left2, right: right2} = left1.as_ref()
            && let AstExpression::Constant{constant: cst1} = right1.as_ref()
            && let AstExpression::Constant{constant: cst2} = right2.as_ref()
            && let AstExpression::Constant{constant: cst3} = left2.as_ref()
        {
            assert_eq!(binop1.clone(), AstBinaryOp::Add);
            assert_eq!(binop2.clone(), AstBinaryOp::Mul);
            assert_eq!(cst1.value, 3);
            assert_eq!(cst2.value, 2);
            assert_eq!(cst3.value, 1);
        } else {
            panic!("Something failed !")
        }
    }

    #[test]
    fn test_parse_exp_binary_operator_error() {
        let parser = Parser::new();
        let mut tokens = vec!["4", "-"].iter().map(|s| s.to_string()).collect();
        let result= parser.parse_expression(&mut tokens, 0);
        assert_eq!(result.is_ok(), false);
    }
}
