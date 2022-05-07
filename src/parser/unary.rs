use crate::{
    exprs::{Expression, UnaryExpression},
    lexer::{Lexer, TokenType},
    Functions,
};

use super::{primary, ParserResult, result};

pub fn parse(lexer: &mut Lexer, functions: &mut Functions) -> ParserResult {
    if let Some(token) = lexer.peek() {
        match token.ttype() {
            TokenType::Minus => {
                let info = token.info();
                lexer.next();
                let expr = primary::parse(lexer, functions)?;
                Ok(Expression::UnaryMinus(Box::new(UnaryExpression::new(
                    expr, info,
                ))))
            }
            TokenType::Exclamation => {
                let info = token.info();
                lexer.next();
                let expr = primary::parse(lexer, functions)?;
                Ok(Expression::UnaryNot(Box::new(UnaryExpression::new(
                    expr, info,
                ))))
            }
            _ => primary::parse(lexer, functions),
        }
    } else {
        result::unexpected_eof()
    }
}
