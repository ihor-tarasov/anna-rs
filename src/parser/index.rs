use crate::{
    exprs::{Expression, IndexExpression},
    lexer::{Lexer, TokenInfo, TokenType},
};

use super::{unexpected, ParserResult};

pub fn parse(lexer: &mut Lexer, from: Expression, info: TokenInfo) -> ParserResult {
    let index = super::parse_expression(lexer)?;
    if let Some(token) = lexer.next() {
        match token.take_type() {
            TokenType::RightSquareBracket => (),
            _ => return unexpected(info),
        }
    }

    if let Some(token) = lexer.peek() {
        match token.ttype() {
            TokenType::LeftSquareBracket => {
                lexer.next();
                return parse(lexer, IndexExpression::new(from, index, info.clone()), info);
            },
            _ => (),
        }
    }

    Ok(IndexExpression::new(from, index, info))
}
