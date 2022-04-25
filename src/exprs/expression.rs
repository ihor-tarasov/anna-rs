use crate::opers::arithmetic::{ArithmeticOperator, AddictArithmetic, MultiplyArithmetic, SubtractArithmetic, DivideArithmetic};

use super::{literal::LiteralExpression, BinaryExpression};

pub enum Expression {
    Literal(LiteralExpression),
    Addict(Box<BinaryExpression<ArithmeticOperator<AddictArithmetic>>>),
    Subtract(Box<BinaryExpression<ArithmeticOperator<SubtractArithmetic>>>),
    Multiply(Box<BinaryExpression<ArithmeticOperator<MultiplyArithmetic>>>),
    Divide(Box<BinaryExpression<ArithmeticOperator<DivideArithmetic>>>),
}

pub type ExpressionBox = Box<Expression>;
