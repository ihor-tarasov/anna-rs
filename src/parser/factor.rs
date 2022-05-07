use crate::{
    exprs::{BinaryExpression, Expression},
    lexer::{Lexer, TokenType},
};

use super::{bitwise, ParserResult, Parser};

pub fn parse(lexer: &mut Lexer, parser: &mut Parser) -> ParserResult {
    let mut lhs = bitwise::parse(lexer, parser)?;
    while let Some(token) = lexer.peek() {
        match token.ttype() {
            TokenType::Asterisk => {
                let info = token.info();
                lexer.next();
                let rhs = bitwise::parse(lexer, parser)?;
                lhs = Expression::Multiply(Box::new(BinaryExpression::new(lhs, rhs, info)));
            }
            TokenType::Slash => {
                let info = token.info();
                lexer.next();
                let rhs = bitwise::parse(lexer, parser)?;
                lhs = Expression::Divide(Box::new(BinaryExpression::new(lhs, rhs, info)));
            }
            _ => break,
        }
    }
    Ok(lhs)
}
