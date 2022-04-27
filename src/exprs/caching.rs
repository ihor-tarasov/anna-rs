use crate::{types::Value, State};

use super::{eval, Expression, ExpressionResult};

pub struct CachingExpression {
    expr: Option<Expression>,
    value: Value,
}

impl CachingExpression {
    pub fn new(expr: Option<Expression>, value: Value) -> Expression {
        Expression::Caching(Box::new(Self { expr, value }))
    }

    pub fn eval(&self, state: &mut State) -> ExpressionResult {
        if let Some(expr) = &self.expr {
            *state.cache_mut() = eval(expr, state)?;
        } else {
            *state.cache_mut() = Value::Void;
        }

        Ok(self.value.clone())
    }
}
