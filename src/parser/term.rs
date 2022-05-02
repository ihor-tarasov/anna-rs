use crate::{
    exprs::{BinaryExpression, Expression},
    lexer::{Lexer, TokenType}, State,
};

use super::{factor, ParserResult};

pub fn parse(lexer: &mut Lexer, state: &mut State) -> ParserResult {
    let mut lhs = factor::parse(lexer, state)?;
    while let Some(token) = lexer.peek() {
        match token.ttype() {
            TokenType::Plus => {
                let info = token.info();
                lexer.next();
                let rhs = factor::parse(lexer, state)?;
                lhs = Expression::Addict(Box::new(BinaryExpression::new(lhs, rhs, info)));
            },
            TokenType::Minus => {
                let info = token.info();
                lexer.next();
                let rhs = factor::parse(lexer, state)?;
                lhs = Expression::Subtract(Box::new(BinaryExpression::new(lhs, rhs, info)));
            },
            _ => break,
        }
    }
    Ok(lhs)
}
