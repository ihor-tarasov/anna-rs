use std::collections::HashSet;

use crate::{
    lexer::{Lexer, TokenInfo, TokenType},
    Functions, exprs::BlockExpression,
};

mod array;
mod bitwise;
mod block;
mod caching;
mod call;
mod comparison;
mod equality;
mod factor;
mod function;
mod identifier;
mod if_parser;
mod index;
mod primary;
mod result;
mod term;
mod unary;
mod var;
mod while_parser;
mod for_parser;

pub use result::ParserError;
pub use result::ParserErrorType;
pub use result::ParserResult;

use self::block::BlockGuard;

pub type ParserBlock = HashSet<String>;

pub struct ParserFrame {
    variables: Vec<ParserBlock>,
    closure: HashSet<String>,
}

impl ParserFrame {
    pub fn new() -> Self {
        Self {
            variables: Vec::new(),
            closure: HashSet::new(),
        }
    }

    pub fn push_block(&mut self, block: ParserBlock) {
        self.variables.push(block);
    }

    pub fn pop_block(&mut self) {
        self.variables.pop().unwrap();
    }

    pub fn push_variable(&mut self, name: String) -> bool {
        self.variables.last_mut().unwrap().insert(name)
    }

    pub fn contains(&self, name: &String) -> bool {
        for block in self.variables.iter().rev() {
            if block.contains(name) {
                return true;
            }
        }
        false
    }

    pub fn push_closure(&mut self, name: String) -> bool {
        self.closure.insert(name)
    }

    pub fn merge_last(&mut self) {
        let last = self.variables.pop().unwrap();
        let target = self.variables.last_mut().unwrap();
        for name in last {
            target.insert(name);
        }
    }
}

pub type ParserStack = Vec<ParserFrame>;

pub struct Parser<'a> {
    functions: &'a mut Functions,
    stack: &'a mut ParserStack,
}

impl<'a> Parser<'a> {
    pub fn new(functions: &'a mut Functions, stack: &'a mut ParserStack) -> Self {
        Self {
            functions,
            stack,
        }
    }

    pub fn functions(&self) -> &Functions {
        self.functions
    }

    pub fn functions_mut(&mut self) -> &mut Functions {
        self.functions
    }

    pub fn stack(&self) -> &ParserStack {
        &self.stack
    }

    pub fn stack_mut(&mut self) -> &mut ParserStack {
        &mut self.stack
    }
}

pub fn parse_expression(lexer: &mut Lexer, parser: &mut Parser, require: bool) -> ParserResult {
    equality::parse(lexer, parser, require)
}

pub fn parse_block(lexer: &mut Lexer, parser: &mut Parser) -> ParserResult {
    let mut guard = BlockGuard::new(parser);

    let mut stats = Vec::new();

    stats.push(parse_expression(
        lexer,
        guard.parser_mut(),
        true,
    )?);

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
            break;
        }

        match lexer.peek(true) {
            Some(_) => (),
            None => break,
        }

        stats.push(parse_expression(lexer, guard.parser_mut(), true)?);
    }
    Ok(BlockExpression::new(stats))
}

pub fn parse(lexer: &mut Lexer, parser: &mut Parser) -> ParserResult {
    let result = match lexer.peek(false) {
        Some(_) => parse_expression(lexer, parser, false)?,
        None => {
            return Err(ParserError::new(
                ParserErrorType::Empty,
                TokenInfo::new(0, 0),
            ))
        }
    };
    if let Some(token) = lexer.next(false) {
        result::unexpected(token.info())
    } else {
        Ok(result)
    }
}
