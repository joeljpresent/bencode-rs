mod decoding;
mod encoding;
mod value;

use value::Value;

fn test_str(s: &str) {
    match Value::decode(s.as_bytes()) {
        Ok(val) => {
            println!("{:?}", val);
            println!("{}", String::from_utf8_lossy(&val.encode()))
        }
        Err(err) => println!("ERROR! {}", err),
    }
}

fn main() {
    test_str("0:");
    test_str("3:oui");
    test_str("7:le:deux-points");
    test_str("26:abcdefghijklmnopqrstuvwxyz");
    test_str("i42e");
    test_str("i-666efe");
    test_str("le");
    test_str("li123ei456ee");
    test_str("l5:hello3:toie");
    test_str("lli123ei456eel5:hello3:toiee");
    test_str("de");
    test_str("d3:ouii47e2:no0:e");
    test_str("d3:ouili123ei456ee2:nod0:i12eee");
    test_str(r#"d8:announce41:http://bttracker.debian.org:6969/announce7:comment35:"Debian CD from cdimage.debian.org"10:created by13:mktorrent 1.113:creation datei1671279452e4:infod6:lengthi3909091328e4:name29:debian-11.6.0-amd64-DVD-1.iso12:piece lengthi262144e6:pieces41:(binary blob of the hashes of each piece)ee"#);
}
