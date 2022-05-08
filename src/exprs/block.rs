use crate::types::Value;

use super::{EvalArgs, Expression, ExpressionResult};

pub struct BlockExpression {
    stats: Vec<Expression>,
}

pub struct BlockGuard<'a, 'b> {
    args: &'a mut EvalArgs<'b>,
}

impl<'a, 'b> BlockGuard<'a, 'b> {
    pub fn new(args: &'a mut EvalArgs<'b>) -> Self {
        args.state.stack_mut().frame_mut().push();
        Self { args }
    }

    pub fn args_mut(&mut self) -> &mut EvalArgs<'b> {
        self.args
    }

    pub fn args(&self) -> &EvalArgs<'b> {
        self.args
    }
}

impl<'a, 'b> Drop for BlockGuard<'a, 'b> {
    fn drop(&mut self) {
        self.args.state.stack_mut().frame_mut().pop();
    }
}

impl BlockExpression {
    pub fn new(stats: Vec<Expression>) -> Expression {
        Expression::Block(Self { stats })
    }

    pub fn eval(&self, args: &mut EvalArgs) -> ExpressionResult {
        let mut guard = BlockGuard::new(args);
        let mut result = Value::Void;
        for stat in &self.stats {
            result = super::eval(stat, guard.args_mut())?;
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
