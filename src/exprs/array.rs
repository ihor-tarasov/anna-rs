use crate::{types::Object, State};

use super::{eval, Expression, ExpressionResult};

pub struct ArrayExpression {
    exprs: Vec<Expression>,
}

impl ArrayExpression {
    pub fn new(exprs: Vec<Expression>) -> Expression {
        Expression::Array(Self { exprs })
    }

    pub fn eval(&self, state: &mut State) -> ExpressionResult {
        let mut array = Vec::new();
        array.reserve(self.exprs.len());
        for expr in &self.exprs {
            array.push(eval(expr, state)?);
        }
        Ok(state
            .global()
            .borrow_mut()
            .storage_mut()
            .push(Object::Array(array)))
    }
}
