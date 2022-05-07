use std::marker::PhantomData;

use crate::{
    lexer::TokenInfo,
    opers::{BinaryOperator, OperatorError},
};

use super::{
    expression::Expression, EvalArgs, ExpressionError, ExpressionErrorType, ExpressionResult,
};

pub struct BinaryExpression<T: BinaryOperator> {
    lhs: Expression,
    rhs: Expression,
    info: TokenInfo,
    phantom: PhantomData<T>,
}

pub fn map_error(info: TokenInfo, error: OperatorError) -> ExpressionResult {
    let etype = match error {
        OperatorError::Unsupported => ExpressionErrorType::UnsupportedOperator,
        OperatorError::DividingByZero => ExpressionErrorType::DividingByZero,
        OperatorError::ShiftNegative => ExpressionErrorType::ShiftNegative,
    };
    Err(ExpressionError::new(etype, info))
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

    pub fn eval(&self, args: &mut EvalArgs) -> ExpressionResult {
        let lhs = super::eval(&self.lhs, args)?;
        let rhs = super::eval(&self.rhs, args)?;
        match T::eval(lhs, rhs) {
            Ok(value) => Ok(value),
            Err(error) => map_error(self.info.clone(), error),
        }
    }
}
