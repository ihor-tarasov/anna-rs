use std::marker::PhantomData;

use crate::{
    lexer::TokenInfo,
    opers::{BinaryOperator, OperatorError}, State,
};

use super::{eval, expression::Expression, ExpressionError, ExpressionErrorType, ExpressionResult};

pub struct BinaryExpression<T: BinaryOperator> {
    lhs: Expression,
    rhs: Expression,
    info: TokenInfo,
    phantom: PhantomData<T>,
}

impl<T: BinaryOperator> BinaryExpression<T> {
    pub fn new(lhs: Expression, rhs: Expression, info: TokenInfo) -> Self {
        Self {
            lhs,
            rhs,
            info,
            phantom: PhantomData,
        }
    }

    fn map_error(&self, error: OperatorError) -> ExpressionResult {
        let etype = match error {
            OperatorError::Unsupported => ExpressionErrorType::UnsupportedOperator,
            OperatorError::DividingByZero => ExpressionErrorType::DividingByZero,
        };
        Err(ExpressionError::new(etype, self.info.clone()))
    }

    pub fn eval(&self, state: &mut State) -> ExpressionResult {
        let lhs = eval(&self.lhs, state)?;
        let rhs = eval(&self.rhs, state)?;
        match T::eval(lhs, rhs) {
            Ok(value) => Ok(value),
            Err(error) => self.map_error(error),
        }
    }
}
