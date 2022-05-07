use crate::{
    exprs::{AssignExpression, VariableExpression},
    lexer::{Lexer, TokenInfo, TokenType},
    Functions,
};

use super::{call, index, ParserResult, result};

pub fn parse(lexer: &mut Lexer, functions: &mut Functions, name: String, info: TokenInfo) -> ParserResult {
    if let Some(token) = lexer.peek() {
        match token.ttype() {
            TokenType::Equal => {
                lexer.next();
                return Ok(AssignExpression::new(
                    super::parse_expression(lexer, functions)?,
                    name,
                    info,
                ));
            }
            TokenType::LeftSquareBracket => {
                lexer.next();
                return index::parse(
                    lexer,
                    functions,
                    VariableExpression::new(name, info.clone()),
                    info,
                );
            }
            TokenType::LeftParenthesis => {
                lexer.next();
                return call::parse(
                    lexer,
                    functions,
                    VariableExpression::new(name, info.clone()),
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
                                VariableExpression::new(name, info.clone()),
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
    Ok(VariableExpression::new(name, info))
}
