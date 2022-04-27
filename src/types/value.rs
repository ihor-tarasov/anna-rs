#[derive(Debug, Clone)]
pub enum Value {
    Void,
    Break,
    Continue,
    Return,
    Boolean(bool),
    Integer(i64),
    Real(f64),
    FunctionId(usize),
    NativeFunctionId(usize),
    ObjectId(usize),
}
