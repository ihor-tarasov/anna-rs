use std::collections::HashMap;

use super::Value;

pub enum Object {
    String(String),
    Array(Vec<Value>),
    Closure((usize, HashMap<String, Value>)),
}
