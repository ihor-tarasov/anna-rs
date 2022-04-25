use crate::StackFrame;

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

    pub fn push(&mut self) {
        self.frames.push(StackFrame::new());
    }

    pub fn pop(&mut self) -> Option<StackFrame> {
        self.frames.pop()
    }
}
