use crate::{
    exprs::{BinaryExpression, Expression},
    lexer::{Lexer, TokenType}, Functions,
};

use super::{ParserResult, comparison};

pub fn parse(lexer: &mut Lexer, functions: &mut Functions) -> ParserResult {
    let mut lhs = comparison::parse(lexer, functions)?;
    while let Some(token) = lexer.peek() {
        match token.ttype() {
            TokenType::EqualEqual => {
                let info = token.info();
                lexer.next();
                let rhs = comparison::parse(lexer, functions)?;
                lhs = Expression::Equal(Box::new(BinaryExpression::new(lhs, rhs, info)));
            },
            TokenType::ExclamationEqual => {
                let info = token.info();
                lexer.next();
                let rhs = comparison::parse(lexer, functions)?;
                lhs = Expression::NotEqual(Box::new(BinaryExpression::new(lhs, rhs, info)));
            },
            _ => break,
        }
    }
    Ok(lhs)
}
