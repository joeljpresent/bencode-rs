mod bencode;

use bencode::get_next_value;

fn test_str(s: &str) {
    match get_next_value(s.as_bytes()) {
        Ok((val, rest)) => println!("{:?} {}", val, String::from_utf8_lossy(&rest)),
        Err(err) => println!("ERROR! {}", err),
    }
}

fn main() {
    test_str("0:");
    test_str("3:oui");
    test_str("7:le:deux-points");
    test_str("i42e");
    test_str("i-666efe");
}
