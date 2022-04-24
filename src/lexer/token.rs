use super::{TokenType, TokenInfo};

pub struct Token {
    ttype: TokenType,
    info: TokenInfo,
}

impl Token {
    pub fn new(ttype: TokenType, info: TokenInfo) -> Self {
        Self { ttype, info }
    }

    pub fn ttype(&self) -> &TokenType {
        &self.ttype
    }

    pub fn info(&self) -> TokenInfo {
        self.info.clone()
    }

    pub fn take_type(self) -> TokenType {
        self.ttype
    }
}
