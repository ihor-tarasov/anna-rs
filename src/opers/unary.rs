use crate::types::Value;

use super::{OperatorError, OperatorResult, UnaryOperator};

pub struct UnaryMinusOperator;

impl UnaryOperator for UnaryMinusOperator {
    fn eval(value: Value) -> OperatorResult {
        match value {
            Value::Integer(value) => Ok(Value::Integer(-value)),
            Value::Real(value) => Ok(Value::Real(-value)),
            _ => Err(OperatorError::Unsupported),
        }
    }
}

pub struct UnaryNotOperator;

impl UnaryOperator for UnaryNotOperator {
    fn eval(value: Value) -> OperatorResult {
        match value {
            Value::Boolean(value) => Ok(Value::Boolean(!value)),
            Value::Integer(value) => Ok(Value::Integer(!value)),
            _ => Err(OperatorError::Unsupported),
        }
    }
}
