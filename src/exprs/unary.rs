use std::marker::PhantomData;

use crate::{lexer::TokenInfo, opers::UnaryOperator};

use super::{binary, EvalArgs, Expression, ExpressionResult};

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

    pub fn eval(&self, args: &mut EvalArgs) -> ExpressionResult {
        let value = super::eval(&self.expr, args)?;
        match T::eval(value) {
            Ok(value) => Ok(value),
            Err(error) => binary::map_error(self.info.clone(), error),
        }
    }
}
