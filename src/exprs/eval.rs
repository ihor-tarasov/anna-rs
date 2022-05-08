use crate::{state::StorageRc, State, functions::FunctionsRc};

use super::{expression::Expression, ExpressionResult};

pub struct EvalArgs<'a> {
    pub state: &'a mut State,
    pub storage: StorageRc,
    pub functions: FunctionsRc,
}

pub fn eval(
    expr: &Expression,
    args: &mut EvalArgs,
) -> ExpressionResult {
    match expr {
        Expression::Literal(literal) => literal.eval(),
        Expression::Addict(addict) => addict.eval(args),
        Expression::Subtract(subtract) => subtract.eval(args),
        Expression::Multiply(multiply) => multiply.eval(args),
        Expression::Divide(divide) => divide.eval(args),
        Expression::Equal(equal) => equal.eval(args),
        Expression::NotEqual(not_equal) => not_equal.eval(args),
        Expression::Less(less) => less.eval(args),
        Expression::Greater(greater) => greater.eval(args),
        Expression::LessEqual(less_equal) => less_equal.eval(args),
        Expression::GreaterEqual(greater_equal) => greater_equal.eval(args),
        Expression::UnaryNot(unary_not) => unary_not.eval(args),
        Expression::UnaryMinus(unary_minus) => unary_minus.eval(args),
        Expression::BitwiseAnd(bitwise_and) => bitwise_and.eval(args),
        Expression::BitwiseOr(bitwise_or) => bitwise_or.eval(args),
        Expression::BitwiseXor(bitwise_xor) => bitwise_xor.eval(args),
        Expression::BitwiseShl(bitwise_shl) => bitwise_shl.eval(args),
        Expression::BitwiseShr(bitwise_shr) => bitwise_shr.eval(args),
        Expression::Variable(variable) => variable.eval(args),
        Expression::Var(var) => var.eval(args),
        Expression::Assign(assign) => assign.eval(args),
        Expression::Array(array) => array.eval(args),
        Expression::Index(index) => index.eval(args),
        Expression::Call(call) => call.eval(args),
        Expression::Caching(caching) => caching.eval(args),
        Expression::Block(block) => block.eval(args),
        Expression::If(if_expr) => if_expr.eval(args),
        Expression::While(while_expr) => while_expr.eval(args),
        Expression::Closure(function) => function.eval(args),
        Expression::StringLiteral(string_literal) => string_literal.eval(args),
        Expression::For(for_expr) => for_expr.eval(args),
    }
}
