use crate::{
    exprs::{CallExpression, Expression},
    lexer::{Lexer, TokenInfo, TokenType},
};

use super::{index, ParserResult, result, Parser};

pub fn parse(
    lexer: &mut Lexer,
    parser: &mut Parser,
    from: Expression,
    info: TokenInfo,
    require: bool,
    is_async: bool,
) -> ParserResult {
    let mut exprs = Vec::new();

    loop {
        match lexer.peek(true) {
            Some(token) => match token.ttype() {
                TokenType::RightParenthesis => {
                    lexer.next(true);
                    break;
                }
                _ => (),
            },
            None => return result::unexpected_eof(),
        }

        exprs.push(super::parse_expression(lexer, parser, true)?);

        match lexer.peek(true) {
            Some(token) => match token.ttype() {
                TokenType::RightParenthesis => {
                    lexer.next(true);
                    break;
                }
                TokenType::Comma => {
                    lexer.next(true);
                }
                _ => return result::unexpected(token.info()),
            },
            None => return result::unexpected_eof(),
        }
    }

    if let Some(token) = lexer.peek(require) {
        match token.ttype() {
            TokenType::LeftSquareBracket => {
                lexer.next(true);
                return index::parse(
                    lexer,
                    parser,
                    CallExpression::new(from, exprs, is_async, info.clone()),
                    info,
                    require,
                );
            }
            TokenType::LeftParenthesis => {
                lexer.next(true);
                return parse(
                    lexer,
                    parser,
                    CallExpression::new(from, exprs, is_async, info.clone()),
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
                            return parse(
                                lexer,
                                parser,
                                CallExpression::new(from, exprs, is_async, info.clone()),
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

    Ok(CallExpression::new(from, exprs, is_async, info))
}
