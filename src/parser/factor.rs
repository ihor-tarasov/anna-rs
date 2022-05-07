use crate::{
    exprs::{BinaryExpression, Expression},
    lexer::{Lexer, TokenType},
    Functions,
};

use super::{bitwise, ParserResult};

pub fn parse(lexer: &mut Lexer, functions: &mut Functions) -> ParserResult {
    let mut lhs = bitwise::parse(lexer, functions)?;
    while let Some(token) = lexer.peek() {
        match token.ttype() {
            TokenType::Asterisk => {
                let info = token.info();
                lexer.next();
                let rhs = bitwise::parse(lexer, functions)?;
                lhs = Expression::Multiply(Box::new(BinaryExpression::new(lhs, rhs, info)));
            }
            TokenType::Slash => {
                let info = token.info();
                lexer.next();
                let rhs = bitwise::parse(lexer, functions)?;
                lhs = Expression::Divide(Box::new(BinaryExpression::new(lhs, rhs, info)));
            }
            _ => break,
        }
    }
    Ok(lhs)
}
