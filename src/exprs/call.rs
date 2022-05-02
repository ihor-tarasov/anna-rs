use crate::{
    lexer::TokenInfo,
    types::{Object, Value},
    State,
};

use super::{eval, Expression, ExpressionError, ExpressionErrorType, ExpressionResult};

pub struct CallExpression {
    from: Expression,
    params: Vec<Expression>,
    info: TokenInfo,
}

fn not_callable_object(info: TokenInfo) -> ExpressionResult {
    Err(ExpressionError::new(
        ExpressionErrorType::NotCallableObject,
        info,
    ))
}

impl CallExpression {
    pub fn new(from: Expression, params: Vec<Expression>, info: TokenInfo) -> Expression {
        Expression::Call(Box::new(Self { from, params, info }))
    }

    pub fn eval(&self, state: &mut State) -> ExpressionResult {
        let from = eval(&self.from, state)?;

        let mut params = Vec::new();
        params.reserve(self.params.len());
        for param in &self.params {
            params.push(eval(param, state)?);
        }

        match from {
            Value::NativeFunctionId(id) => Ok((state.global().borrow().native(id))(state, params)),
            Value::ObjectId(id) => match state.global().borrow().storage().get(id) {
                Object::Closure((id, closure)) => {
                    state
                        .global()
                        .borrow()
                        .function(*id)
                        .call(state, params, closure.clone())
                }
                _ => not_callable_object(self.info.clone()),
            },
            _ => not_callable_object(self.info.clone()),
        }
    }
}
