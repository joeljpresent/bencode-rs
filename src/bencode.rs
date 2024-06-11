use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Value {
    String(Vec<u8>),
    Int(i64),
    List(Vec<Value>),
    Dictionary(HashMap<String, Value>),
}

pub fn get_next_value(bytes: &[u8]) -> Result<(Value, &[u8]), String> {
    match bytes.get(0) {
        Some(b'0'..=b'9') => {
            let mut it = bytes.splitn(2, |b| *b == b':');
            let length_bytes = it.next().ok_or("Missing colon")?;
            let rest = it.next().unwrap_or(&[]);
            println!(
                "\u{1f234} {} // {}",
                String::from_utf8_lossy(length_bytes),
                String::from_utf8_lossy(rest)
            );
            let length_str =
                std::str::from_utf8(length_bytes).or(Err("Incorrect length string".to_owned()))?;
            let length: usize = length_str
                .parse()
                .or(Err("Invalid length number".to_owned()))?;
            if length > rest.len() {
                return Err("Length is bigger than the rest of the input".to_owned());
            }
            let (byte_string, rest) = rest.split_at(length);
            println!(
                "\u{1f235} {} || {}",
                String::from_utf8_lossy(byte_string),
                String::from_utf8_lossy(rest)
            );
            Ok((Value::String(byte_string.to_owned()), rest))
        }
        Some(b'i') => {
            let mut it = bytes[1..].splitn(2, |b| *b == b'e');
            let int_bytes = it.next().ok_or("Missing 'e'")?;
            let rest = it.next().unwrap_or(&[]);
            println!(
                "\u{1f234} {} // {}",
                String::from_utf8_lossy(int_bytes),
                String::from_utf8_lossy(rest)
            );
            let int_str =
                std::str::from_utf8(int_bytes).or(Err("Incorrect int string".to_owned()))?;
            let int: i64 = int_str.parse().or(Err("Invalid int number".to_owned()))?;
            Ok((Value::Int(int), rest))
        }
        _ => Err("NON IMPLÉMENTÉ".to_owned()),
    }
}
