use crate::{
    exprs::{BinaryExpression, Expression},
    lexer::{Lexer, TokenType},
};

use super::{ParserResult, unary, Parser};

pub fn parse(lexer: &mut Lexer, parser: &mut Parser, require: bool) -> ParserResult {
    let mut lhs = unary::parse(lexer, parser, require)?;
    while let Some(token) = lexer.peek(require) {
        match token.ttype() {
            TokenType::Ampersand => {
                let info = token.info();
                lexer.next(true);
                lexer.peek(true);
                let rhs = unary::parse(lexer, parser, require)?;
                lhs = Expression::BitwiseAnd(Box::new(BinaryExpression::new(lhs, rhs, info)));
            },
            TokenType::VerticalBar => {
                let info = token.info();
                lexer.next(true);
                lexer.peek(true);
                let rhs = unary::parse(lexer, parser, require)?;
                lhs = Expression::BitwiseOr(Box::new(BinaryExpression::new(lhs, rhs, info)));
            },
            TokenType::Circumflex => {
                let info = token.info();
                lexer.next(true);
                lexer.peek(true);
                let rhs = unary::parse(lexer, parser, require)?;
                lhs = Expression::BitwiseXor(Box::new(BinaryExpression::new(lhs, rhs, info)));
            },
            TokenType::LessLess => {
                let info = token.info();
                lexer.next(true);
                lexer.peek(true);
                let rhs = unary::parse(lexer, parser, require)?;
                lhs = Expression::BitwiseShl(Box::new(BinaryExpression::new(lhs, rhs, info)));
            },
            TokenType::GreaterGreater => {
                let info = token.info();
                lexer.next(true);
                lexer.peek(true);
                let rhs = unary::parse(lexer, parser, require)?;
                lhs = Expression::BitwiseShr(Box::new(BinaryExpression::new(lhs, rhs, info)));
            },
            _ => break,
        }
    }
    Ok(lhs)
}
