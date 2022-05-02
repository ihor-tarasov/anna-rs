use crate::{State, types::Value};

use super::{Expression, ExpressionResult, eval};

pub struct BlockExpression {
    stats: Vec<Expression>,
}

struct BlockGuard<'a> {
    state: &'a mut State,
}

impl<'a> BlockGuard<'a> {
    pub fn new(state: &'a mut State) -> Self {
        match state.stack_mut().frame_mut() {
            Some(frame) => frame.push(),
            None => state.global().borrow_mut().frame_mut().push(),
        }
        Self {
            state,
        }
    }

    pub fn state_mut(&mut self) -> &mut State {
        self.state
    }
}

impl<'a> Drop for BlockGuard<'a> {
    fn drop(&mut self) {
        match self.state.stack_mut().frame_mut() {
            Some(frame) => frame.pop().unwrap(),
            None => self.state.global().borrow_mut().frame_mut().pop().unwrap(),
        };
    }
}

impl BlockExpression {
    pub fn new(stats: Vec<Expression>) -> Expression {
        Expression::Block(Self { stats })
    }

    pub fn eval(&self, state: &mut State) -> ExpressionResult {
        let mut guard = BlockGuard::new(state);
        let mut result = Value::Void;
        for stat in &self.stats {
            result = eval(stat, guard.state_mut())?;
            match result.clone() {
                Value::Break => return Ok(Value::Break),
                Value::Continue => return Ok(Value::Continue),
                Value::Return => return Ok(Value::Return),
                _ => (),
            }
        }
        Ok(result)
    }
}
