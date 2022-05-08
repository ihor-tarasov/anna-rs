use std::{collections::HashMap, thread::JoinHandle};

use super::Value;

pub enum Object {
    String(String),
    Array(Vec<Value>),
    Closure((usize, HashMap<String, Value>)),
    Thread(Option<JoinHandle<Value>>),
}
