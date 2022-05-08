use crate::{lexer::{Lexer, TokenType}, exprs::BlockExpression};

use super::{ParserResult, result, Parser};

struct BlockGuard<'a, 'b> {
    parser: &'a mut Parser<'b>,
}

impl<'a, 'b> BlockGuard<'a, 'b> {
    pub fn new(parser: &'a mut Parser<'b>) -> Self {
        parser.stack_mut().last_mut().unwrap().push_block();
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

pub fn parse(lexer: &mut Lexer, parser: &mut Parser) -> ParserResult {
    let is_multiline = if let Some(token) = lexer.peek() {
        match token.ttype() {
            TokenType::LeftBrace => {
                lexer.next();
                true
            }
            _ => false,
        }
    } else {
        return result::unexpected_eof();
    };

    let mut guard = BlockGuard::new(parser);

    let mut stats = Vec::new();

    stats.push(super::parse_expression(lexer, guard.parser_mut())?);

    if is_multiline {
        loop {
            if let Some(token) = lexer.peek() {
                match token.ttype() {
                    TokenType::Semicolon => {
                        lexer.next();
                        ()
                    }
                    _ => (),
                }
            } else {
                return result::unexpected_eof();
            }

            if let Some(token) = lexer.peek() {
                match token.ttype() {
                    TokenType::RightBrace => {
                        lexer.next();
                        break;
                    }
                    _ => (),
                }
            } else {
                return result::unexpected_eof();
            }

            stats.push(super::parse_expression(lexer, guard.parser_mut())?);
        }
    }

    Ok(BlockExpression::new(stats))
}
