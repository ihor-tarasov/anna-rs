use crate::{lexer::TokenInfo, types::Value};

use super::{
    block::BlockGuard, index, result, BlockExpression, EvalArgs, Expression, ExpressionErrorType,
    ExpressionResult,
};

pub struct ForExpression {
    variable: String,
    condition: Expression,
    block: BlockExpression,
    info: TokenInfo,
}

impl ForExpression {
    pub fn new(
        variable: String,
        condition: Expression,
        block: BlockExpression,
        info: TokenInfo,
    ) -> Expression {
        Expression::For(Box::new(Self {
            variable,
            condition,
            block,
            info,
        }))
    }

    pub fn eval(&self, args: &mut EvalArgs) -> ExpressionResult {
        let mut guard = BlockGuard::new(args);

        let iterable = match super::eval(&self.condition, guard.args_mut())? {
            Value::ObjectId(id) => id,
            _ => return result::not_iterable(self.info.clone()),
        };

        let mut index = 0;

        let mut iter_value = index::get_by_index(
            guard.args().storage.lock().unwrap().get(iterable),
            index,
            &self.info,
        )?;

        guard
            .args_mut()
            .state
            .stack_mut()
            .frame_mut()
            .var(self.variable.clone(), iter_value);

        let mut result = Value::Void;
        loop {
            let value = self.block.eval(guard.args_mut())?;
            match value {
                Value::Break => {
                    result = guard.args().state.cache().clone();
                    break;
                }
                Value::Continue => (),
                Value::Return => {
                    result = Value::Return;
                    break;
                }
                _ => result = value,
            }

            index += 1;

            iter_value = match index::get_by_index(
                guard.args().storage.lock().unwrap().get(iterable),
                index,
                &self.info,
            ) {
                Ok(value) => value,
                Err(err) => match err.etype() {
                    ExpressionErrorType::IndexOutOfBounds => break,
                    _ => return Err(err),
                },
            };

            *guard
                .args_mut()
                .state
                .stack_mut()
                .frame_mut()
                .get_mut(&self.variable)
                .unwrap() = iter_value;
        }
        Ok(result)
    }
}
