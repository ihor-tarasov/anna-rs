#[derive(Debug, Clone)]
pub enum Value {
    Boolean(bool),
    Integer(i64),
    Real(f64),
    NativeFunctionId(usize),
    ObjectId(usize),
}
