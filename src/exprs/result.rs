use crate::{lexer::TokenInfo, types::Value};

#[derive(Debug, Clone)]
pub enum ExpressionErrorType {
    UnsupportedOperator,
    DividingByZero,
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
