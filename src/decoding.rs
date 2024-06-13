use std::collections::BTreeMap;

use super::value::Value;

pub(crate) fn get_next_value(bytes: &[u8]) -> Result<(Value, &[u8]), String> {
    match bytes.get(0) {
        Some(b'0'..=b'9') => {
            let mut it = bytes.splitn(2, |b| *b == b':');
            let length_bytes = it.next().ok_or("Missing colon")?;
            let rest = it.next().unwrap_or(&[]);
            let length_str =
                std::str::from_utf8(length_bytes).or(Err("Incorrect length string".to_owned()))?;
            let length: usize = length_str
                .parse()
                .or(Err("Invalid length number".to_owned()))?;
            if length > rest.len() {
                return Err("Length is bigger than the rest of the input".to_owned());
            }
            let (byte_string, rest) = rest.split_at(length);
            Ok((Value::String(byte_string.to_owned()), rest))
        }
        Some(b'i') => {
            let mut it = bytes[1..].splitn(2, |b| *b == b'e');
            let int_bytes = it.next().ok_or("Missing int-ending 'e'")?;
            let rest = it.next().unwrap_or(&[]);
            let int_str =
                std::str::from_utf8(int_bytes).or(Err("Incorrect int string".to_owned()))?;
            let int: i64 = int_str.parse().or(Err("Invalid int number".to_owned()))?;
            Ok((Value::Int(int), rest))
        }
        Some(b'l') => match parse_list(&bytes[1..]) {
            Ok((list, rest)) => Ok((Value::List(list), rest)),
            Err(error) => Err(error),
        },
        Some(b'd') => match parse_dictionary(&bytes[1..]) {
            Ok((dict, rest)) => Ok((Value::Dictionary(dict), rest)),
            Err(error) => Err(error),
        },
        _ => Err("Unknown value".to_owned()),
    }
}

fn parse_list(bytes: &[u8]) -> Result<(Vec<Value>, &[u8]), String> {
    let mut list = Vec::<Value>::new();
    let mut current = bytes;
    loop {
        match current.get(0) {
            Some(b'e') => return Ok((list, &current[1..])),
            Some(_) => match get_next_value(current) {
                Ok((val, rest)) => {
                    list.push(val);
                    current = rest;
                }
                Err(error) => return Err(error),
            },
            None => return Err("Missing list-ending e".to_owned()),
        }
    }
}

fn parse_dictionary(bytes: &[u8]) -> Result<(BTreeMap<Vec<u8>, Value>, &[u8]), String> {
    let mut dict = BTreeMap::<Vec<u8>, Value>::new();
    let mut current = bytes;
    loop {
        match current.get(0) {
            Some(b'e') => return Ok((dict, &current[1..])),
            Some(b'0'..=b'9') => match get_next_value(current) {
                Err(error) => return Err(error),
                Ok((Value::String(key), rest)) => match get_next_value(rest) {
                    Ok((val, rest)) => {
                        dict.insert(key, val);
                        current = rest;
                    }
                    Err(error) => return Err(error),
                },
                _ => return Err("Invalid key".to_owned()),
            },
            Some(_) => return Err("Invalid key".to_owned()),
            None => return Err("Missing dictionary-ending e".to_owned()),
        }
    }
}
