use crate::{lexer::TokenInfo, State};

use super::{eval, Expression, ExpressionError, ExpressionErrorType, ExpressionResult};

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

    pub fn eval(&self, state: &mut State) -> ExpressionResult {
        let value = eval(&self.expr, state)?;
        if let Some(frame) = state.stack_mut().frame_mut() {
            if frame.var(self.name.clone(), value.clone()) {
                Ok(value)
            } else {
                variable_already_exist(self.info.clone())
            }
        } else {
            if state
                .global()
                .borrow_mut()
                .frame_mut()
                .var(self.name.clone(), value.clone())
            {
                Ok(value)
            } else {
                variable_already_exist(self.info.clone())
            }
        }
    }
}
