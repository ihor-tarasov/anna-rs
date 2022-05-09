use std::collections::HashSet;

use crate::{lexer::{Lexer, TokenType, TokenInfo}, exprs::{Expression, ClosureExpression}, Function};

use super::{ParserResult, block, ParserError, ParserErrorType, result, Parser, ParserFrame};

struct FrameGuard<'a, 'b> {
    parser: &'a mut Parser<'b>,
}

impl<'a, 'b> FrameGuard<'a, 'b> {
    fn new(parser: &'a mut Parser<'b>) -> Self {
        parser.stack_mut().push(ParserFrame::new());
        parser.stack_mut().last_mut().unwrap().push_block();
        Self { parser }
    }

    fn parser_mut(&mut self) -> &mut Parser<'b> {
        self.parser
    }
}

impl<'a, 'b> Drop for FrameGuard<'a, 'b> {
    fn drop(&mut self) {
        self.parser.stack_mut().pop().unwrap();
    }
}

pub fn parse(lexer: &mut Lexer, parser: &mut Parser, info: TokenInfo, require: bool) -> ParserResult {
    let mut args = HashSet::new();

    let mut guard = FrameGuard::new(parser);

    loop {
        if let Some(token) = lexer.next(true) {
            let info = token.info();
            match token.take_type() {
                TokenType::Identifier(name) => {
                    if !args.insert(name.clone()) {
                        if !guard.parser_mut().stack_mut().last_mut().unwrap().push_variable(name.clone()) {
                            return Err(ParserError::new(ParserErrorType::ArgumentAlreadyExist, info));
                        }
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

        if let Some(token) = lexer.next(true) {
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

    let block = match block::parse(lexer, guard.parser_mut(), require)? {
        Expression::Block(block) => block,
        _ => panic!("Expected BlockExpression"),
    };

    let function = Function::new(args, block, info);

    let id = guard.parser_mut().functions_mut().push(function);

    Ok(ClosureExpression::new(guard.parser_mut().stack_mut().last_mut().unwrap().closure.clone(), id))
}
