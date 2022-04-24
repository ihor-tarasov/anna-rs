use std::marker::PhantomData;

use crate::types::Value;

use super::{BinaryOperator, OperatorError, OperatorResult};

pub trait ArithmeticType {
    fn eval_real(lhs: f64, rhs: f64) -> OperatorResult;
    fn eval_int(lhs: i64, rhs: i64) -> OperatorResult;
}

pub struct AddictArithmetic;

impl ArithmeticType for AddictArithmetic {
    fn eval_real(lhs: f64, rhs: f64) -> OperatorResult {
        Ok(Value::Real(lhs + rhs))
    }

    fn eval_int(lhs: i64, rhs: i64) -> OperatorResult {
        Ok(Value::Integer(lhs.wrapping_add(rhs)))
    }
}

pub struct SubtractArithmetic;

impl ArithmeticType for SubtractArithmetic {
    fn eval_real(lhs: f64, rhs: f64) -> OperatorResult {
        Ok(Value::Real(lhs - rhs))
    }

    fn eval_int(lhs: i64, rhs: i64) -> OperatorResult {
        Ok(Value::Integer(lhs.wrapping_sub(rhs)))
    }
}

pub struct MultiplyArithmetic;

impl ArithmeticType for MultiplyArithmetic {
    fn eval_real(lhs: f64, rhs: f64) -> OperatorResult {
        Ok(Value::Real(lhs * rhs))
    }

    fn eval_int(lhs: i64, rhs: i64) -> OperatorResult {
        Ok(Value::Integer(lhs.wrapping_mul(rhs)))
    }
}

pub struct DivideArithmetic;

impl ArithmeticType for DivideArithmetic {
    fn eval_real(lhs: f64, rhs: f64) -> OperatorResult {
        Ok(Value::Real(lhs / rhs))
    }

    fn eval_int(lhs: i64, rhs: i64) -> OperatorResult {
        if rhs == 0 {
            Err(OperatorError::DividingByZero)
        } else {
            Ok(Value::Integer(lhs.wrapping_add(rhs)))
        }
    }
}

pub struct ArithmeticOperator<T: ArithmeticType> {
    phantom: PhantomData<T>,
}

impl<T: ArithmeticType> BinaryOperator for ArithmeticOperator<T> {
    fn eval(lhs: Value, rhs: Value) -> OperatorResult {
        match (lhs, rhs) {
            (Value::Integer(lhs), Value::Integer(rhs)) => T::eval_int(lhs, rhs),
            (Value::Integer(lhs), Value::Real(rhs)) => T::eval_real(lhs as f64, rhs),
            (Value::Real(lhs), Value::Integer(rhs)) => T::eval_real(lhs, rhs as f64),
            (Value::Real(lhs), Value::Real(rhs)) => T::eval_real(lhs, rhs),
            _ => Err(OperatorError::Unsupported),
        }
    }
}
