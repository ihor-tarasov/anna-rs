
use crate::{
    exprs::{BinaryExpression, Expression},
    lexer::{Lexer, TokenType},
};

use super::{ParserResult, term};

pub fn parse(lexer: &mut Lexer) -> ParserResult {
    let mut lhs = term::parse(lexer)?;
    while let Some(token) = lexer.peek() {
        match token.ttype() {
            TokenType::Less => {
                let info = token.info();
                lexer.next();
                let rhs = term::parse(lexer)?;
                lhs = Expression::Less(Box::new(BinaryExpression::new(lhs, rhs, info)));
            },
            TokenType::Greater => {
                let info = token.info();
                lexer.next();
                let rhs = term::parse(lexer)?;
                lhs = Expression::Greater(Box::new(BinaryExpression::new(lhs, rhs, info)));
            },
            TokenType::LessEqual => {
                let info = token.info();
                lexer.next();
                let rhs = term::parse(lexer)?;
                lhs = Expression::LessEqual(Box::new(BinaryExpression::new(lhs, rhs, info)));
            },
            TokenType::GreaterEqual => {
                let info = token.info();
                lexer.next();
                let rhs = term::parse(lexer)?;
                lhs = Expression::GreaterEqual(Box::new(BinaryExpression::new(lhs, rhs, info)));
            },
            _ => break,
        }
    }
    Ok(lhs)
}
