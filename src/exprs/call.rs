use core::panic;
use std::{collections::HashMap, sync::Arc};

use crate::{
    lexer::TokenInfo,
    types::{Object, Value}, State,
};

use super::{EvalArgs, Expression, ExpressionError, ExpressionErrorType, ExpressionResult};

pub struct CallExpression {
    from: Expression,
    params: Vec<Expression>,
    is_async: bool,
    info: TokenInfo,
}

fn call_native(args: &EvalArgs, id: usize, params: Vec<Value>, is_async: bool) -> ExpressionResult {
    if is_async {
        let jh = std::thread::spawn({
            let functions = Arc::clone(&args.functions);
            let storage = Arc::clone(&args.storage);
            move || {
            functions.native(id)(storage, params)
        }});
        Ok(args.storage.lock().unwrap().push(Object::Thread(Some(jh))))
    } else {
        Ok(args.functions.native(id)(Arc::clone(&args.storage), params))
    }
}

fn call_closure(
    args: &mut EvalArgs,
    id: usize,
    params: Vec<Value>,
    closure: HashMap<String, Value>,
    is_async: bool,
    info: TokenInfo,
) -> ExpressionResult {
    let functions = Arc::clone(&args.functions);
    if is_async {
        let storage = Arc::clone(&args.storage);
        let jh = std::thread::spawn(move || {
            let mut state = State::new();
            let functions2 = Arc::clone(&functions);
            let mut eval_args = EvalArgs {
                state: &mut state,
                storage,
                functions,
            };
            match functions2.function(id).call(&mut eval_args, params, closure, info) {
                Ok(value) => value,
                Err(_) => Value::Void,
            }
        });
        Ok(args.storage.lock().unwrap().push(Object::Thread(Some(jh))))
    } else {
        functions.function(id).call(args, params, closure, info)
    }
}

fn not_callable_object(info: TokenInfo) -> ExpressionResult {
    Err(ExpressionError::new(
        ExpressionErrorType::NotCallableObject,
        info,
    ))
}

pub fn call(
    from: Value,
    params: Vec<Value>,
    args: &mut EvalArgs,
    is_async: bool,
    info: TokenInfo,
) -> ExpressionResult {
    match from {
        Value::NativeFunctionId(id) => {
            call_native(args, id, params, is_async)
        }
        Value::ObjectId(id) => {
            let object = match args.storage.lock().unwrap().get_mut(id) {
                Object::Closure((id, closure)) => Object::Closure((*id, closure.clone())),
                Object::Thread(jh) => {
                    match jh.take() {
                        Some(jh) => Object::Thread(Some(jh)),
                        None => return Ok(Value::Void),
                    }
                }
                _ => return not_callable_object(info.clone()),
            };

            match object {
                Object::Closure(closure) => {
                    call_closure(args, closure.0, params, closure.1, is_async, info)
                },
                Object::Thread(jh) => Ok(jh.unwrap().join().unwrap()),
                _ => panic!("Not callable"),
            }
        }
        _ => not_callable_object(info),
    }
}

impl CallExpression {
    pub fn new(
        from: Expression,
        params: Vec<Expression>,
        is_async: bool,
        info: TokenInfo,
    ) -> Expression {
        Expression::Call(Box::new(Self {
            from,
            params,
            is_async,
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

        call(from, params, args, self.is_async, self.info.clone())
    }
}
