
use crate::{
    exprs::{BinaryExpression, Expression},
    lexer::{Lexer, TokenType}, State,
};

use super::{ParserResult, term};

pub fn parse(lexer: &mut Lexer, state: &mut State) -> ParserResult {
    let mut lhs = term::parse(lexer, state)?;
    while let Some(token) = lexer.peek() {
        match token.ttype() {
            TokenType::Less => {
                let info = token.info();
                lexer.next();
                let rhs = term::parse(lexer, state)?;
                lhs = Expression::Less(Box::new(BinaryExpression::new(lhs, rhs, info)));
            },
            TokenType::Greater => {
                let info = token.info();
                lexer.next();
                let rhs = term::parse(lexer, state)?;
                lhs = Expression::Greater(Box::new(BinaryExpression::new(lhs, rhs, info)));
            },
            TokenType::LessEqual => {
                let info = token.info();
                lexer.next();
                let rhs = term::parse(lexer, state)?;
                lhs = Expression::LessEqual(Box::new(BinaryExpression::new(lhs, rhs, info)));
            },
            TokenType::GreaterEqual => {
                let info = token.info();
                lexer.next();
                let rhs = term::parse(lexer, state)?;
                lhs = Expression::GreaterEqual(Box::new(BinaryExpression::new(lhs, rhs, info)));
            },
            _ => break,
        }
    }
    Ok(lhs)
}
