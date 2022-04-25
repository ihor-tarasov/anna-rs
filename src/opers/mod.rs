use crate::types::Value;

#[derive(Debug)]
pub enum OperatorError {
    Unsupported,
    DividingByZero,
    ShiftNegative,
}

pub type OperatorResult = Result<Value, OperatorError>;

pub trait BinaryOperator {
    fn eval(lhs: Value, rhs: Value) -> OperatorResult;
}

pub trait UnaryOperator {
    fn eval(value: Value) -> OperatorResult;
}

pub mod arithmetic;
pub mod bitwise;
pub mod comparison;
pub mod unary;
