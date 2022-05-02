use crate::{
    exprs::{Expression, IfExpression},
    lexer::{Lexer, TokenInfo, TokenType},
};

use super::{block, parse_expression, ParserResult, unexpected_eof};

pub fn parse(lexer: &mut Lexer, if_info: TokenInfo) -> ParserResult {
    let mut conditions = Vec::new();
    let mut blocks = Vec::new();
    let mut info = Vec::new();
    let mut else_block = None;
    info.push(if_info);

    loop {
        let condition = parse_expression(lexer)?;
        let block = match block::parse(lexer)? {
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
                                else_block = Some(match block::parse(lexer)? {
                                    Expression::Block(block) => block,
                                    _ => panic!("Expected BlockExpression"),
                                });
                                break;
                            }
                        }
                    } else {
                        return unexpected_eof();
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
