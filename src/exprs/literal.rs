use crate::types::{Value, Object};

use super::{Expression, ExpressionResult, EvalArgs};

pub struct LiteralExpression(Value);

impl LiteralExpression {
    pub fn new(value: Value) -> Expression {
        Expression::Literal(Self { 0: value })
    }

    pub fn eval(&self) -> ExpressionResult {
        Ok(self.0.clone())
    }
}

pub struct StringLiteralExpression(String);

impl StringLiteralExpression {
    pub fn new(value: String) -> Expression {
        Expression::StringLiteral(Self { 0: value })
    }

    pub fn eval(&self, args: &mut EvalArgs) -> ExpressionResult {
        Ok(args.storage.lock().unwrap().push(Object::String(self.0.clone())))
    }
}
