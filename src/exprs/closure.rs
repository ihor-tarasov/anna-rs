use std::collections::{HashMap, HashSet};

use crate::{types::Object, State};

use super::{Expression, ExpressionResult};

pub struct ClosureExpression {
    closure: HashSet<String>,
    id: usize,
}

impl ClosureExpression {
    pub fn new(closure: HashSet<String>, id: usize) -> Expression {
        Expression::Closure(Self { closure, id })
    }

    pub fn eval(&self, state: &mut State) -> ExpressionResult {
        let mut closure = HashMap::new();

        for name in &self.closure {
            if let Some(frame) = state.stack().frame() {
                closure.insert(name.clone(), frame.get(name).unwrap().clone());
            } else {
                closure.insert(
                    name.clone(),
                    state.global().borrow().frame().get(name).unwrap().clone(),
                );
            }
        }

        Ok(state
            .global()
            .borrow_mut()
            .storage_mut()
            .push(Object::Closure((self.id, closure))))
    }
}
