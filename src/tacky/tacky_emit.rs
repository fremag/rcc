use crate::ast_model::ast_return::AstReturn;
use crate::ast_model::expression::AstExpression;
use crate::ast_model::function::AstFunction;
use crate::ast_model::program::AstProgram;
use crate::ast_model::unary::AstUnaryOp;
use crate::tacky::{TackyFunction, TackyInstruction, TackyProgram, TackyUnaryOp, TackyVal};

pub struct TackyEmit {
    tmp_var_count : i32
}

impl TackyEmit {
    pub fn emit_expression(&mut self, expression: &AstExpression, instructions: &mut Vec<TackyInstruction>) -> TackyVal {
        match expression {
            AstExpression::Constant(cst) => {
                TackyVal::Constant(cst.value)
            }
            AstExpression::Unary(op, inner) => {
                let exp = inner.as_ref().clone();
                let src = self.emit_expression(&exp, instructions);
                let dst_name = self.make_temporary();
                let dst = TackyVal::Var(dst_name);
                let tacky_op = TackyEmit::convert_unop(op);
                let tacky_inst = TackyInstruction::Unary(tacky_op, src, dst.clone());
                instructions.push(tacky_inst);
                dst
            }
        }
    }

    fn convert_unop(ast_unary_op: &AstUnaryOp) -> TackyUnaryOp {
        match ast_unary_op {
            AstUnaryOp::Negate => TackyUnaryOp::Negate,
            AstUnaryOp::BitwiseComplement => TackyUnaryOp::Complement
        }
    }

    fn make_temporary(&mut self) -> String {
        self.tmp_var_count+=1;
        String::from("tmp.") + &self.tmp_var_count.to_string()
    }

    pub fn emit_return(&mut self, ast_return: &AstReturn, instructions: &mut Vec<TackyInstruction>) {
        let exp = self.emit_expression(&ast_return.expression, instructions);
        instructions.push(TackyInstruction::Return(exp));
    }

    pub fn emit_program(&mut self, program: &AstProgram) -> TackyProgram {
        TackyProgram{
            function_def: self.emit_function(&program.function)
        }
    }

    pub fn emit_function(&mut self, function: &AstFunction) -> TackyFunction {
        let mut instructions : Vec<TackyInstruction> = Vec::new();
        let _ = self.emit_return(&function.body.return_exp, &mut instructions);
        TackyFunction {
            identifier: function.identifier.clone(),
            body: instructions
        }
    }
}

