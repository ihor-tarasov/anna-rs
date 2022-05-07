use crate::{
    exprs::{Expression, UnaryExpression},
    lexer::{Lexer, TokenType},
};

use super::{primary, ParserResult, result, Parser};

pub fn parse(lexer: &mut Lexer, parser: &mut Parser) -> ParserResult {
    if let Some(token) = lexer.peek() {
        match token.ttype() {
            TokenType::Minus => {
                let info = token.info();
                lexer.next();
                let expr = primary::parse(lexer, parser)?;
                Ok(Expression::UnaryMinus(Box::new(UnaryExpression::new(
                    expr, info,
                ))))
            }
            TokenType::Exclamation => {
                let info = token.info();
                lexer.next();
                let expr = primary::parse(lexer, parser)?;
                Ok(Expression::UnaryNot(Box::new(UnaryExpression::new(
                    expr, info,
                ))))
            }
            _ => primary::parse(lexer, parser),
        }
    } else {
        result::unexpected_eof()
    }
}
