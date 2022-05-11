use crate::{
    lexer::TokenInfo,
    types::{Object, Value},
};

use super::{
    eval,
    result::{expect_indexable_object, index_out_of_bounds, invalid_index},
    EvalArgs, Expression, ExpressionResult,
};

pub struct IndexExpression {
    from: Expression,
    index: Expression,
    info: TokenInfo,
}

pub fn get_by_index(object: &Object, index: usize, info: &TokenInfo) -> ExpressionResult {
    match object {
        Object::String(string) => {
            if let Some(c) = string.chars().nth(index as usize) {
                Ok(Value::Integer(c as i64))
            } else {
                index_out_of_bounds(info.clone())
            }
        }
        Object::Array(array) => {
            if let Some(value) = array.get(index as usize) {
                Ok(value.clone())
            } else {
                index_out_of_bounds(info.clone())
            }
        }
        Object::Range(range) => {
            let pos = index as i64 + range.0;
            if pos < range.1 {
                Ok(Value::Integer(pos))
            } else {
                index_out_of_bounds(info.clone())
            }
        }
        _ => expect_indexable_object(info.clone()),
    }
}

impl IndexExpression {
    pub fn new(from: Expression, index: Expression, info: TokenInfo) -> Expression {
        Expression::Index(Box::new(Self { from, index, info }))
    }

    pub fn eval(&self, args: &mut EvalArgs) -> ExpressionResult {
        let from = eval(&self.from, args)?;
        let index = eval(&self.index, args)?;

        let guard = args.storage.lock().unwrap();

        let object = match from {
            Value::ObjectId(id) => guard.get(id),
            _ => return expect_indexable_object(self.info.clone()),
        };

        let index = match index {
            Value::Integer(index) => index,
            _ => return invalid_index(self.info.clone()),
        };

        if index < 0 {
            return invalid_index(self.info.clone());
        }

        get_by_index(object, index as usize, &self.info)
    }
}
