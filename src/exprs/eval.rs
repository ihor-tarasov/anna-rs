use super::{expression::Expression, ExpressionResult};

pub fn eval(expr: &Expression) -> ExpressionResult {
    match expr {
        Expression::Literal(literal) => literal.eval(),
        Expression::Addict(addict) => addict.eval(),
        Expression::Multiply(multiply) => multiply.eval(),
    }
}
