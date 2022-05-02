use crate::{lexer::TokenInfo, types::Value, State};

use super::{
    eval, BlockExpression, Expression, ExpressionError, ExpressionErrorType, ExpressionResult,
};

pub struct WhileExpression {
    condition: Expression,
    block: BlockExpression,
    info: TokenInfo,
}

impl WhileExpression {
    pub fn new(condition: Expression, block: BlockExpression, info: TokenInfo) -> Expression {
        Expression::While(Box::new(Self {
            condition,
            block,
            info,
        }))
    }

    pub fn eval(&self, state: &mut State) -> ExpressionResult {
        let mut result = Value::Void;
        loop {
            match eval(&self.condition, state)? {
                Value::Boolean(value) => {
                    if !value {
                        break;
                    }
                }
                _ => {
                    return Err(ExpressionError::new(
                        ExpressionErrorType::ExpectedBoolForConditions,
                        self.info.clone(),
                    ))
                }
            }

            let value = self.block.eval(state)?;
            match value {
                Value::Break => {
                    result = state.cache().clone();
                    break;
                }
                Value::Continue => (),
                Value::Return => {
                    result = Value::Return;
                    break;
                }
                _ => result = value,
            }
        }
        Ok(result)
    }
}
