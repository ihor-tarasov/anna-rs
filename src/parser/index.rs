use crate::{
    exprs::{Expression, IndexExpression},
    lexer::{Lexer, TokenInfo, TokenType},
    Functions,
};

use super::{call, ParserResult, result};

pub fn parse(
    lexer: &mut Lexer,
    functions: &mut Functions,
    from: Expression,
    info: TokenInfo,
) -> ParserResult {
    let index = super::parse_expression(lexer, functions)?;
    if let Some(token) = lexer.next() {
        match token.take_type() {
            TokenType::RightSquareBracket => (),
            _ => return result::unexpected(info),
        }
    }

    if let Some(token) = lexer.peek() {
        match token.ttype() {
            TokenType::LeftSquareBracket => {
                lexer.next();
                return parse(
                    lexer,
                    functions,
                    IndexExpression::new(from, index, info.clone()),
                    info,
                );
            }
            TokenType::LeftParenthesis => {
                lexer.next();
                return call::parse(
                    lexer,
                    functions,
                    IndexExpression::new(from, index, info.clone()),
                    info,
                    false,
                );
            }
            TokenType::Exclamation => {
                lexer.next();
                match lexer.peek() {
                    Some(token) => match token.ttype() {
                        TokenType::LeftParenthesis => {
                            lexer.next();
                            return call::parse(
                                lexer,
                                functions,
                                IndexExpression::new(from, index, info.clone()),
                                info,
                                true,
                            );
                        },
                        _ => return result::unexpected(token.info())
                    },
                    None => return result::unexpected_eof(),
                }
            }
            _ => (),
        }
    }

    Ok(IndexExpression::new(from, index, info))
}
