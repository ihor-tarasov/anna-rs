use crate::State;

use super::{expression::Expression, ExpressionResult};

pub fn eval(expr: &Expression, state: &mut State) -> ExpressionResult {
    match expr {
        Expression::Literal(literal) => literal.eval(),
        Expression::Addict(addict) => addict.eval(state),
        Expression::Subtract(subtract) => subtract.eval(state),
        Expression::Multiply(multiply) => multiply.eval(state),
        Expression::Divide(divide) => divide.eval(state),
        Expression::Equal(equal) => equal.eval(state),
        Expression::NotEqual(not_equal) => not_equal.eval(state),
        Expression::Less(less) => less.eval(state),
        Expression::Greater(greater) => greater.eval(state),
        Expression::LessEqual(less_equal) => less_equal.eval(state),
        Expression::GreaterEqual(greater_equal) => greater_equal.eval(state),
        Expression::UnaryNot(unary_not) => unary_not.eval(state),
        Expression::UnaryMinus(unary_minus) => unary_minus.eval(state),        
    }
}
