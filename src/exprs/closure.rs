use std::collections::{HashMap, HashSet};

use crate::types::Object;

use super::{EvalArgs, Expression, ExpressionResult};

pub struct ClosureExpression {
    closure: HashSet<String>,
    id: usize,
}

impl ClosureExpression {
    pub fn new(closure: HashSet<String>, id: usize) -> Expression {
        Expression::Closure(Self { closure, id })
    }

    pub fn eval(&self, args: &EvalArgs) -> ExpressionResult {
        let mut closure = HashMap::new();

        for name in &self.closure {
            closure.insert(
                name.clone(),
                args.state.stack().frame().get(name).unwrap().clone(),
            );
        }

        Ok(args
            .storage
            .lock()
            .unwrap()
            .push(Object::Closure((self.id, closure))))
    }
}
