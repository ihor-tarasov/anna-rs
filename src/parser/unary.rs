use crate::{
    exprs::{Expression, UnaryExpression},
    lexer::{Lexer, TokenType},
};

use super::{primary, unexpected_eof, ParserResult};

pub fn parse(lexer: &mut Lexer) -> ParserResult {
    if let Some(token) = lexer.peek() {
        match token.ttype() {
            TokenType::Minus => {
                let info = token.info();
                lexer.next();
                let expr = primary::parse(lexer)?;
                Ok(Expression::UnaryMinus(Box::new(UnaryExpression::new(
                    expr, info,
                ))))
            }
            TokenType::Exclamation => {
                let info = token.info();
                lexer.next();
                let expr = primary::parse(lexer)?;
                Ok(Expression::UnaryNot(Box::new(UnaryExpression::new(
                    expr, info,
                ))))
            }
            _ => primary::parse(lexer),
        }
    } else {
        unexpected_eof()
    }
}
