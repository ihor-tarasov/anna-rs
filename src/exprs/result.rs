use crate::{lexer::TokenInfo, types::Value};

#[derive(Debug, Clone)]
pub enum ExpressionErrorType {
    UnsupportedOperator,
    DividingByZero,
    ShiftNegative,
    VariableNotExist,
    VariableAlreadyExists,
    InvalidIndex,
    ExpectIndexableObject,
    IndexOutOfBounds,
    NotCallableObject,
    ExpectedBoolForConditions,
}

pub struct ExpressionError {
    etype: ExpressionErrorType,
    info: TokenInfo,
}

impl ExpressionError {
    pub fn new(etype: ExpressionErrorType, info: TokenInfo) -> Self {
        Self { etype, info }
    }

    pub fn etype(&self) -> ExpressionErrorType {
        self.etype.clone()
    }

    pub fn info(&self) -> TokenInfo {
        self.info.clone()
    }
}

pub type ExpressionResult = Result<Value, ExpressionError>;

pub fn variable_not_exist(info: TokenInfo) -> ExpressionResult {
    Err(ExpressionError::new(ExpressionErrorType::VariableNotExist, info))
}

pub fn invalid_index(info: TokenInfo) -> ExpressionResult {
    Err(ExpressionError::new(ExpressionErrorType::InvalidIndex, info))
}

pub fn expect_indexable_object(info: TokenInfo) -> ExpressionResult {
    Err(ExpressionError::new(ExpressionErrorType::ExpectIndexableObject, info))
}

pub fn index_out_of_bounds(info: TokenInfo) -> ExpressionResult {
    Err(ExpressionError::new(ExpressionErrorType::IndexOutOfBounds, info))
}
