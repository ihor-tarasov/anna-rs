use crate::{
    exprs::{AssignExpression, VariableExpression},
    lexer::{Lexer, TokenInfo, TokenType},
};

use super::{call, index, result, Parser, ParserResult};

pub fn parse(
    lexer: &mut Lexer,
    parser: &mut Parser,
    name: String,
    info: TokenInfo,
    require: bool,
) -> ParserResult {
    if !parser.stack_mut().last_mut().unwrap().contains(&name) {
        parser
            .stack_mut()
            .last_mut()
            .unwrap()
            .push_closure(name.clone());
    }

    if let Some(token) = lexer.peek(require) {
        match token.ttype() {
            TokenType::Equal => {
                lexer.next(true);
                return Ok(AssignExpression::new(
                    super::parse_expression(lexer, parser, require)?,
                    name,
                    info,
                ));
            }
            TokenType::LeftSquareBracket => {
                lexer.next(true);
                return index::parse(
                    lexer,
                    parser,
                    VariableExpression::new(name, info.clone()),
                    info,
                    require,
                );
            }
            TokenType::LeftParenthesis => {
                lexer.next(true);
                return call::parse(
                    lexer,
                    parser,
                    VariableExpression::new(name, info.clone()),
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
                                VariableExpression::new(name, info.clone()),
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
    Ok(VariableExpression::new(name, info))
}
