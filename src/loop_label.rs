use crate::ast_model::function::{AstBlock, AstBlockItem, AstFunction};
use crate::ast_model::program::AstProgram;
use crate::ast_model::statement::AstStatement;

pub struct LoopLabel {
    label_count: i32
}

impl LoopLabel {
    pub fn new() -> Self {
        LoopLabel { label_count: 0 }
    }

    pub fn label_loops(&mut self, program: &AstProgram) -> Result<AstProgram, String> {
        let new_block_items= self.label_block(&program.function.body, None)?;
        Ok(AstProgram { function: AstFunction {identifier: program.function.identifier.clone(), body: new_block_items}})
    }

    pub fn label_block(&mut self, block: &AstBlock, current_label : Option<String>) -> Result<AstBlock, String> {
        let mut new_block_items: Vec<AstBlockItem> = Vec::new();
        for block_item in &block.block_items {
            match block_item {
                AstBlockItem::Statement(statement) => {
                    let statement = self.label_statement(statement, current_label.clone())?;
                    new_block_items.push(AstBlockItem::Statement(statement));
                }
                AstBlockItem::Declaration(declaration) => {
                    let decl = declaration.clone();
                    new_block_items.push(AstBlockItem::Declaration(decl));
                }
            }
        }
        Ok(AstBlock{block_items: new_block_items})
    }

    fn label_statement(&mut self, ast_statement: &AstStatement, current_label : Option<String>) -> Result<AstStatement, String> {
        match ast_statement {
            AstStatement::Break {..} =>
                {
                    if ! current_label.is_some() {
                        return Err("break statement outside of loop".to_string());
                    }
                    Ok(AstStatement::Break {label: String::from(current_label.unwrap())})
                },
            AstStatement::Continue {..} =>
                {
                    if ! current_label.is_some() {
                        return Err("continue statement outside of loop".to_string());
                    }
                    Ok(AstStatement::Continue {label: String::from(current_label.unwrap())})
                },
            AstStatement::While {condition, body, ..} =>
                {
                    let new_label = self.make_label();
                    let result = self.label_statement(body.as_ref(), Some(new_label.clone()))?;
                    let new_body = Box::new(result);
                    Ok(AstStatement::While {condition: condition.clone(), body: new_body, label: new_label})
                },
            AstStatement::DoWhile {condition, body, ..} =>
                {
                    let new_label = self.make_label();
                    let result  = self.label_statement(body.as_ref(), Some(new_label.clone()))?;
                    let new_body = Box::new(result);
                    Ok(AstStatement::DoWhile {condition: condition.clone(), body: new_body, label: new_label})
                },
            AstStatement::For {init, condition, post, body, ..} =>
                {
                    let new_label = self.make_label();
                    let result = self.label_statement(body.as_ref(), Some(new_label.clone()))?;
                    let new_body = Box::new(result);
                    Ok(AstStatement::For {init: init.clone(), condition: condition.clone(), post: post.clone(), body: new_body, label: new_label})
                },
            AstStatement::If{expression, then_statement, else_statement} => {
                let labeled_then = self.label_statement(then_statement, current_label)?;
                let labeled_else = match else_statement {
                    Some(statement) => Some(Box::new(self.label_statement(statement, None)?)),
                    None => None
                };
                Ok(AstStatement::If{expression: expression.clone(), then_statement: Box::new(labeled_then), else_statement: labeled_else })
            },
            AstStatement::Compound{block} => {
                let labeled_block = self.label_block(&block, current_label)?;
                Ok(AstStatement::Compound{block: labeled_block})

            },
            AstStatement::Label { label, statement} => {
                let labeled_statement = self.label_statement(statement, current_label)?;
                Ok(AstStatement::Label{label: label.clone(), statement: Box::new(labeled_statement) })
            },
            _ => {
                Ok(ast_statement.clone())
            }
        }
    }

    fn make_label(&mut self) -> String {
        self.label_count += 1;
        format!("loop_label_{}", self.label_count)
    }
}

#[cfg(test)]
mod tests {
    use crate::ast_model::constant::AstConstant;
    use crate::ast_model::expression::AstExpression;
    use crate::ast_model::function::{AstBlock, AstBlockItem, AstFunction};
    use crate::ast_model::program::AstProgram;
    use crate::ast_model::statement::AstStatement;
    use crate::loop_label::LoopLabel;

    #[test]
    pub fn test_loop_label_while() {
        let condition = AstExpression::Constant {constant: AstConstant {value: 0}};
        let while_body = AstStatement::Break { label: "".to_string()};
        let while_statement = AstStatement::While {condition, body: Box::new(while_body), label: "".to_string()};
        let block_items= vec![  AstBlockItem::Statement(while_statement) ];
        let function_body = AstBlock{ block_items };
        let program = AstProgram {function: AstFunction{identifier: "main".to_string(), body: function_body}};

        let mut loop_label = LoopLabel::new();
        let labeled_program = loop_label.label_loops(&program).unwrap();

        let txt = format!("{labeled_program:?}");
        assert_eq!(txt, "AstProgram { function: AstFunction { identifier: \"main\", body: AstBlock { block_items: [Statement(While { condition: Constant { constant: AstConstant { value: 0 } }, body: Break { label: \"loop_label_1\" }, label: \"loop_label_1\" })] } } }");
    }
}