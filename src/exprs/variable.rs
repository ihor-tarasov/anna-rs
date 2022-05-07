use crate::lexer::TokenInfo;

use super::{result, EvalArgs, Expression, ExpressionResult};

pub struct VariableExpression {
    name: String,
    info: TokenInfo,
}

impl VariableExpression {
    pub fn new(name: String, info: TokenInfo) -> Expression {
        Expression::Variable(Self { name, info })
    }

    pub fn eval(&self, args: &EvalArgs) -> ExpressionResult {
        if let Some(value) = args.state.stack().frame().get(&self.name) {
            return Ok(value.clone());
        } else {
            result::variable_not_exist(self.info.clone())
        }
    }
}
