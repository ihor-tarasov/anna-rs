use crate::{
    exprs::{AssignExpression, VariableExpression},
    lexer::{Lexer, TokenInfo, TokenType},
};

use super::{call, index, ParserResult, result, Parser};

pub fn parse(lexer: &mut Lexer, parser: &mut Parser, name: String, info: TokenInfo) -> ParserResult {
    if !parser.stack_mut().last_mut().unwrap().contains(&name) {
        if !parser.stack_mut().last_mut().unwrap().push_closure(name.clone()) {
            return result::already_exist(info);
        }
    }

    if let Some(token) = lexer.peek() {
        match token.ttype() {
            TokenType::Equal => {
                lexer.next();
                return Ok(AssignExpression::new(
                    super::parse_expression(lexer, parser)?,
                    name,
                    info,
                ));
            }
            TokenType::LeftSquareBracket => {
                lexer.next();
                return index::parse(
                    lexer,
                    parser,
                    VariableExpression::new(name, info.clone()),
                    info,
                );
            }
            TokenType::LeftParenthesis => {
                lexer.next();
                return call::parse(
                    lexer,
                    parser,
                    VariableExpression::new(name, info.clone()),
                    info,
                    false,
                );
            }
            _ => (),
        }
    }
    Ok(VariableExpression::new(name, info))
}
