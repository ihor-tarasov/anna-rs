use std::collections::HashSet;

use crate::{lexer::{Lexer, TokenType}, exprs::{Expression, ClosureExpression}, Function};

use super::{ParserResult, block, ParserError, ParserErrorType, result, Parser, ParserFrame, ParserBlock};

struct FrameGuard<'a, 'b> {
    parser: &'a mut Parser<'b>,
}

impl<'a, 'b> FrameGuard<'a, 'b> {
    fn new(parser: &'a mut Parser<'b>) -> Self {
        parser.stack_mut().push(ParserFrame::new());
        parser.stack_mut().last_mut().unwrap().push_block(ParserBlock::new());
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

pub fn parse(lexer: &mut Lexer, parser: &mut Parser, require: bool) -> ParserResult {
    let mut args = HashSet::new();

    let mut guard = FrameGuard::new(parser);

    loop {
        if let Some(token) = lexer.next(true) {
            let info = token.info();
            match token.take_type() {
                TokenType::Identifier(name) => {
                    if !args.insert(name.clone()) {
                        return Err(ParserError::new(ParserErrorType::ArgumentAlreadyExist, info));
                    }
                    if !guard.parser_mut().stack_mut().last_mut().unwrap().push_variable(name.clone()) {
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

    let function = Function::new(args, block);

    let id = guard.parser_mut().functions_mut().push(function);

    let curr_frame = guard.parser_mut().stack_mut().pop().unwrap();
    let last_frame = guard.parser_mut().stack_mut().last_mut().unwrap();

    for variable in &curr_frame.closure {
        if !last_frame.contains(variable) {
            last_frame.push_closure(variable.clone());
        }
    }

    guard.parser_mut().stack_mut().push(curr_frame);

    Ok(ClosureExpression::new(guard.parser_mut().stack().last().unwrap().closure.clone(), id))
}
