use std::collections::HashSet;

use crate::{lexer::{Lexer, TokenType, TokenInfo}, exprs::{Expression, ClosureExpression}, Function};

use super::{ParserResult, block, ParserError, ParserErrorType, result, Parser};

pub fn parse(lexer: &mut Lexer, parser: &mut Parser, info: TokenInfo) -> ParserResult {
    let mut args = HashSet::new();

    loop {
        if let Some(token) = lexer.next() {
            let info = token.info();
            match token.take_type() {
                TokenType::Identifier(name) => {
                    if !args.insert(name.clone()) {
                        return Err(ParserError::new(ParserErrorType::ArgumentAlreadyExist, info));
                    }
                },
                TokenType::VerticalBar => {
                    break;
                },
                _ => return result::unexpected(info),
            }
        } else {
            return result::unexpected_eof();
        }

        if let Some(token) = lexer.next() {
            let info = token.info();
            match token.take_type() {
                TokenType::Comma => (),
                TokenType::VerticalBar => break,
                _ => return result::unexpected(info),
            }
        } else {
            return result::unexpected_eof();
        }
    }

    let block = match block::parse(lexer, parser)? {
        Expression::Block(block) => block,
        _ => panic!("Expected BlockExpression"),
    };

    let function = Function::new(args, block, info);

    let id = parser.functions_mut().push(function);

    Ok(ClosureExpression::new(HashSet::new(), id))
}
