use crate::{
    exprs::{Expression, UnaryExpression},
    lexer::{Lexer, TokenType},
};

use super::{primary, ParserResult, result, Parser};

pub fn parse(lexer: &mut Lexer, parser: &mut Parser, require: bool) -> ParserResult {
    if let Some(token) = lexer.peek(require) {
        match token.ttype() {
            TokenType::Minus => {
                let info = token.info();
                lexer.next(true);
                lexer.peek(true);
                let expr = primary::parse(lexer, parser, require)?;
                Ok(Expression::UnaryMinus(Box::new(UnaryExpression::new(
                    expr, info,
                ))))
            }
            TokenType::Exclamation => {
                let info = token.info();
                lexer.next(true);
                lexer.peek(true);
                let expr = primary::parse(lexer, parser, require)?;
                Ok(Expression::UnaryNot(Box::new(UnaryExpression::new(
                    expr, info,
                ))))
            }
            _ => primary::parse(lexer, parser, require),
        }
    } else {
        result::unexpected_eof()
    }
}
