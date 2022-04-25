use crate::State;

use super::{expression::Expression, ExpressionResult};

pub fn eval(expr: &Expression, state: &mut State) -> ExpressionResult {
    match expr {
        Expression::Literal(literal) => literal.eval(),
        Expression::Addict(addict) => addict.eval(state),
        Expression::Subtract(subtract) => subtract.eval(state),
        Expression::Multiply(multiply) => multiply.eval(state),
        Expression::Divide(divide) => divide.eval(state),
    }
}
