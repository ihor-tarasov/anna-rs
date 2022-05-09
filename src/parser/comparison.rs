
use crate::{
    exprs::{BinaryExpression, Expression},
    lexer::{Lexer, TokenType},
};

use super::{ParserResult, term, Parser};

pub fn parse(lexer: &mut Lexer, parser: &mut Parser, require: bool) -> ParserResult {
    let mut lhs = term::parse(lexer, parser, require)?;
    while let Some(token) = lexer.peek(require) {
        match token.ttype() {
            TokenType::Less => {
                let info = token.info();
                lexer.next(true);
                lexer.peek(true);
                let rhs = term::parse(lexer, parser, require)?;
                lhs = Expression::Less(Box::new(BinaryExpression::new(lhs, rhs, info)));
            },
            TokenType::Greater => {
                let info = token.info();
                lexer.next(true);
                lexer.peek(true);
                let rhs = term::parse(lexer, parser, require)?;
                lhs = Expression::Greater(Box::new(BinaryExpression::new(lhs, rhs, info)));
            },
            TokenType::LessEqual => {
                let info = token.info();
                lexer.next(true);
                lexer.peek(true);
                let rhs = term::parse(lexer, parser, require)?;
                lhs = Expression::LessEqual(Box::new(BinaryExpression::new(lhs, rhs, info)));
            },
            TokenType::GreaterEqual => {
                let info = token.info();
                lexer.next(true);
                lexer.peek(true);
                let rhs = term::parse(lexer, parser, require)?;
                lhs = Expression::GreaterEqual(Box::new(BinaryExpression::new(lhs, rhs, info)));
            },
            _ => break,
        }
    }
    Ok(lhs)
}
