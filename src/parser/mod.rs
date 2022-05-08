use std::collections::HashSet;

use crate::{
    lexer::{Lexer, TokenInfo},
    Functions,
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

type ParserBlock = HashSet<String>;

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

    pub fn push_block(&mut self) {
        self.variables.push(HashSet::new());
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

pub fn parse_expression(lexer: &mut Lexer, parser: &mut Parser) -> ParserResult {
    equality::parse(lexer, parser)
}

pub fn parse(lexer: &mut Lexer, parser: &mut Parser) -> ParserResult {
    let result = match lexer.peek() {
        Some(_) => parse_expression(lexer, parser)?,
        None => {
            return Err(ParserError::new(
                ParserErrorType::Empty,
                TokenInfo::new(0, 0),
            ))
        }
    };
    if let Some(token) = lexer.peek() {
        result::unexpected(token.info())
    } else {
        Ok(result)
    }
}
