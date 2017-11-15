extern crate rustc_serialize as serialize;

use serialize::base64::{self, ToBase64};
use serialize::hex::FromHex;

fn main() {
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    // let result = input.from_hex().unwrap().as_slice().to_base64(base64::STANDARD);
    let result = hex_to_base64(input);
    println!("This is the input -> {}", input);
    println!("This is the result -> {}", result);
}

fn hex_to_base64(hex_string: &str) -> String {
    return hex_string.from_hex().unwrap().as_slice().to_base64(base64::STANDARD);
}
