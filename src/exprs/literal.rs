use crate::types::Value;

use super::{Expression, ExpressionResult};

pub struct LiteralExpression(Value);

impl LiteralExpression {
    pub fn new(value: Value) -> Expression {
        Expression::Literal(Self { 0: value })
    }

    pub fn eval(&self) -> ExpressionResult {
        Ok(self.0.clone())
    }
}
