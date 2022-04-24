use crate::opers::arithmetic::{ArithmeticOperator, AddictArithmetic, MultiplyArithmetic};

use super::{literal::LiteralExpression, BinaryExpression};

pub enum Expression {
    Literal(LiteralExpression),
    Addict(Box<BinaryExpression<ArithmeticOperator<AddictArithmetic>>>),
    Multiply(Box<BinaryExpression<ArithmeticOperator<MultiplyArithmetic>>>),
}

pub type ExpressionBox = Box<Expression>;
