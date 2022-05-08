use std::{collections::HashMap, thread::JoinHandle};

use super::Value;

pub enum Object {
    String(String),
    Array(Vec<Value>),
    Closure((usize, HashMap<String, Value>)),
    Range((i64, i64)),
    Thread(Option<JoinHandle<Value>>),
}
