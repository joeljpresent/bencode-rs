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
    test_str("le");
    test_str("li123ei456ee");
    test_str("l5:hello3:toie");
    test_str("lli123ei456eel5:hello3:toiee");
    test_str("de");
    test_str("d3:ouii47e2:no0:e");
    test_str("d3:ouili123ei456ee2:nod0:i12eee");
}
