use std::sync::Arc;

use crate::{types::Value, Function, State};

pub type NativeFunctionCallback = dyn Fn(Vec<Value>) -> Value + Send + Sync;
pub type NativeFunctionCallbackBox = Box<NativeFunctionCallback>;

pub struct Functions {
    natives: Vec<NativeFunctionCallbackBox>,
    functions: Vec<Function>,
}

pub type FunctionsRc = Arc<Functions>;

pub fn native<F>(state: &mut State, functions: &mut Functions, name: String, f: F) -> bool
where
    F: Fn(Vec<Value>) -> Value + 'static + Send + Sync,
{
    let value = Value::NativeFunctionId(functions.push_native(Box::new(f)));
    state.stack_mut().frame_mut().var(name, value)
}

impl Functions {
    pub fn new() -> Self {
        Self {
            natives: Vec::new(),
            functions: Vec::new(),
        }
    }

    pub fn push_native(&mut self, callback: NativeFunctionCallbackBox) -> usize {
        self.natives.push(callback);
        self.natives.len() - 1
    }

    pub fn native(&self, id: usize) -> &NativeFunctionCallback {
        self.natives.get(id).expect("Native Function not exist")
    }

    pub fn push(&mut self, function: Function) -> usize {
        self.functions.push(function);
        self.functions.len() - 1
    }

    pub fn function(&self, id: usize) -> &Function {
        self.functions.get(id).expect("Function not exist")
    }
}
