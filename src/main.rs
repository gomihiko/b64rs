extern crate anyhow;
extern crate argparse;

use anyhow::Result;
use argparse::ArgumentParser;
use argparse::Store;
use argparse::StoreFalse;
use argparse::StoreTrue;

// helper fns
pub fn char_to_bin(input: char) -> String {
    let x = input as u8;
    dec_to_bin(x)
}

pub fn dec_to_bin(input: u8) -> String {
    format!("{:b}", input)
}

pub fn binary_to_dec(input: &str) -> u8 {
    u8::from_str_radix(input, 2).unwrap()
}

// b64 indices string
pub const B64_INDICES: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

// encode
// encodes a string using base 64.
// IN:
// 	input - unencoded string input
// OUT:
// 	encoded - encoded String output
pub fn encode(input: &str) -> String {
    let mut asciistr: String = String::new();
    let mut asciivals = Vec::new();

    // get ascii codes for each character
    for letter in input.chars() {
        let x: String = (char_to_bin(letter)).to_string();
        asciivals.push(x);
    }
    for val in asciivals {
        let mut padding: String = String::new();
        if val.len() < 8 {
            let l_padding = 8 - val.len();
            for _i in 0..l_padding {
                padding += "0";
            }
        }

        let padded: String = format!("{}{}", padding, val);
        asciistr = format!("{}{}", asciistr, padded);
    }

    // divide into groups of 6
    let mut bytes = vec![];
    while !asciistr.is_empty() {
        if (asciistr.len()) >= 6 {
            bytes.push((asciistr[0..6]).to_string());
            asciistr = (asciistr[6..]).to_string();
        } else {
            bytes.push(asciistr[..].to_string());
            String::clear(&mut asciistr);
        }
    }

    // pad the last group with 0s if necessary
	let last_size: i8 = bytes.last().unwrap().chars().count() as i8;
    if last_size < 6 {
        let padding_amount = 6 - bytes[bytes.len() - 1].len();
        let mut padding = String::new();
        for _i in 0..padding_amount {
            padding += "0";
        }
        let index = bytes.len() - 1;
        bytes[index] += &padding;
    }

    // convert from ascii binary to decimal
    let mut dec = vec![];
    for byte in bytes {
        dec.push(binary_to_dec(&byte[..]));
    }

    // convert decimals to characters in the b64 str
    let mut encoded: String = String::new();
    for decimal in dec {
        let found_char: char = B64_INDICES.chars().nth(decimal.into()).unwrap();
        encoded.push(found_char);
    }

    // return the encoded string
    encoded
}

// decode
// decodes a string using base 64.
// IN:
// 	input - encoded string input
// OUT:
// 	encoded - unencoded String output
pub fn decode(input: &str) -> String {
    // convert encoded characters to indices from the b64 string
    let mut indices = vec![];
    for chara in input.chars() {
        indices.push(B64_INDICES.find(chara).unwrap());
    }

    // convert the decimal indices to binary representation
    let mut indices_bytes = vec![];
    for index in indices {
        indices_bytes.push(dec_to_bin(index as u8));
    }

    // pad the binary values if necessary
    let mut bits_str: String = String::new();
    for mut byte in indices_bytes {
        if byte.len() < 6 {
            let padding_amount = 6 - byte.len();
            let mut padding: String = String::new();
            for _i in 0..padding_amount {
                padding += "0";
            }
            byte = format!("{}{}", padding, byte);
        }
        bits_str += &byte;
    }

    // divide the binary string into bytes
    let mut bytes = vec![];
    while !bits_str.is_empty() {
        if (bits_str.len()) >= 8 {
            bytes.push(bits_str[0..8].to_string());
            bits_str = (&bits_str[8..]).to_string();
        } else {
            bytes.push(bits_str.to_string());
            String::clear(&mut bits_str);
        }
    }

    // decode ascii bytes to characters in str
    let mut decoded: String = String::new();
    for byte in bytes {
        decoded.push((binary_to_dec(&byte)) as char);
    }

    // return decoded String
    decoded
}

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
        println!("{}", encode(&input));
    } else {
        println!("{}", decode(&input));
    }

    // program complete
    Ok(())
}
