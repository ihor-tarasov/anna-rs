use crate::types::Value;

use super::{EvalArgs, Expression, ExpressionResult};

pub struct CachingExpression {
    expr: Option<Expression>,
    value: Value,
}

impl CachingExpression {
    pub fn new(expr: Option<Expression>, value: Value) -> Expression {
        Expression::Caching(Box::new(Self { expr, value }))
    }

    pub fn eval(&self, args: &mut EvalArgs) -> ExpressionResult {
        if let Some(expr) = &self.expr {
            *args.state.cache_mut() = super::eval(expr, args)?;
        } else {
            *args.state.cache_mut() = Value::Void;
        }

        Ok(self.value.clone())
    }
}
