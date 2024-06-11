use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Value {
    String(Vec<u8>),
    Int(i64),
    List(Vec<Value>),
    Dictionary(HashMap<Vec<u8>, Value>),
}
