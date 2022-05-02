use std::collections::HashMap;

use crate::types::Value;

pub type Block = HashMap<String, Value>;

pub struct StackFrame {
    blocks: Vec<Block>,
}

impl StackFrame {
    pub fn new() -> Self {
        Self {
            blocks: Vec::new(),
        }
    }

    pub fn with_closure(closure: HashMap<String, Value>) -> Self {
        let mut blocks = Vec::new();
        blocks.push(closure);
        Self {
            blocks
        }
    }

    pub fn get(&self, name: &String) -> Option<&Value> {
        for block in self.blocks.iter().rev() {
            match block.get(name) {
                Some(value) => return Some(value),
                None => (),
            }
        }
        None
    }

    pub fn get_mut(&mut self, name: &String) -> Option<&mut Value> {
        for block in self.blocks.iter_mut().rev() {
            match block.get_mut(name) {
                Some(value) => return Some(value),
                None => (),
            }
        }
        None
    }

    pub fn var(&mut self, name: String, value: Value) -> bool {
        if let Some(block) = self.blocks.last_mut() {
            if block.contains_key(&name) {
                false
            } else {
                block.insert(name, value);
                true
            }
        } else {
            panic!("Stack frame is empty.")
        }
    }

    pub fn push(&mut self) {
        self.blocks.push(Block::new());
    }

    pub fn pop(&mut self) -> Option<Block> {
        self.blocks.pop()
    }
}
