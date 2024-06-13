use core::fmt;
use std::collections::BTreeMap;

#[derive(Clone)]
pub enum Value {
    String(Vec<u8>),
    Int(i64),
    List(Vec<Value>),
    Dictionary(BTreeMap<Vec<u8>, Value>),
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(bytes) => write!(f, "{:?}", String::from_utf8_lossy(bytes)),
            Self::Int(int) => write!(f, "{}", int),
            Self::List(list) => write!(f, "{:?}", list),
            Self::Dictionary(dict) => write!(
                f,
                "{:?}",
                dict.iter()
                    .map(|(key, val)| (String::from_utf8_lossy(key), val))
                    .collect::<BTreeMap<_, _>>()
            ),
        }
    }
}
