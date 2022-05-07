use crate::{lexer::{Lexer, TokenType}, exprs::BlockExpression, Functions};

use super::{parse_expression, unexpected_eof, ParserResult};

pub fn parse(lexer: &mut Lexer, functions: &mut Functions) -> ParserResult {
    let is_multiline = if let Some(token) = lexer.peek() {
        match token.ttype() {
            TokenType::LeftBrace => {
                lexer.next();
                true
            }
            _ => false,
        }
    } else {
        return unexpected_eof();
    };

    let mut stats = Vec::new();

    stats.push(parse_expression(lexer, functions)?);

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
                return unexpected_eof();
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
                return unexpected_eof();
            }

            stats.push(parse_expression(lexer, functions)?);
        }
    }

    Ok(BlockExpression::new(stats))
}
