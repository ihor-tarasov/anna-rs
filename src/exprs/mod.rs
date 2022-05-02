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
mod index;
mod call;
mod caching;
mod block;
mod if_expr;
mod while_expr;
mod closure;

pub use expression::Expression;

pub use binary::BinaryExpression;
pub use literal::LiteralExpression;
pub use unary::UnaryExpression;
pub use variable::VariableExpression;
pub use var::VarExpression;
pub use assign::AssignExpression;
pub use array::ArrayExpression;
pub use index::IndexExpression;
pub use call::CallExpression;
pub use caching::CachingExpression;
pub use block::BlockExpression;
pub use if_expr::IfExpression;
pub use while_expr::WhileExpression;
pub use closure::ClosureExpression;

pub use eval::eval;
