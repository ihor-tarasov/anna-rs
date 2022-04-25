use crate::{lexer::TokenInfo, State};

use super::{result::variable_not_exist, Expression, ExpressionResult};

pub struct VariableExpression {
    name: String,
    info: TokenInfo,
}

impl VariableExpression {
    pub fn new(name: String, info: TokenInfo) -> Expression {
        Expression::Variable(Self { name, info })
    }

    pub fn eval(&self, state: &State) -> ExpressionResult {
        if let Some(frame) = state.stack().frame() {
            if let Some(value) = frame.get(&self.name) {
                return Ok(value.clone());
            }
        }
        if let Some(value) = state.global().borrow().frame().get(&self.name) {
            Ok(value.clone())
        } else {
            variable_not_exist(self.info.clone())
        }
    }
}
