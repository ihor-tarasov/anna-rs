mod result;

pub use result::ExpressionError;
pub use result::ExpressionErrorType;
pub use result::ExpressionResult;

mod binary;
mod expression;
mod literal;
mod eval;

pub use expression::Expression;
pub use expression::ExpressionBox;

pub use literal::LiteralExpression;
pub use binary::BinaryExpression;

pub use eval::eval;
