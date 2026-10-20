use crate::ast_model::expression::AstExpression;
use crate::ast_model::function::{AstBlock, AstDeclaration};

#[derive(Debug, Clone)]
pub enum AstStatement {
    Return{expression: AstExpression},
    Expression{ expression: AstExpression},
    If{expression: AstExpression, then_statement : Box<AstStatement>, else_statement: Option<Box<AstStatement>>},
    Compound{block: AstBlock},
    Break { label: String },
    Continue { label: String },
    While{ condition: AstExpression, body : Box<AstStatement>, label: String },
    DoWhile{ condition: AstExpression, body : Box<AstStatement>, label:  String },
    For{init: Option<AstForInit>, condition : Option<Box<AstExpression>>, post: Option<Box<AstExpression>>, body: Box<AstStatement>, label: String },
    Null,
    Label { label: String, statement: Box<AstStatement> },
    Goto { target: String },
}

#[derive(Debug, Clone)]
pub enum AstForInit {
    InitDecl { declaration : Box<AstDeclaration>}, InitExpr { initializer : Box<AstExpression> },
}