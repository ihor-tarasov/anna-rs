use std::collections::{HashMap, HashSet};

use crate::{
    exprs::{BlockExpression, ExpressionError, ExpressionErrorType, ExpressionResult},
    lexer::TokenInfo,
    types::Value,
    State,
};

pub struct Function {
    args: HashSet<String>,
    block: BlockExpression,
    info: TokenInfo,
}

struct FrameGuard<'a> {
    state: &'a mut State,
}

impl<'a> FrameGuard<'a> {
    fn new(state: &'a mut State, closure: HashMap<String, Value>) -> Self {
        state.stack_mut().push(closure);
        state.stack_mut().frame_mut().unwrap().push();
        Self { state }
    }

    fn state_mut(&mut self) -> &mut State {
        self.state
    }
}

impl<'a> Drop for FrameGuard<'a> {
    fn drop(&mut self) {
        self.state.stack_mut().pop().unwrap();
    }
}

impl Function {
    pub fn new(args: HashSet<String>, block: BlockExpression, info: TokenInfo) -> Self {
        Self { args, block, info }
    }

    pub fn call(
        &self,
        state: &mut State,
        args: Vec<Value>,
        closure: HashMap<String, Value>,
    ) -> ExpressionResult {
        let mut guard = FrameGuard::new(state, closure);

        if self.args.len() != args.len() {
            return Err(ExpressionError::new(
                ExpressionErrorType::InvalidArgumentCount,
                self.info.clone(),
            ));
        }

        for (i, name) in self.args.iter().enumerate() {
            guard
                .state_mut()
                .stack_mut()
                .frame_mut()
                .unwrap()
                .var(name.clone(), args[i].clone());
        }

        let result = self.block.eval(guard.state_mut())?;

        match result {
            Value::Break => Ok(Value::Void),
            Value::Continue => Ok(Value::Void),
            Value::Return => Ok(guard.state_mut().cache().clone()),
            _ => Ok(result),
        }
    }
}
