use crate::{
    exprs::{Expression, IndexExpression},
    lexer::{Lexer, TokenInfo, TokenType},
};

use super::{call, ParserResult, result, Parser};

pub fn parse(
    lexer: &mut Lexer,
    parser: &mut Parser,
    from: Expression,
    info: TokenInfo,
    require: bool,
) -> ParserResult {
    let index = super::parse_expression(lexer, parser, true)?;
    if let Some(token) = lexer.next(true) {
        match token.take_type() {
            TokenType::RightSquareBracket => (),
            _ => return result::unexpected(info),
        }
    }

    if let Some(token) = lexer.peek(require) {
        match token.ttype() {
            TokenType::LeftSquareBracket => {
                lexer.next(true);
                return parse(
                    lexer,
                    parser,
                    IndexExpression::new(from, index, info.clone()),
                    info,
                    require,
                );
            }
            TokenType::LeftParenthesis => {
                lexer.next(true);
                return call::parse(
                    lexer,
                    parser,
                    IndexExpression::new(from, index, info.clone()),
                    info,
                    require,
                    false,
                );
            }
            TokenType::Exclamation => {
                lexer.next(true);
                if let Some(token) = lexer.next(true) {
                    let info = token.info();
                    match token.ttype() {
                        TokenType::LeftParenthesis => {
                            return call::parse(
                                lexer,
                                parser,
                                IndexExpression::new(from, index, info.clone()),
                                info,
                                require,
                                true,
                            );
                        }
                        _ => return result::unexpected(info),
                    }
                } else {
                    return result::unexpected_eof();
                }
            }
            _ => (),
        }
    }

    Ok(IndexExpression::new(from, index, info))
}
