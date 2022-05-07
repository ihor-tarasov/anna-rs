use crate::lexer::TokenInfo;

use super::{result, EvalArgs, Expression, ExpressionResult};

pub struct AssignExpression {
    expr: Expression,
    name: String,
    info: TokenInfo,
}

impl AssignExpression {
    pub fn new(expr: Expression, name: String, info: TokenInfo) -> Expression {
        Expression::Assign(Box::new(Self { expr, name, info }))
    }

    pub fn eval(&self, args: &mut EvalArgs) -> ExpressionResult {
        let value = super::eval(&self.expr, args)?;
        if let Some(dst) = args.state.stack_mut().frame_mut().get_mut(&self.name) {
            *dst = value.clone();
            Ok(value)
        } else {
            result::variable_not_exist(self.info.clone())
        }
    }
}
