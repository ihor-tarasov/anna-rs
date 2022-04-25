use crate::{
    exprs::{BinaryExpression, Expression},
    lexer::{Lexer, TokenType},
};

use super::{primary, ParserResult};

pub fn parse(lexer: &mut Lexer) -> ParserResult {
    let mut lhs = primary::parse(lexer)?;
    while let Some(token) = lexer.peek() {
        match token.ttype() {
            TokenType::Asterisk => {
                let info = token.info();
                lexer.next();
                let rhs = primary::parse(lexer)?;
                lhs = Expression::Multiply(Box::new(BinaryExpression::new(lhs, rhs, info)));
            },
            TokenType::Slash => {
                let info = token.info();
                lexer.next();
                let rhs = primary::parse(lexer)?;
                lhs = Expression::Divide(Box::new(BinaryExpression::new(lhs, rhs, info)));
            },
            _ => break,
        }
    }
    Ok(lhs)
}
