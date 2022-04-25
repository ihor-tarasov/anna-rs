use super::{BinaryOperator, OperatorError, OperatorResult};
use crate::types::Value;
use std::marker::PhantomData;

pub trait ComparisonType {
    fn eval_int(lhs: i64, rhs: i64) -> bool;
    fn eval_real(lhs: f64, rhs: f64) -> bool;
}

pub struct EqualComparison;

impl ComparisonType for EqualComparison {
    fn eval_int(lhs: i64, rhs: i64) -> bool {
        lhs == rhs
    }
    fn eval_real(lhs: f64, rhs: f64) -> bool {
        lhs == rhs
    }
}

pub struct NotEqualComparison;

impl ComparisonType for NotEqualComparison {
    fn eval_int(lhs: i64, rhs: i64) -> bool {
        lhs != rhs
    }

    fn eval_real(lhs: f64, rhs: f64) -> bool {
        lhs != rhs
    }
}

pub struct LessComparison;

impl ComparisonType for LessComparison {
    fn eval_int(lhs: i64, rhs: i64) -> bool {
        lhs < rhs
    }

    fn eval_real(lhs: f64, rhs: f64) -> bool {
        lhs < rhs
    }
}

pub struct GreaterComparison;

impl ComparisonType for GreaterComparison {
    fn eval_int(lhs: i64, rhs: i64) -> bool {
        lhs > rhs
    }

    fn eval_real(lhs: f64, rhs: f64) -> bool {
        lhs > rhs
    }
}

pub struct LessEqualComparison;

impl ComparisonType for LessEqualComparison {
    fn eval_int(lhs: i64, rhs: i64) -> bool {
        lhs <= rhs
    }

    fn eval_real(lhs: f64, rhs: f64) -> bool {
        lhs <= rhs
    }
}

pub struct GreaterEqualComparison;

impl ComparisonType for GreaterEqualComparison {
    fn eval_int(lhs: i64, rhs: i64) -> bool {
        lhs >= rhs
    }

    fn eval_real(lhs: f64, rhs: f64) -> bool {
        lhs >= rhs
    }
}

pub struct ComparisonOperator<T: ComparisonType> {
    phantom: PhantomData<T>,
}

impl<T: ComparisonType> BinaryOperator for ComparisonOperator<T> {
    fn eval(lhs: Value, rhs: Value) -> OperatorResult {
        match (lhs, rhs) {
            (Value::Integer(lhs), Value::Integer(rhs)) => Ok(Value::Boolean(T::eval_int(lhs, rhs))),
            (Value::Integer(lhs), Value::Real(rhs)) => {
                Ok(Value::Boolean(T::eval_real(lhs as f64, rhs)))
            }
            (Value::Real(lhs), Value::Integer(rhs)) => {
                Ok(Value::Boolean(T::eval_real(lhs, rhs as f64)))
            }
            (Value::Real(lhs), Value::Real(rhs)) => Ok(Value::Boolean(T::eval_real(lhs, rhs))),
            _ => Err(OperatorError::Unsupported),
        }
    }
}
