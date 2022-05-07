use std::{sync::{Arc, Mutex}};

use crate::{types::{Value, Storage}, Stack};

pub type StorageRc = Arc<Mutex<Storage>>;

pub struct State {
    stack: Stack,
    cache: Value,
}

impl State {
    pub fn new() -> Self {
        Self {
            stack: Stack::new(),
            cache: Value::Void,
        }
    }

    pub fn stack(&self) -> &Stack {
        &self.stack
    }

    pub fn stack_mut(&mut self) -> &mut Stack {
        &mut self.stack
    }

    pub fn cache(&self) -> &Value { &self.cache }
    pub fn cache_mut(&mut self) -> &mut Value { &mut self.cache }
}
