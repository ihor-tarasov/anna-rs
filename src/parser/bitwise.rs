use crate::{
    exprs::{BinaryExpression, Expression},
    lexer::{Lexer, TokenType}, Functions,
};

use super::{ParserResult, unary};

pub fn parse(lexer: &mut Lexer, functions: &mut Functions) -> ParserResult {
    let mut lhs = unary::parse(lexer, functions)?;
    while let Some(token) = lexer.peek() {
        match token.ttype() {
            TokenType::Ampersand => {
                let info = token.info();
                lexer.next();
                let rhs = unary::parse(lexer, functions)?;
                lhs = Expression::BitwiseAnd(Box::new(BinaryExpression::new(lhs, rhs, info)));
            },
            TokenType::VerticalBar => {
                let info = token.info();
                lexer.next();
                let rhs = unary::parse(lexer, functions)?;
                lhs = Expression::BitwiseOr(Box::new(BinaryExpression::new(lhs, rhs, info)));
            },
            TokenType::Circumflex => {
                let info = token.info();
                lexer.next();
                let rhs = unary::parse(lexer, functions)?;
                lhs = Expression::BitwiseXor(Box::new(BinaryExpression::new(lhs, rhs, info)));
            },
            TokenType::LessLess => {
                let info = token.info();
                lexer.next();
                let rhs = unary::parse(lexer, functions)?;
                lhs = Expression::BitwiseShl(Box::new(BinaryExpression::new(lhs, rhs, info)));
            },
            TokenType::GreaterGreater => {
                let info = token.info();
                lexer.next();
                let rhs = unary::parse(lexer, functions)?;
                lhs = Expression::BitwiseShr(Box::new(BinaryExpression::new(lhs, rhs, info)));
            },
            _ => break,
        }
    }
    Ok(lhs)
}
