use crate::opers::{
    arithmetic::{
        AddictArithmetic, ArithmeticOperator, DivideArithmetic, MultiplyArithmetic,
        SubtractArithmetic,
    },
    bitwise::{AndBitwise, BitwiseOperator, OrBitwise, ShlBitwise, ShrBitwise, XorBitwise},
    comparison::{
        ComparisonOperator, EqualComparison, GreaterComparison, GreaterEqualComparison,
        LessComparison, LessEqualComparison, NotEqualComparison,
    },
    unary::{UnaryMinusOperator, UnaryNotOperator},
};

use super::{
    assign::AssignExpression, block::BlockExpression, index::IndexExpression,
    literal::{LiteralExpression, StringLiteralExpression}, var::VarExpression, variable::VariableExpression, ArrayExpression,
    BinaryExpression, CachingExpression, CallExpression, ClosureExpression, IfExpression,
    UnaryExpression, WhileExpression, ForExpression,
};

pub enum Expression {
    Literal(LiteralExpression),
    Addict(Box<BinaryExpression<ArithmeticOperator<AddictArithmetic>>>),
    Subtract(Box<BinaryExpression<ArithmeticOperator<SubtractArithmetic>>>),
    Multiply(Box<BinaryExpression<ArithmeticOperator<MultiplyArithmetic>>>),
    Divide(Box<BinaryExpression<ArithmeticOperator<DivideArithmetic>>>),
    Equal(Box<BinaryExpression<ComparisonOperator<EqualComparison>>>),
    NotEqual(Box<BinaryExpression<ComparisonOperator<NotEqualComparison>>>),
    Less(Box<BinaryExpression<ComparisonOperator<LessComparison>>>),
    Greater(Box<BinaryExpression<ComparisonOperator<GreaterComparison>>>),
    LessEqual(Box<BinaryExpression<ComparisonOperator<LessEqualComparison>>>),
    GreaterEqual(Box<BinaryExpression<ComparisonOperator<GreaterEqualComparison>>>),
    UnaryNot(Box<UnaryExpression<UnaryNotOperator>>),
    UnaryMinus(Box<UnaryExpression<UnaryMinusOperator>>),
    BitwiseAnd(Box<BinaryExpression<BitwiseOperator<AndBitwise>>>),
    BitwiseOr(Box<BinaryExpression<BitwiseOperator<OrBitwise>>>),
    BitwiseXor(Box<BinaryExpression<BitwiseOperator<XorBitwise>>>),
    BitwiseShl(Box<BinaryExpression<BitwiseOperator<ShlBitwise>>>),
    BitwiseShr(Box<BinaryExpression<BitwiseOperator<ShrBitwise>>>),
    Variable(VariableExpression),
    Var(Box<VarExpression>),
    Assign(Box<AssignExpression>),
    Array(ArrayExpression),
    Index(Box<IndexExpression>),
    Call(Box<CallExpression>),
    Caching(Box<CachingExpression>),
    Block(BlockExpression),
    If(IfExpression),
    While(Box<WhileExpression>),
    Closure(ClosureExpression),
    StringLiteral(StringLiteralExpression),
    For(Box<ForExpression>),
}
