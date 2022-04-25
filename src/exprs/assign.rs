use crate::{lexer::TokenInfo, State};

use super::{eval, result::variable_not_exist, Expression, ExpressionResult};

pub struct AssignExpression {
    expr: Expression,
    name: String,
    info: TokenInfo,
}

impl AssignExpression {
    pub fn new(expr: Expression, name: String, info: TokenInfo) -> Expression {
        Expression::Assign(Box::new(Self { expr, name, info }))
    }

    pub fn eval(&self, state: &mut State) -> ExpressionResult {
        let value = eval(&self.expr, state)?;
        if let Some(frame) = state.stack_mut().frame_mut() {
            if let Some(dst) = frame.get_mut(&self.name) {
                *dst = value.clone();
            }
        }
        match state.global().borrow_mut().frame_mut().get_mut(&self.name) {
            Some(dst) => *dst = value.clone(),
            None => return variable_not_exist(self.info.clone()),
        }
        Ok(value)
    }
}
