use std::collections::HashSet;

use crate::{lexer::{Lexer, TokenType, TokenInfo}, exprs::{Expression, ClosureExpression}, Function, Functions};

use super::{ParserResult, unexpected, unexpected_eof, block, ParserError, ParserErrorType};

pub fn parse(lexer: &mut Lexer, functions: &mut Functions, info: TokenInfo) -> ParserResult {
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
                _ => return unexpected(info),
            }
        } else {
            return unexpected_eof();
        }

        if let Some(token) = lexer.next() {
            let info = token.info();
            match token.take_type() {
                TokenType::Comma => (),
                TokenType::VerticalBar => break,
                _ => return unexpected(info),
            }
        } else {
            return unexpected_eof();
        }
    }

    let block = match block::parse(lexer, functions)? {
        Expression::Block(block) => block,
        _ => panic!("Expected BlockExpression"),
    };

    let function = Function::new(args, block, info);

    let id = functions.push(function);

    Ok(ClosureExpression::new(HashSet::new(), id))
}
