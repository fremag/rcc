use std::collections::{HashMap};
use crate::ast_model::constant::AstConstant;
use crate::ast_model::expression::AstExpression;
use crate::ast_model::function::{AstBlockItem, AstDeclaration, AstFunction};
use crate::ast_model::program::AstProgram;
use crate::ast_model::statement::AstStatement;

pub struct Resolver {}

impl Resolver {
    pub(crate) fn new() -> Self {
            Self {}
        }

    pub(crate) fn resolve(&self, ast_program: &AstProgram) -> Result<AstProgram, String> {
        let result = self.resolve_function(&ast_program.function);
        match result {
            Ok(main_function) => Ok(AstProgram {function: main_function}),
            Err(error) => Err(error)
        }
    }

    fn resolve_function(&self, ast_function: &AstFunction) -> Result<AstFunction, String> {
        let mut variable_map = HashMap::<String, String>::new();
        let mut body : Vec<AstBlockItem>=  Vec::new();
        for block_item in ast_function.body.iter() {
            let ast_block_item : AstBlockItem = match block_item {
                AstBlockItem::Statement(statement) => {
                    let result = self.resolve_statement(&statement, &mut variable_map);
                    match result {
                        Ok(resolved_statement) => AstBlockItem::Statement(resolved_statement),
                        Err(msg) => return Err(msg)
                    }
                }
                AstBlockItem::Declaration(declaration) => {
                    let result = self.resolve_declaration(declaration, &mut variable_map);
                    match result {
                        Ok(resolved_declaration) => AstBlockItem::Declaration(resolved_declaration),
                        Err(msg) => return Err(msg)
                    }
                }
            };
            body.push(ast_block_item);
        }

        Ok(AstFunction {identifier: ast_function.identifier.clone(), body})
    }

    fn resolve_declaration(&self, ast_declaration: &AstDeclaration, variable_map: &mut HashMap<String, String>) -> Result<AstDeclaration, String> {
        let name = &ast_declaration.identifier;
        if variable_map.contains_key(name) {
            return Err(format!("Variable already declared: {name} "));
        }

        let nb_variables = variable_map.len();
        let unique_hame = format!("{name}-{nb_variables}");
        variable_map.insert(name.clone(), unique_hame.clone());

        if let Some(init_expression) = &ast_declaration.init {
            if let Ok(resolved_init_expression) = self.resolve_expression(&init_expression, variable_map) {
                Ok(AstDeclaration {identifier: unique_hame, init: Some(resolved_init_expression)})
            } else {
                Err(format!("Failed to resolve expression: {name}"))
            }
        } else {
            Ok(AstDeclaration {identifier: unique_hame, init: None })
        }
    }

    fn resolve_expression(&self, ast_expression: &AstExpression, variable_map: &HashMap<String, String>) -> Result<AstExpression, String> {
        match ast_expression {
            AstExpression::Constant { constant } => Ok(AstExpression::Constant {constant: AstConstant {value: constant.value }}),
            AstExpression::Var { identifier } => {
                if ! variable_map.contains_key(identifier) {
                    Err(format!("Undeclared variable ! {identifier} "))
                } else {
                    let unique_variable_name = variable_map[identifier].clone();
                    Ok(AstExpression::Var {identifier: unique_variable_name })
                }
            }
            AstExpression::Unary { unary_op, factor } => {
                match self.resolve_expression(factor.as_ref(), variable_map) {
                    Ok(resolved_factor) => Ok( AstExpression::Unary {unary_op: unary_op.clone(), factor: Box::new(resolved_factor)} ),
                    Err(msg) => Err(msg)
                }
            }
            AstExpression::Binary { binop, left, right  } => {
                let result_left = self.resolve_expression(left.as_ref(), variable_map);
                if let Err(msg) = result_left {
                    return Err(msg);
                }
                let result_right = self.resolve_expression(right.as_ref(), variable_map);
                if let Err(msg) = result_right {
                    return Err(msg);
                }

                Ok(AstExpression::Binary {binop: binop.clone(), right: Box::new(result_right.unwrap()), left: Box::new(result_left.unwrap())})
            }
            AstExpression::Assignment { left, right } => {
                match left.as_ref() {
                    AstExpression::Var { .. } => {}
                    _ => {
                        let left_exp = left.as_ref();
                        return Err(format!("Invalid assignment, left is not a var identifier ! {left_exp:?}"));
                    }
                }
                let result_left = self.resolve_expression(left.as_ref(), variable_map);
                if let Err(msg) = result_left {
                    return Err(msg);
                }
                let result_right = self.resolve_expression(right.as_ref(), variable_map);
                if let Err(msg) = result_right {
                    return Err(msg);
                }
                Ok(AstExpression::Assignment {left: Box::new(result_left.unwrap()), right: Box::new(result_right.unwrap())})
            }
        }
    }

    fn resolve_statement(&self, ast_statement: &AstStatement, variable_map: &mut HashMap<String, String>) -> Result<AstStatement, String> {
        match ast_statement {
            AstStatement::Return { expression } => {
                let result = self.resolve_expression(&expression, variable_map);
                match result {
                    Ok(expression) => return Ok(AstStatement::Return {expression: expression}),
                    Err(msg) => Err(msg)
                }
            }
            AstStatement::Expression { expression} => {
                let result = self.resolve_expression(&expression, variable_map);
                match result {
                    Ok(expression) => return Ok(AstStatement::Expression {expression: expression}),
                    Err(msg) => Err(msg)
                }
            }
            AstStatement::Null => Ok(AstStatement::Null)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    use super::*;
    use test_case::test_case;

    #[test]
    fn test_resolve_declaration() {
        let result = resolve("return 42;");
        assert!(result.is_ok());
        let expected = vec!["Statement(Return { expression: Constant { constant: AstConstant { value: 42 } } })"];
        check(result.unwrap(), expected)
    }

    #[test]
    fn test_resolve_declaration_1() {
        let result = resolve("int x = 42; return x;");
        assert!(result.is_ok());
        let expected = vec![
            "Declaration(AstDeclaration { identifier: \"x-0\", init: Some(Constant { constant: AstConstant { value: 42 } }) })",
            "Statement(Return { expression: Var { identifier: \"x-0\" } })"
        ];

        check(result.unwrap(), expected)
    }

    #[test_case("return x;", "Undeclared variable ! x ")]
    #[test_case("int 42 = 10;", "Invalid program: Invalid block: Invalid identifier: 42")]
    pub fn test_failed_resolve_declaration(code : &str, expected_error : &str) {
        let result = resolve(code);
        assert!(result.is_err());
        assert_eq!(format!("{}", result.unwrap_err()), expected_error);
    }

    pub fn resolve(code : &str) -> Result<Vec<String>, String> {
        let program: String = format!("int main(void) {{{code}}}");
        let lexer = Lexer::new(program);
        let mut tokens = lexer.tokenize().unwrap();
        let parser = Parser::new();
        let program_result = parser.parse_program(&mut tokens);
        if let Err(error) = program_result {
            return Err(error);
        }
        
        let resolver = Resolver::new();
        let resolved_function =  resolver.resolve_function(&program_result.unwrap().function);

        match resolved_function {
            Ok(function) => {
                let txt : Vec<String>= function.body.iter().map(|block_item | format!("{block_item:?}")).collect();
                Ok(txt)
            }
            Err(msg) => Err(msg)
        }
    }

    pub fn check(lines1 : Vec<String>, lines2 : Vec<&str>) {
        if lines1.len() != lines2.len() {
            panic!("{lines1:?}\n{lines2:?}")
        }

        for (i, line) in lines1.iter().enumerate() {
            let other_line = lines2.get(i).unwrap();
            assert_eq!(line, other_line);
        }
    }
}