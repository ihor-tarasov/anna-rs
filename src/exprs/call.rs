use std::{collections::HashMap, sync::Arc};

use crate::{
    lexer::TokenInfo,
    types::{Object, Value},
};

use super::{EvalArgs, Expression, ExpressionError, ExpressionErrorType, ExpressionResult};

pub struct CallExpression {
    from: Expression,
    params: Vec<Expression>,
    info: TokenInfo,
}

fn call_closure(
    args: &mut EvalArgs,
    id: usize,
    params: Vec<Value>,
    closure: HashMap<String, Value>,
) -> ExpressionResult {
    let functions = Arc::clone(&args.functions);
    functions.function(id).call(args, params, closure)
}

fn not_callable_object(info: TokenInfo) -> ExpressionResult {
    Err(ExpressionError::new(
        ExpressionErrorType::NotCallableObject,
        info,
    ))
}

impl CallExpression {
    pub fn new(
        from: Expression,
        params: Vec<Expression>,
        info: TokenInfo,
    ) -> Expression {
        Expression::Call(Box::new(Self {
            from,
            params,
            info,
        }))
    }

    pub fn eval(&self, args: &mut EvalArgs) -> ExpressionResult {
        let from = super::eval(&self.from, args)?;

        let mut params = Vec::new();
        params.reserve(self.params.len());
        for param in &self.params {
            params.push(super::eval(param, args)?);
        }

        match from {
            Value::NativeFunctionId(id) => Ok(args.functions.native(id)(Arc::clone(&args.storage), params)),
            Value::ObjectId(id) => {
                let closure = match args.storage.lock().unwrap().get(id) {
                    Object::Closure((id, closure)) => (*id, closure.clone()),
                    _ => return not_callable_object(self.info.clone()),
                };

                call_closure(args, closure.0, params, closure.1)
            }
            _ => not_callable_object(self.info.clone()),
        }
    }
}
