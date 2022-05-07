use crate::{
    exprs::{BinaryExpression, Expression},
    lexer::{Lexer, TokenType},
};

use super::{ParserResult, comparison, Parser};

pub fn parse(lexer: &mut Lexer, parser: &mut Parser) -> ParserResult {
    let mut lhs = comparison::parse(lexer, parser)?;
    while let Some(token) = lexer.peek() {
        match token.ttype() {
            TokenType::EqualEqual => {
                let info = token.info();
                lexer.next();
                let rhs = comparison::parse(lexer, parser)?;
                lhs = Expression::Equal(Box::new(BinaryExpression::new(lhs, rhs, info)));
            },
            TokenType::ExclamationEqual => {
                let info = token.info();
                lexer.next();
                let rhs = comparison::parse(lexer, parser)?;
                lhs = Expression::NotEqual(Box::new(BinaryExpression::new(lhs, rhs, info)));
            },
            _ => break,
        }
    }
    Ok(lhs)
}
