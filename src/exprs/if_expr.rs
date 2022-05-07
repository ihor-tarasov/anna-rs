use crate::{lexer::TokenInfo, types::Value};

use super::{
    BlockExpression, EvalArgs, Expression, ExpressionError, ExpressionErrorType, ExpressionResult,
};

pub struct IfExpression {
    conditions: Vec<Expression>,
    blocks: Vec<BlockExpression>,
    else_block: Option<BlockExpression>,
    info: Vec<TokenInfo>,
}

impl IfExpression {
    pub fn new(
        conditions: Vec<Expression>,
        blocks: Vec<BlockExpression>,
        else_block: Option<BlockExpression>,
        info: Vec<TokenInfo>,
    ) -> Expression {
        Expression::If(Self {
            conditions,
            blocks,
            else_block,
            info,
        })
    }

    pub fn eval(&self, args: &mut EvalArgs) -> ExpressionResult {
        let mut result = Value::Void;
        for (i, expr) in self.conditions.iter().enumerate() {
            match super::eval(expr, args)? {
                Value::Boolean(value) => {
                    if !value {
                        continue;
                    }
                }
                _ => {
                    return Err(ExpressionError::new(
                        ExpressionErrorType::ExpectedBoolForConditions,
                        self.info.get(i).unwrap().clone(),
                    ))
                }
            }
            result = self.blocks.get(i).unwrap().eval(args)?;
        }
        if let Some(else_block) = &self.else_block {
            result = else_block.eval(args)?;
        }
        Ok(result)
    }
}
