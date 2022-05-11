use std::collections::{HashMap, HashSet};

use crate::{
    exprs::{BlockExpression, EvalArgs, ExpressionError, ExpressionErrorType, ExpressionResult},
    lexer::TokenInfo,
    types::Value,
};

pub struct Function {
    args: HashSet<String>,
    block: BlockExpression,
}

struct FrameGuard<'a, 'b> {
    args: &'a mut EvalArgs<'b>,
}

impl<'a, 'b> FrameGuard<'a, 'b> {
    fn new(args: &'a mut EvalArgs<'b>, closure: HashMap<String, Value>) -> Self {
        args.state.stack_mut().push(closure);
        args.state.stack_mut().frame_mut().push();
        Self { args }
    }

    fn args_mut(&mut self) -> &mut EvalArgs<'b> {
        self.args
    }
}

impl<'a, 'b> Drop for FrameGuard<'a, 'b> {
    fn drop(&mut self) {
        self.args.state.stack_mut().pop();
    }
}

impl Function {
    pub fn new(args: HashSet<String>, block: BlockExpression) -> Self {
        Self { args, block }
    }

    pub fn call(
        &self,
        eval_args: &mut EvalArgs,
        args: Vec<Value>,
        closure: HashMap<String, Value>,
        info: TokenInfo,
    ) -> ExpressionResult {
        let mut guard = FrameGuard::new(eval_args, closure);

        if self.args.len() != args.len() {
            return Err(ExpressionError::new(
                ExpressionErrorType::InvalidArgumentCount,
                info,
            ));
        }

        for (i, name) in self.args.iter().enumerate() {
            guard
                .args_mut()
                .state
                .stack_mut()
                .frame_mut()
                .var(name.clone(), args[i].clone());
        }

        let result = self.block.eval(guard.args_mut())?;

        match result {
            Value::Break => Ok(Value::Void),
            Value::Continue => Ok(Value::Void),
            Value::Return => Ok(guard.args_mut().state.cache().clone()),
            _ => Ok(result),
        }
    }
}
