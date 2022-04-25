use crate::{
    exprs::{BinaryExpression, Expression},
    lexer::{Lexer, TokenType},
};

use super::{ParserResult, unary};

pub fn parse(lexer: &mut Lexer) -> ParserResult {
    let mut lhs = unary::parse(lexer)?;
    while let Some(token) = lexer.peek() {
        match token.ttype() {
            TokenType::Ampersand => {
                let info = token.info();
                lexer.next();
                let rhs = unary::parse(lexer)?;
                lhs = Expression::BitwiseAnd(Box::new(BinaryExpression::new(lhs, rhs, info)));
            },
            TokenType::VerticalBar => {
                let info = token.info();
                lexer.next();
                let rhs = unary::parse(lexer)?;
                lhs = Expression::BitwiseOr(Box::new(BinaryExpression::new(lhs, rhs, info)));
            },
            TokenType::Circumflex => {
                let info = token.info();
                lexer.next();
                let rhs = unary::parse(lexer)?;
                lhs = Expression::BitwiseXor(Box::new(BinaryExpression::new(lhs, rhs, info)));
            },
            TokenType::LessLess => {
                let info = token.info();
                lexer.next();
                let rhs = unary::parse(lexer)?;
                lhs = Expression::BitwiseShl(Box::new(BinaryExpression::new(lhs, rhs, info)));
            },
            TokenType::GreaterGreater => {
                let info = token.info();
                lexer.next();
                let rhs = unary::parse(lexer)?;
                lhs = Expression::BitwiseShr(Box::new(BinaryExpression::new(lhs, rhs, info)));
            },
            _ => break,
        }
    }
    Ok(lhs)
}
