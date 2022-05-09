use crate::{
    exprs::{AssignExpression, VariableExpression},
    lexer::{Lexer, TokenInfo, TokenType},
};

use super::{call, index, ParserResult, result, Parser};

pub fn parse(lexer: &mut Lexer, parser: &mut Parser, name: String, info: TokenInfo, require: bool) -> ParserResult {
    if !parser.stack_mut().last_mut().unwrap().contains(&name) {
        if !parser.stack_mut().last_mut().unwrap().push_closure(name.clone()) {
            return result::already_exist(info);
        }
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
                );
            }
            _ => (),
        }
    }
    Ok(VariableExpression::new(name, info))
}
