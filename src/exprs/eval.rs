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
        Expression::BitwiseAnd(bitwise_and) => bitwise_and.eval(state),
        Expression::BitwiseOr(bitwise_or) => bitwise_or.eval(state),
        Expression::BitwiseXor(bitwise_xor) => bitwise_xor.eval(state),
        Expression::BitwiseShl(bitwise_shl) => bitwise_shl.eval(state),
        Expression::BitwiseShr(bitwise_shr) => bitwise_shr.eval(state),
        Expression::Variable(variable) => variable.eval(state),
        Expression::Var(var) => var.eval(state),
        Expression::Assign(assign) => assign.eval(state),
        Expression::Array(array) => array.eval(state),
        Expression::Index(index) => index.eval(state),
        Expression::Call(call) => call.eval(state),
    }
}
