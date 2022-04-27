mod result;

pub use result::ExpressionError;
pub use result::ExpressionErrorType;
pub use result::ExpressionResult;

mod binary;
mod eval;
mod expression;
mod literal;
mod unary;
mod variable;
mod var;
mod assign;
mod array;

pub use expression::Expression;
pub use expression::ExpressionBox;

pub use binary::BinaryExpression;
pub use literal::LiteralExpression;
pub use unary::UnaryExpression;
pub use variable::VariableExpression;
pub use var::VarExpression;
pub use assign::AssignExpression;
pub use array::ArrayExpression;

pub use eval::eval;
