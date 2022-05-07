
use crate::{
    exprs::{BinaryExpression, Expression},
    lexer::{Lexer, TokenType}, Functions,
};

use super::{ParserResult, term};

pub fn parse(lexer: &mut Lexer, functions: &mut Functions) -> ParserResult {
    let mut lhs = term::parse(lexer, functions)?;
    while let Some(token) = lexer.peek() {
        match token.ttype() {
            TokenType::Less => {
                let info = token.info();
                lexer.next();
                let rhs = term::parse(lexer, functions)?;
                lhs = Expression::Less(Box::new(BinaryExpression::new(lhs, rhs, info)));
            },
            TokenType::Greater => {
                let info = token.info();
                lexer.next();
                let rhs = term::parse(lexer, functions)?;
                lhs = Expression::Greater(Box::new(BinaryExpression::new(lhs, rhs, info)));
            },
            TokenType::LessEqual => {
                let info = token.info();
                lexer.next();
                let rhs = term::parse(lexer, functions)?;
                lhs = Expression::LessEqual(Box::new(BinaryExpression::new(lhs, rhs, info)));
            },
            TokenType::GreaterEqual => {
                let info = token.info();
                lexer.next();
                let rhs = term::parse(lexer, functions)?;
                lhs = Expression::GreaterEqual(Box::new(BinaryExpression::new(lhs, rhs, info)));
            },
            _ => break,
        }
    }
    Ok(lhs)
}
