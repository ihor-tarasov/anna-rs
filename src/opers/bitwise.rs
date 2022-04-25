use std::marker::PhantomData;

use crate::types::Value;

use super::{BinaryOperator, OperatorError, OperatorResult};

pub trait BitwiseType {
    fn eval(lhs: i64, rhs: i64) -> OperatorResult;
}

pub struct AndBitwise;

impl BitwiseType for AndBitwise {
    fn eval(lhs: i64, rhs: i64) -> OperatorResult {
        Ok(Value::Integer(lhs & rhs))
    }
}

pub struct OrBitwise;

impl BitwiseType for OrBitwise {
    fn eval(lhs: i64, rhs: i64) -> OperatorResult {
        Ok(Value::Integer(lhs | rhs))
    }
}

pub struct XorBitwise;

impl BitwiseType for XorBitwise {
    fn eval(lhs: i64, rhs: i64) -> OperatorResult {
        Ok(Value::Integer(lhs ^ rhs))
    }
}

pub struct ShlBitwise;

impl BitwiseType for ShlBitwise {
    fn eval(lhs: i64, rhs: i64) -> OperatorResult {
        if rhs < 0 {
            Err(OperatorError::ShiftNegative)
        } else {
            Ok(Value::Integer(lhs.wrapping_shl(rhs as u32)))
        }
    }
}

pub struct ShrBitwise;

impl BitwiseType for ShrBitwise {
    fn eval(lhs: i64, rhs: i64) -> OperatorResult {
        if rhs < 0 {
            Err(OperatorError::ShiftNegative)
        } else {
            Ok(Value::Integer(lhs.wrapping_shr(rhs as u32)))
        }
    }
}

pub struct BitwiseOperator<T: BitwiseType> {
    phantom: PhantomData<T>,
}

impl<T: BitwiseType> BinaryOperator for BitwiseOperator<T> {
    fn eval(lhs: Value, rhs: Value) -> OperatorResult {
        match (lhs, rhs) {
            (Value::Integer(lhs), Value::Integer(rhs)) => T::eval(lhs, rhs),
            _ => Err(OperatorError::Unsupported),
        }
    }
}
