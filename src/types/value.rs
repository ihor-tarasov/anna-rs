#[derive(Debug, Clone)]
pub enum Value {
    Void,
    Break,
    Continue,
    Return,
    Boolean(bool),
    Integer(i64),
    Real(f64),
    NativeFunctionId(usize),
    ObjectId(usize),
}
