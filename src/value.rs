use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub enum Value {
    String(Vec<u8>),
    Int(i64),
    List(Vec<Value>),
    Dictionary(BTreeMap<Vec<u8>, Value>),
}
