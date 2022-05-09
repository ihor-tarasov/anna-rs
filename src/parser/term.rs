use crate::{
    exprs::{BinaryExpression, Expression},
    lexer::{Lexer, TokenType},
};

use super::{factor, ParserResult, Parser};

pub fn parse(lexer: &mut Lexer, parser: &mut Parser, require: bool) -> ParserResult {
    let mut lhs = factor::parse(lexer, parser, require)?;
    while let Some(token) = lexer.peek(require) {
        match token.ttype() {
            TokenType::Plus => {
                let info = token.info();
                lexer.next(true);
                lexer.peek(true);
                let rhs = factor::parse(lexer, parser, require)?;
                lhs = Expression::Addict(Box::new(BinaryExpression::new(lhs, rhs, info)));
            },
            TokenType::Minus => {
                let info = token.info();
                lexer.next(true);
                lexer.peek(true);
                let rhs = factor::parse(lexer, parser, require)?;
                lhs = Expression::Subtract(Box::new(BinaryExpression::new(lhs, rhs, info)));
            },
            _ => break,
        }
    }
    Ok(lhs)
}
