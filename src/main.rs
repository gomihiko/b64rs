extern crate anyhow;
extern crate argparse;

use anyhow::Result;
use argparse::ArgumentParser;
use argparse::Store;
use argparse::StoreFalse;
use argparse::StoreTrue;

mod b64rs;

pub fn main() -> Result<()> {
    // command line arguments
    let mut is_encode = false;
    let mut input: String = String::new();
    {
        // parse command line arguments
        let mut parser = ArgumentParser::new();
        parser.set_description("Encode or decode a string using Base64.");
        parser
            .refer(&mut is_encode)
            .add_option(&["-e", "--encode"], StoreTrue, "Encode a string")
            .add_option(&["-d", "--decode"], StoreFalse, "Decode a string")
            .required();

        parser
            .refer(&mut input)
            .add_argument("input", Store, "String to encode or decode.");
        parser.parse_args_or_exit();
    }

    // encode or decode the string
    if is_encode {
        println!("{}", b64rs::encode(&input));
    } else {
        println!("{}", b64rs::decode(&input));
    }

    // program complete
    Ok(())
}

#[cfg(test)]
#[test]
fn control_test_pass() {
    assert!(1 == 1); // OK
}

#[test]
#[should_panic]
fn control_test_panic() {
    panic!("contol test panic");
}

#[test]
fn encode_test() {
    assert!(b64rs::encode("hello world!") == "aGVsbG8gd29ybGQh")
}

#[test]
fn decode_test() {
    assert!(b64rs::decode("aGVsbG8gd29ybGQh") == "hello world!")
}
