use crate::{
    exprs::{BinaryExpression, Expression},
    lexer::{Lexer, TokenType}, State,
};

use super::{ParserResult, unary};

pub fn parse(lexer: &mut Lexer, state: &mut State) -> ParserResult {
    let mut lhs = unary::parse(lexer, state)?;
    while let Some(token) = lexer.peek() {
        match token.ttype() {
            TokenType::Ampersand => {
                let info = token.info();
                lexer.next();
                let rhs = unary::parse(lexer, state)?;
                lhs = Expression::BitwiseAnd(Box::new(BinaryExpression::new(lhs, rhs, info)));
            },
            TokenType::VerticalBar => {
                let info = token.info();
                lexer.next();
                let rhs = unary::parse(lexer, state)?;
                lhs = Expression::BitwiseOr(Box::new(BinaryExpression::new(lhs, rhs, info)));
            },
            TokenType::Circumflex => {
                let info = token.info();
                lexer.next();
                let rhs = unary::parse(lexer, state)?;
                lhs = Expression::BitwiseXor(Box::new(BinaryExpression::new(lhs, rhs, info)));
            },
            TokenType::LessLess => {
                let info = token.info();
                lexer.next();
                let rhs = unary::parse(lexer, state)?;
                lhs = Expression::BitwiseShl(Box::new(BinaryExpression::new(lhs, rhs, info)));
            },
            TokenType::GreaterGreater => {
                let info = token.info();
                lexer.next();
                let rhs = unary::parse(lexer, state)?;
                lhs = Expression::BitwiseShr(Box::new(BinaryExpression::new(lhs, rhs, info)));
            },
            _ => break,
        }
    }
    Ok(lhs)
}
