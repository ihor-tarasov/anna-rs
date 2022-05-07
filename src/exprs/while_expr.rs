use crate::{lexer::TokenInfo, types::Value};

use super::{
    BlockExpression, EvalArgs, Expression, ExpressionError, ExpressionErrorType, ExpressionResult,
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

    pub fn eval(&self, args: &mut EvalArgs) -> ExpressionResult {
        let mut result = Value::Void;
        loop {
            match super::eval(&self.condition, args)? {
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

            let value = self.block.eval(args)?;
            match value {
                Value::Break => {
                    result = args.state.cache().clone();
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
