#[macro_use]
extern crate clap;
use clap::App;

extern crate anyhow;
use anyhow::Result;

mod b64rs;

pub fn main() -> Result<()> {
    // load cli config from yaml and parse args
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    // store args
    let encode = matches.is_present("ENCODE"); // true if -e
    let input_string = matches.value_of("INPUT").unwrap(); // input is required

    // encode or decode based on input
    if encode {
        println!("{}", b64rs::encode(input_string));
    } else {
        println!("{}", b64rs::decode(input_string));
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
