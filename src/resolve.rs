use std::collections::{HashMap};
use crate::ast_model::constant::AstConstant;
use crate::ast_model::expression::AstExpression;
use crate::ast_model::function::{AstBlock, AstBlockItem, AstDeclaration, AstFunction};
use crate::ast_model::program::AstProgram;
use crate::ast_model::statement::{AstForInit, AstStatement};

#[derive(Debug, Clone)]
pub struct VariableInfo {
    pub new_name: String,
    pub from_current_block: bool,
}

pub struct Resolver {}

impl Resolver {
    pub(crate) fn new() -> Self {
            Self {}
        }

    pub(crate) fn resolve(&self, ast_program: &AstProgram) -> Result<AstProgram, String> {
        let main_function = self.resolve_function(&ast_program.function)?;
        Ok(AstProgram {function: main_function})
    }

    fn resolve_function(&self, ast_function: &AstFunction) -> Result<AstFunction, String> {
        let mut variable_map = HashMap::<String, VariableInfo>::new();
        let resolved_block = self.resolve_block(&ast_function.body, &mut variable_map)?;
        Ok(AstFunction {identifier: ast_function.identifier.clone(), body: resolved_block })
    }

    fn resolve_declaration(&self, ast_declaration: &AstDeclaration, variable_map: &mut HashMap<String, VariableInfo>) -> Result<AstDeclaration, String> {
        let name = &ast_declaration.identifier;
        if let Some(variable_info) = variable_map.get(name)  && variable_info.from_current_block {
            return Err(format!("Variable already declared: {name} "));
        }

        let nb_variables = variable_map.len();
        let unique_hame = format!("{name}-{nb_variables}");
        variable_map.insert(name.clone(), VariableInfo {new_name: unique_hame.clone(), from_current_block: true});

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

    fn resolve_expression(&self, ast_expression: &AstExpression, variable_map: &HashMap<String, VariableInfo>) -> Result<AstExpression, String> {
        match ast_expression {
            AstExpression::Constant { constant } => Ok(AstExpression::Constant {constant: AstConstant {value: constant.value }}),
            AstExpression::Var { identifier } => {
                if ! variable_map.contains_key(identifier) {
                    Err(format!("Undeclared variable ! {identifier} "))
                } else {
                    let unique_variable_name = variable_map[identifier].new_name.clone();
                    Ok(AstExpression::Var {identifier: unique_variable_name })
                }
            }
            AstExpression::Unary { unary_op, factor } => {
                let resolved_factor =  self.resolve_expression(factor.as_ref(), variable_map)?;
                Ok( AstExpression::Unary {unary_op: unary_op.clone(), factor: Box::new(resolved_factor)} )
            }
            AstExpression::Binary { binop, left, right  } => {
                let result_left = self.resolve_expression(left.as_ref(), variable_map)?;
                let result_right = self.resolve_expression(right.as_ref(), variable_map)?;
                Ok(AstExpression::Binary {binop: binop.clone(), right: Box::new(result_right), left: Box::new(result_left)})
            }
            AstExpression::Assignment { left, right } => {
                if let AstExpression::Var { .. } = left.as_ref() {} else {
                    let left_exp = left.as_ref();
                    return Err(format!("Invalid assignment, left is not a var identifier ! {left_exp:?}"));
                }
                let result_left = self.resolve_expression(left.as_ref(), variable_map)?;
                let result_right = self.resolve_expression(right.as_ref(), variable_map)?;
                Ok(AstExpression::Assignment {left: Box::new(result_left), right: Box::new(result_right)})
            }
            AstExpression::PrefixIncrement { factor } => {
                let resolved_factor = Self::check_variable(factor, variable_map)?;
                Ok(AstExpression::PrefixIncrement {factor: Box::new(resolved_factor)})
            },
            AstExpression::PrefixDecrement { factor } => {
                let resolved_factor = Self::check_variable(factor, variable_map)?;
                Ok(AstExpression::PrefixDecrement {factor: Box::new(resolved_factor)})
            },
            AstExpression::PostfixIncrement { factor } => {
                let resolved_factor = Self::check_variable(factor, variable_map)?;
                Ok(AstExpression::PostfixIncrement {factor: Box::new(resolved_factor)})
            },
            AstExpression::PostfixDecrement { factor } => {
                let resolved_factor = Self::check_variable(factor, variable_map)?;
                Ok(AstExpression::PostfixDecrement {factor: Box::new(resolved_factor)})
            },
            AstExpression::Conditional { condition, then_expression, else_expression } => {
                let resolved_condition = self.resolve_expression(condition.as_ref(), variable_map)?;
                let resolved_then = self.resolve_expression(then_expression.as_ref(), variable_map)?;
                let resolved_else = self.resolve_expression(else_expression.as_ref(), variable_map)?;
                Ok(AstExpression::Conditional {condition: Box::new(resolved_condition.clone()), then_expression: Box::new(resolved_then.clone()), else_expression: Box::new(resolved_else.clone())})
            }
        }
    }

    fn resolve_statement(&self, ast_statement: &AstStatement, variable_map: &mut HashMap<String, VariableInfo>) -> Result<AstStatement, String> {
        match ast_statement {
            AstStatement::Return { expression } => {
                let expression = self.resolve_expression(&expression, variable_map)?;
                Ok(AstStatement::Return {expression: expression})
            }
            AstStatement::Expression { expression} => {
                let expression = self.resolve_expression(&expression, variable_map)?;
                Ok(AstStatement::Expression {expression: expression})
            }
            AstStatement::Null => Ok(AstStatement::Null),
            AstStatement::If { expression, then_statement, else_statement } => {
                let result_expression = self.resolve_expression(&expression, variable_map)?;
                let result_then = self.resolve_statement(&then_statement, variable_map)?;

                let result_else = match else_statement {
                    Some(else_statement) => {
                        let else_result = self.resolve_statement(&else_statement.as_ref(), variable_map);
                        match else_result {
                            Ok(else_exp) => Some (Box::new(else_exp)),
                            Err(msg) => return Err(msg.to_string())
                        }
                    },
                    None => None
                };

                Ok(AstStatement::If {expression: result_expression, then_statement: Box::new(result_then), else_statement: result_else})
            }
            AstStatement::Label { label, statement } => {
                let resolved_statement = self.resolve_statement(statement.as_ref(), variable_map)?;
                Ok(AstStatement::Label { label: label.clone(), statement: Box::new(resolved_statement) })
            },
            AstStatement::Goto { target } => Ok(AstStatement::Goto { target: target.clone() }),
            AstStatement::Compound { block } => {
                let mut new_variable_map = self.copy_variable_map(variable_map);
                let resolved_block = self.resolve_block(block, &mut new_variable_map)?;
                Ok( AstStatement::Compound {block: resolved_block})
            }
            AstStatement::Break { label } => Ok(AstStatement::Break {label : label.clone()}),
            AstStatement::Continue { label } => Ok(AstStatement::Continue {label : label.clone()}),
            AstStatement::While { condition, body, label } => {
                let result_condition = self.resolve_expression(&condition, variable_map)?;
                let result_body = self.resolve_statement(&body, variable_map)?;
                Ok(AstStatement::While {condition: result_condition, body: Box::new(result_body), label: label.clone()})
            },
            AstStatement::DoWhile { condition, body, label } => {
                let result_condition = self.resolve_expression(&condition, variable_map)?;
                let result_body = self.resolve_statement(&body, variable_map)?;

                Ok(AstStatement::DoWhile {condition: result_condition, body: Box::new(result_body), label: label.clone()})
            },
            AstStatement::For { init, condition, post, body, label } => {
                let mut new_variable_map = self.copy_variable_map(variable_map);

                let resolved_init = match init {
                    Some(AstForInit::InitDecl {declaration}) => {
                        let resolved_declaration = self.resolve_declaration(declaration, &mut new_variable_map)?;
                        Some(AstForInit::InitDecl {declaration: Box::new(resolved_declaration)})
                    },
                    Some(AstForInit::InitExpr {initializer}) => {
                        let resolved_initializer= self.resolve_expression(&initializer, &mut new_variable_map)?;
                        Some(AstForInit::InitExpr {initializer: Box::new(resolved_initializer)})
                    },
                    None => None
                };

                let resolved_condition = match condition {
                    Some(expression) => {
                        let resolved_condition = self.resolve_expression(expression.as_ref(), &mut new_variable_map)?;
                        Some(Box::new(resolved_condition))
                    }
                    None => None
                };

                let resolved_post = match post {
                    Some(expression) => {
                        let resolved_post = self.resolve_expression(expression.as_ref(), &mut new_variable_map)?;
                        Some(Box::new(resolved_post))
                    }
                    None => None
                };

                let resolved_body = self.resolve_statement(&body, &mut new_variable_map)?;
                Ok(AstStatement::For {init: resolved_init, condition: resolved_condition, post: resolved_post, body: Box::new(resolved_body), label: label.clone() })
            },
        }
    }

    fn check_variable(ast_expression: &Box<AstExpression>, variable_map: &HashMap<String, VariableInfo>) -> Result<AstExpression, String> {
        match ast_expression.as_ref() {
            AstExpression::Var { identifier } => match variable_map.contains_key(identifier) {
                true => Ok(AstExpression::Var {identifier: variable_map.get(identifier).unwrap().new_name.clone()}),
                false => Err(format!("Undeclared variable : {identifier}"))
            }
            _ => Err(format!("Invalid variable expression"))
        }
    }

    fn copy_variable_map(&self, variable_map: &mut HashMap<String, VariableInfo>) -> HashMap<String, VariableInfo> {
        let mut new_variable_map = variable_map.clone();
        for (_, variable_info) in new_variable_map.iter_mut() {
            variable_info.from_current_block = false;
        }
        new_variable_map
    }

    fn resolve_block(&self, ast_block: &AstBlock, variable_map: &mut HashMap<String, VariableInfo>) -> Result<AstBlock, String> {
        let mut block_items: Vec<AstBlockItem>=  Vec::new();
        for block_item in ast_block.block_items.iter() {
            let ast_block_item : AstBlockItem = match block_item {
                AstBlockItem::Statement(statement) => {
                    let resolved_statement = self.resolve_statement(&statement, variable_map)?;
                    AstBlockItem::Statement(resolved_statement)
                }
                AstBlockItem::Declaration(declaration) => {
                    let resolved_declaration = self.resolve_declaration(declaration, variable_map)?;
                    AstBlockItem::Declaration(resolved_declaration)
                }
            };
            block_items.push(ast_block_item);
        }

        Ok(AstBlock {block_items})
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

    #[test]
    fn test_resolve_declaration_two_variables() {
        let result = resolve("int x = 42; int y = 12; return x;");
        assert!(result.is_ok());
        let expected = vec![
            "Declaration(AstDeclaration { identifier: \"x-0\", init: Some(Constant { constant: AstConstant { value: 42 } }) })",
            "Declaration(AstDeclaration { identifier: \"y-1\", init: Some(Constant { constant: AstConstant { value: 12 } }) })",
            "Statement(Return { expression: Var { identifier: \"x-0\" } })"
        ];

        check(result.unwrap(), expected)
    }

    #[test_case("return x;", "Undeclared variable ! x ")]
    #[test_case("int 42 = 10;", "Invalid program: Invalid function: Invalid block: Invalid identifier: 42")]
    pub fn test_failed_resolve_declaration(code: &str, expected_error: &str) {
        let result = resolve(code);
        assert!(result.is_err());
        assert_eq!(format!("{}", result.unwrap_err()), expected_error);
    }

    pub fn resolve(code: &str) -> Result<Vec<String>, String> {
        let program: String = format!("int main(void) {{{code}}}");
        let lexer = Lexer::new(program);
        let mut tokens = lexer.tokenize().unwrap();
        let parser = Parser::new();
        let program_result = parser.parse_program(&mut tokens);
        if let Err(error) = program_result {
            return Err(error);
        }

        let resolver = Resolver::new();
        let resolved_function = resolver.resolve_function(&program_result.unwrap().function);

        match resolved_function {
            Ok(function) => {
                let txt: Vec<String> = function.body.block_items.iter().map(|block_item| format!("{block_item:?}")).collect();
                Ok(txt)
            }
            Err(msg) => Err(msg)
        }
    }

    pub fn check(lines1: Vec<String>, lines2: Vec<&str>) {
        if lines1.len() != lines2.len() {
            panic!("{lines1:?}\n{lines2:?}")
        }

        for (i, line) in lines1.iter().enumerate() {
            let other_line = lines2.get(i).unwrap();
            assert_eq!(line, other_line);
        }
    }
}
 