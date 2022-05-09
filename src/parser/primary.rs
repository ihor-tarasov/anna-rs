use crate::{
    exprs::{LiteralExpression, StringLiteralExpression},
    lexer::{Lexer, TokenType},
    types::Value,
};

use super::{
    array, caching, function, identifier, if_parser, var,
    while_parser, ParserResult, result, Parser, block, for_parser,
};

pub fn parse(lexer: &mut Lexer, parser: &mut Parser, require: bool) -> ParserResult {
    if let Some(token) = lexer.peek(require) {
        match token.ttype() {
            TokenType::LeftBrace => {
                return block::parse(lexer, parser, true);
            }
            _ => (),
        }
    }

    let token = match lexer.next(require) {
        Some(token) => token,
        None => return result::unexpected_eof(),
    };
    let info = token.info();
    match token.take_type() {
        TokenType::Integer(value) => Ok(LiteralExpression::new(Value::Integer(value))),
        TokenType::Real(value) => Ok(LiteralExpression::new(Value::Real(value))),
        TokenType::Identifier(name) => identifier::parse(lexer, parser, name, info, require),
        TokenType::Var => var::parse(lexer, parser, require),
        TokenType::If => if_parser::parse(lexer, parser, info, require),
        TokenType::While => while_parser::parse(lexer, parser, info, require),
        TokenType::LeftSquareBracket => array::parse(lexer, parser),
        TokenType::Break => caching::parse(lexer, parser, Value::Break, require),
        TokenType::Return => caching::parse(lexer, parser, Value::Return, require),
        TokenType::Continue => Ok(LiteralExpression::new(Value::Continue)),
        TokenType::VerticalBar => function::parse(lexer, parser, info, require),
        TokenType::String(value) => Ok(StringLiteralExpression::new(value)),
        TokenType::For => for_parser::parse(lexer, parser, info, require),
        TokenType::Unknown => result::unknown(info),
        _ => result::unexpected(info),
    }
}
