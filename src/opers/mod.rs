use crate::types::Value;

#[derive(Debug)]
pub enum OperatorError {
    Unsupported,
    DividingByZero,
}

pub type OperatorResult = Result<Value, OperatorError>;

pub trait BinaryOperator {
    fn eval(lhs: Value, rhs: Value) -> OperatorResult;
}

pub mod arithmetic;
pub mod comparison;
