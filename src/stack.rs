use std::collections::HashMap;

use crate::{StackFrame, types::Value};

pub struct Stack {
    frames: Vec<StackFrame>,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            frames: Vec::new(),
        }
    }

    pub fn frame(&self) -> Option<&StackFrame> {
        self.frames.last()
    }

    pub fn frame_mut(&mut self) -> Option<&mut StackFrame> {
        self.frames.last_mut()
    }

    pub fn push(&mut self, closure: HashMap<String, Value>) {
        self.frames.push(StackFrame::with_closure(closure));
    }

    pub fn pop(&mut self) -> Option<StackFrame> {
        self.frames.pop()
    }
}
