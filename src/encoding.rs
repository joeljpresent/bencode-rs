use super::value::Value;

pub(crate) fn encode(value: &Value) -> Vec<u8> {
    match value {
        Value::String(bytes) => [bytes.len().to_string().as_bytes(), &[b':'], &bytes].concat(),
        Value::Int(int) => format!("i{}e", int).as_bytes().to_vec(),
        Value::List(list) => [
            &[b'l'] as &[u8],
            &list
                .iter()
                .map(|item| encode(item))
                .flatten()
                .collect::<Vec<_>>(),
            &[b'e'],
        ]
        .concat(),
        Value::Dictionary(dict) => [
            &[b'd'] as &[u8],
            &dict
                .iter()
                .map(|(key, item)| {
                    [
                        key.len().to_string().as_bytes(),
                        &[b':'],
                        &key,
                        &encode(item),
                    ]
                    .concat()
                })
                .flatten()
                .collect::<Vec<_>>(),
            &[b'e'],
        ]
        .concat(),
    }
}
