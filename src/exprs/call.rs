use std::{collections::HashMap, sync::Arc};

use crate::{
    functions::FunctionsRc,
    lexer::TokenInfo,
    state::StorageRc,
    types::{Object, Value},
    State,
};

use super::{EvalArgs, Expression, ExpressionError, ExpressionErrorType, ExpressionResult};

pub struct CallExpression {
    from: Expression,
    params: Vec<Expression>,
    is_async: bool,
    info: TokenInfo,
}

fn call_native_sync(
    functions: FunctionsRc,
    storage: StorageRc,
    id: usize,
    params: Vec<Value>,
) -> Value {
    functions.native(id)(storage, params)
}

fn call_native(args: &EvalArgs, id: usize, params: Vec<Value>, is_async: bool) -> Value {
    if is_async {
        let jh = std::thread::spawn({
            let functions = Arc::clone(&args.functions);
            let storage = Arc::clone(&args.storage);
            move || call_native_sync(functions, storage, id, params)
        });

        args.storage.lock().unwrap().push(Object::Thread(Some(jh)))
    } else {
        call_native_sync(
            Arc::clone(&args.functions),
            Arc::clone(&args.storage),
            id,
            params,
        )
    }
}

fn call_closure(
    args: &mut EvalArgs,
    id: usize,
    params: Vec<Value>,
    closure: HashMap<String, Value>,
    is_async: bool,
) -> ExpressionResult {
    if is_async {
        let jh = std::thread::spawn({
            let storage = Arc::clone(&args.storage);
            let functions = Arc::clone(&args.functions);
            move || {
                let mut state = State::new();
                let functions2 = Arc::clone(&functions);
                let mut eval_args = EvalArgs {
                    state: &mut state,
                    storage,
                    functions,
                };

                functions2
                    .function(id)
                    .call(&mut eval_args, params, closure)
                    .unwrap()
            }
        });

        Ok(args.storage.lock().unwrap().push(Object::Thread(Some(jh))))
    } else {
        let functions = Arc::clone(&args.functions);
        functions.function(id).call(args, params, closure)
    }
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

        match from {
            Value::NativeFunctionId(id) => Ok(call_native(args, id, params, self.is_async)),
            Value::ObjectId(id) => {
                let closure = match args.storage.lock().unwrap().get(id) {
                    Object::Closure((id, closure)) => (*id, closure.clone()),
                    _ => return not_callable_object(self.info.clone()),
                };

                call_closure(args, closure.0, params, closure.1, self.is_async)
            }
            _ => not_callable_object(self.info.clone()),
        }
    }
}
