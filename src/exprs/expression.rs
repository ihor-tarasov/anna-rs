use crate::opers::{
    arithmetic::{
        AddictArithmetic, ArithmeticOperator, DivideArithmetic, MultiplyArithmetic,
        SubtractArithmetic,
    },
    comparison::{
        ComparisonOperator, EqualComparison, GreaterComparison, GreaterEqualComparison,
        LessComparison, LessEqualComparison,
    }, unary::{UnaryNotOperator, UnaryMinusOperator},
};

use super::{literal::LiteralExpression, BinaryExpression, UnaryExpression};

pub enum Expression {
    Literal(LiteralExpression),
    Addict(Box<BinaryExpression<ArithmeticOperator<AddictArithmetic>>>),
    Subtract(Box<BinaryExpression<ArithmeticOperator<SubtractArithmetic>>>),
    Multiply(Box<BinaryExpression<ArithmeticOperator<MultiplyArithmetic>>>),
    Divide(Box<BinaryExpression<ArithmeticOperator<DivideArithmetic>>>),
    Equal(Box<BinaryExpression<ComparisonOperator<EqualComparison>>>),
    NotEqual(Box<BinaryExpression<ComparisonOperator<EqualComparison>>>),
    Less(Box<BinaryExpression<ComparisonOperator<LessComparison>>>),
    Greater(Box<BinaryExpression<ComparisonOperator<GreaterComparison>>>),
    LessEqual(Box<BinaryExpression<ComparisonOperator<LessEqualComparison>>>),
    GreaterEqual(Box<BinaryExpression<ComparisonOperator<GreaterEqualComparison>>>),
    UnaryNot(Box<UnaryExpression<UnaryNotOperator>>),
    UnaryMinus(Box<UnaryExpression<UnaryMinusOperator>>),
}

pub type ExpressionBox = Box<Expression>;
