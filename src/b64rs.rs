// helper fns

// b64 indices string
const B64_INDICES: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn get_b64_index(chara: char) -> u8 {
    B64_INDICES.find(chara).unwrap() as u8
}

fn get_b64_char(index: u8) -> char {
    B64_INDICES.chars().nth(index.into()).unwrap()
}

// converts a char to a binary String
fn char_to_bin(input: char) -> String {
    dec_to_bin(input as u8)
}

// converts a decimal into a binary String
fn dec_to_bin(input: u8) -> String {
    format!("{:b}", input)
}

// converts a binary String into a decimal
fn binary_to_dec(input: &str) -> u8 {
    u8::from_str_radix(input, 2).unwrap()
}

// encode
// encodes a string using base 64.
// IN:
// 	input - unencoded string input
// OUT:
// 	encoded - encoded String output
pub fn encode(input: &str) -> String {
    let mut asciistr = String::new();
    let mut asciivals = vec![];

    // get ascii codes for each character
    for letter in input.chars() {
        let x: String = (char_to_bin(letter)).to_string();
        asciivals.push(x);
    }

    // pad ascii values if necessary to make them 8 bits
    for mut val in asciivals {
        let mut padding = String::new();
        if val.len() < 8 {
            let l_padding = 8 - val.len();
            for _i in 0..l_padding {
                padding += "0";
            }
            val = format!("{}{}", padding, val);
        }

        // append the ascii value to the string
        asciistr = format!("{}{}", asciistr, val);
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
    let mut encoded = String::new();
    for decimal in dec {
        let found_char: char = get_b64_char(decimal);
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
        indices.push(get_b64_index(chara));
    }

    // convert the decimal indices to binary representation
    let mut indices_bytes = vec![];
    for index in indices {
        indices_bytes.push(dec_to_bin(index as u8));
    }

    // pad the binary values if necessary
    let mut bits_str = String::new();
    for mut byte in indices_bytes {
        if byte.len() < 6 {
            let padding_amount = 6 - byte.len();
            let mut padding = String::new();
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
    let mut decoded = String::new();
    for byte in bytes {
        decoded.push((binary_to_dec(&byte)) as char);
    }

    // return decoded String
    decoded
}
