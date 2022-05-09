use crate::{
    exprs::{BinaryExpression, Expression},
    lexer::{Lexer, TokenType},
};

use super::{bitwise, ParserResult, Parser};

pub fn parse(lexer: &mut Lexer, parser: &mut Parser, require: bool) -> ParserResult {
    let mut lhs = bitwise::parse(lexer, parser, require)?;
    while let Some(token) = lexer.peek(require) {
        match token.ttype() {
            TokenType::Asterisk => {
                let info = token.info();
                lexer.next(true);
                lexer.peek(true);
                let rhs = bitwise::parse(lexer, parser, require)?;
                lhs = Expression::Multiply(Box::new(BinaryExpression::new(lhs, rhs, info)));
            }
            TokenType::Slash => {
                let info = token.info();
                lexer.next(true);
                lexer.peek(true);
                let rhs = bitwise::parse(lexer, parser, require)?;
                lhs = Expression::Divide(Box::new(BinaryExpression::new(lhs, rhs, info)));
            }
            _ => break,
        }
    }
    Ok(lhs)
}
