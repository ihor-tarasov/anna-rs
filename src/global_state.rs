use crate::{StackFrame, types::Storage};

pub struct GlobalState {
    frame: StackFrame,
    storage: Storage,
}

impl GlobalState {
    pub fn new() -> Self {
        let mut frame = StackFrame::new();
        frame.push();
        Self {
            frame,
            storage: Storage::new(),
        }
    }

    pub fn frame(&self) -> &StackFrame {
        &self.frame
    }

    pub fn frame_mut(&mut self) -> &mut StackFrame {
        &mut self.frame
    }

    pub fn storage(&self) -> &Storage {
        &self.storage
    }

    pub fn storage_mut(&mut self) -> &mut Storage {
        &mut self.storage
    }
}
