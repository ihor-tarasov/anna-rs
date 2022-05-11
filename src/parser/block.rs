use crate::{
    exprs::BlockExpression,
    lexer::{Lexer, TokenType},
};

use super::{result, Parser, ParserResult, ParserBlock};

pub struct BlockGuard<'a, 'b> {
    parser: &'a mut Parser<'b>,
}

impl<'a, 'b> BlockGuard<'a, 'b> {
    pub fn new(parser: &'a mut Parser<'b>) -> Self {
        parser.stack_mut().last_mut().unwrap().push_block(ParserBlock::new());
        Self { parser }
    }

    pub fn parser_mut(&mut self) -> &mut Parser<'b> {
        self.parser
    }
}

impl<'a, 'b> Drop for BlockGuard<'a, 'b> {
    fn drop(&mut self) {
        self.parser.stack_mut().last_mut().unwrap().pop_block();
    }
}

pub fn parse(lexer: &mut Lexer, parser: &mut Parser, require: bool) -> ParserResult {
    let is_multiline = if let Some(token) = lexer.peek(true) {
        match token.ttype() {
            TokenType::LeftBrace => {
                lexer.next(true);
                true
            }
            _ => false,
        }
    } else {
        return result::unexpected_eof();
    };

    let mut guard = BlockGuard::new(parser);

    let mut stats = Vec::new();

    stats.push(super::parse_expression(
        lexer,
        guard.parser_mut(),
        if is_multiline { true } else { require },
    )?);

    if is_multiline {
        loop {
            if let Some(token) = lexer.peek(true) {
                match token.ttype() {
                    TokenType::Semicolon => {
                        lexer.next(true);
                        ()
                    }
                    _ => (),
                }
            } else {
                return result::unexpected_eof();
            }

            if let Some(token) = lexer.peek(true) {
                match token.ttype() {
                    TokenType::RightBrace => {
                        lexer.next(true);
                        break;
                    }
                    _ => (),
                }
            } else {
                return result::unexpected_eof();
            }

            stats.push(super::parse_expression(lexer, guard.parser_mut(), true)?);
        }
    }

    Ok(BlockExpression::new(stats))
}
