use crate::{
    lexer::TokenInfo,
    types::{Object, Value},
    State,
};

use super::{
    eval,
    result::{expect_indexable_object, index_out_of_bounds, invalid_index},
    Expression, ExpressionResult,
};

pub struct IndexExpression {
    from: Expression,
    index: Expression,
    info: TokenInfo,
}

impl IndexExpression {
    pub fn new(from: Expression, index: Expression, info: TokenInfo) -> Expression {
        Expression::Index(Box::new(Self { from, index, info }))
    }

    pub fn eval(&self, state: &mut State) -> ExpressionResult {
        let from = eval(&self.from, state)?;
        let index = eval(&self.index, state)?;

        let global = state.global();
        let global_b = global.borrow();

        let object = match from {
            Value::ObjectId(id) => global_b.storage().get(id),
            _ => return expect_indexable_object(self.info.clone()),
        };

        let index = match index {
            Value::Integer(index) => index,
            _ => return invalid_index(self.info.clone()),
        };

        if index < 0 {
            return invalid_index(self.info.clone());
        }

        match object {
            Object::String(string) => {
                if let Some(c) = string.chars().nth(index as usize) {
                    Ok(Value::Integer(c as i64))
                } else {
                    index_out_of_bounds(self.info.clone())
                }
            }
            Object::Array(array) => {
                if let Some(value) = array.get(index as usize) {
                    Ok(value.clone())
                } else {
                    index_out_of_bounds(self.info.clone())
                }
            }
            _ => expect_indexable_object(self.info.clone()),
        }
    }
}
