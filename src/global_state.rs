use crate::{
    types::{Storage, Value},
    StackFrame, State, Function,
};

pub type NativeFunctionCallback = dyn Fn(&mut State, Vec<Value>) -> Value;
pub type NativeFunctionCallbackBox = Box<NativeFunctionCallback>;

pub struct GlobalState {
    frame: StackFrame,
    storage: Storage,
    natives: Vec<NativeFunctionCallbackBox>,
    functions: Vec<Function>,
}

impl GlobalState {
    pub fn new() -> Self {
        let mut frame = StackFrame::new();
        frame.push();
        Self {
            frame,
            storage: Storage::new(),
            natives: Vec::new(),
            functions: Vec::new(),
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

    pub fn push_native(&mut self, callback: NativeFunctionCallbackBox) -> usize {
        self.natives.push(callback);
        self.natives.len() - 1
    }

    pub fn native(&self, id: usize) -> &NativeFunctionCallback {
        self.natives.get(id).expect("Native Function not exist")
    }

    pub fn push_function(&mut self, function: Function) -> usize {
        self.functions.push(function);
        self.functions.len() - 1
    }

    pub fn function(&self, id: usize) -> &Function {
        self.functions.get(id).expect("Function not exist")
    }
}
