use crate::{
    exprs::{BinaryExpression, Expression},
    lexer::{Lexer, TokenType},
};

use super::{factor, ParserResult};

pub fn parse(lexer: &mut Lexer) -> ParserResult {
    let mut lhs = factor::parse(lexer)?;
    while let Some(token) = lexer.peek() {
        match token.ttype() {
            TokenType::Plus => {
                let info = token.info();
                lexer.next();
                let rhs = factor::parse(lexer)?;
                lhs = Expression::Addict(Box::new(BinaryExpression::new(lhs, rhs, info)));
            },
            TokenType::Minus => {
                let info = token.info();
                lexer.next();
                let rhs = factor::parse(lexer)?;
                lhs = Expression::Subtract(Box::new(BinaryExpression::new(lhs, rhs, info)));
            },
            _ => break,
        }
    }
    Ok(lhs)
}
