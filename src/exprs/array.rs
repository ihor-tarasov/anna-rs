use crate::types::Object;

use super::{EvalArgs, Expression, ExpressionResult};

pub struct ArrayExpression {
    exprs: Vec<Expression>,
}

impl ArrayExpression {
    pub fn new(exprs: Vec<Expression>) -> Expression {
        Expression::Array(Self { exprs })
    }

    pub fn eval(&self, args: &mut EvalArgs) -> ExpressionResult {
        let mut array = Vec::new();
        array.reserve(self.exprs.len());
        for expr in &self.exprs {
            array.push(super::eval(expr, args)?);
        }
        Ok(args.storage.lock().unwrap().push(Object::Array(array)))
    }
}
