use std::marker::PhantomData;

use crate::{lexer::TokenInfo, opers::UnaryOperator, State};

use super::{binary::map_error, eval, Expression, ExpressionResult};

pub struct UnaryExpression<T: UnaryOperator> {
    info: TokenInfo,
    expr: Expression,
    phantom: PhantomData<T>,
}

impl<T: UnaryOperator> UnaryExpression<T> {
    pub fn new(expr: Expression, info: TokenInfo) -> Self {
        Self {
            info,
            expr,
            phantom: PhantomData,
        }
    }

    pub fn eval(&self, state: &mut State) -> ExpressionResult {
        let value = eval(&self.expr, state)?;
        match T::eval(value) {
            Ok(value) => Ok(value),
            Err(error) => map_error(self.info.clone(), error),
        }
    }
}
