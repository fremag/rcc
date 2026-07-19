use crate::ast_model::ast_return::AstReturn;

#[derive(Debug)]
pub struct AstStatement {
    pub(crate) return_exp: AstReturn,
}
