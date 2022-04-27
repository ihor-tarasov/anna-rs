use std::{cell::RefCell, rc::Rc};

use crate::{global_state::GlobalState, types::Value, Stack};

pub struct State {
    stack: Stack,
    global: Rc<RefCell<GlobalState>>,
    cache: Value,
}

impl State {
    pub fn new() -> Self {
        Self {
            stack: Stack::new(),
            global: Rc::new(RefCell::new(GlobalState::new())),
            cache: Value::Void,
        }
    }

    pub fn stack(&self) -> &Stack {
        &self.stack
    }

    pub fn stack_mut(&mut self) -> &mut Stack {
        &mut self.stack
    }

    pub fn global(&self) -> Rc<RefCell<GlobalState>> {
        self.global.clone()
    }

    pub fn native<F>(&mut self, name: String, f: F) -> bool
    where
        F: Fn(&mut State, Vec<Value>) -> Value + 'static,
    {
        let value = Value::NativeFunctionId(self.global.borrow_mut().push_native(Box::new(f)));
        self.global().borrow_mut().frame_mut().var(name, value)
    }

    pub fn cache(&self) -> &Value { &self.cache }
    pub fn cache_mut(&mut self) -> &mut Value { &mut self.cache }
}
