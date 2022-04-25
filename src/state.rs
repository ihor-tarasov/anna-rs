use std::{rc::Rc, cell::RefCell};

use crate::{Stack, global_state::GlobalState};

pub struct State {
    stack: Stack,
    global: Rc<RefCell<GlobalState>>,
}

impl State {
    pub fn new() -> Self {
        Self {
            stack: Stack::new(),
            global: Rc::new(RefCell::new(GlobalState::new())),
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
}
