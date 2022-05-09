use crate::{
    exprs::{BinaryExpression, Expression},
    lexer::{Lexer, TokenType},
};

use super::{ParserResult, comparison, Parser};

pub fn parse(lexer: &mut Lexer, parser: &mut Parser, require: bool) -> ParserResult {
    let mut lhs = comparison::parse(lexer, parser, require)?;
    while let Some(token) = lexer.peek(require) {
        match token.ttype() {
            TokenType::EqualEqual => {
                let info = token.info();
                lexer.next(true);
                lexer.peek(true);
                let rhs = comparison::parse(lexer, parser, require)?;
                lhs = Expression::Equal(Box::new(BinaryExpression::new(lhs, rhs, info)));
            },
            TokenType::ExclamationEqual => {
                let info = token.info();
                lexer.next(true);
                lexer.peek(true);
                let rhs = comparison::parse(lexer, parser, require)?;
                lhs = Expression::NotEqual(Box::new(BinaryExpression::new(lhs, rhs, info)));
            },
            _ => break,
        }
    }
    Ok(lhs)
}
