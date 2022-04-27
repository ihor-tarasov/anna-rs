use super::Value;

pub enum Object {
    String(String),
    Array(Vec<Value>),
}
