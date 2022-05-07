use crate::{
    exprs::{Expression, IfExpression},
    lexer::{Lexer, TokenInfo, TokenType},
    Functions,
};

use super::{block, result, ParserResult};

pub fn parse(lexer: &mut Lexer, functions: &mut Functions, if_info: TokenInfo) -> ParserResult {
    let mut conditions = Vec::new();
    let mut blocks = Vec::new();
    let mut info = Vec::new();
    let mut else_block = None;
    info.push(if_info);

    loop {
        let condition = super::parse_expression(lexer, functions)?;
        let block = match block::parse(lexer, functions)? {
            Expression::Block(block) => block,
            _ => panic!("Expected BlockExpression"),
        };

        conditions.push(condition);
        blocks.push(block);

        if let Some(token) = lexer.peek() {
            match token.ttype() {
                TokenType::Else => {
                    lexer.next();
                    if let Some(token) = lexer.peek() {
                        match token.ttype() {
                            TokenType::If => {
                                info.push(token.info());
                                lexer.next();
                            }
                            _ => {
                                else_block = Some(match block::parse(lexer, functions)? {
                                    Expression::Block(block) => block,
                                    _ => panic!("Expected BlockExpression"),
                                });
                                break;
                            }
                        }
                    } else {
                        return result::unexpected_eof();
                    }
                }
                _ => break,
            }
        } else {
            break;
        }
    }

    Ok(IfExpression::new(conditions, blocks, else_block, info))
}
