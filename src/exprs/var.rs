use crate::lexer::TokenInfo;

use super::{EvalArgs, Expression, ExpressionError, ExpressionErrorType, ExpressionResult};

pub struct VarExpression {
    expr: Expression,
    name: String,
    info: TokenInfo,
}

fn variable_already_exist(info: TokenInfo) -> ExpressionResult {
    Err(ExpressionError::new(
        ExpressionErrorType::VariableAlreadyExists,
        info,
    ))
}

impl VarExpression {
    pub fn new(expr: Expression, name: String, info: TokenInfo) -> Expression {
        Expression::Var(Box::new(Self { expr, name, info }))
    }

    pub fn eval(&self, args: &mut EvalArgs) -> ExpressionResult {
        let value = super::eval(&self.expr, args)?;
        if args
            .state
            .stack_mut()
            .frame_mut()
            .var(self.name.clone(), value.clone())
        {
            Ok(value)
        } else {
            variable_already_exist(self.info.clone())
        }
    }
}
