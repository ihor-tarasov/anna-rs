use crate::StackFrame;

pub struct GlobalState {
    frame: StackFrame,
}

impl GlobalState {
    pub fn new() -> Self {
        let mut frame = StackFrame::new();
        frame.push();
        Self { frame }
    }

    pub fn frame(&self) -> &StackFrame {
        &self.frame
    }

    pub fn frame_mut(&mut self) -> &mut StackFrame {
        &mut self.frame
    }
}
