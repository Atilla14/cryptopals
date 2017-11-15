extern crate rustc_serialize as serialize;

use serialize::base64::{self, ToBase64};
use serialize::hex::FromHex;

fn main() {
    //set-1 challenge-1
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let result = utf8_to_base64(&string_to_hex(input));

    //set-1 challenge-2
    let input1 = "1c0111001f010100061a024b53535009181c";
    let input2 = "686974207468652062756c6c277320657965";
    let result2 = bitwise_xor(&string_to_hex(input1), &string_to_hex(input2));
    let result2_string = to_hex_string(result2);

    println!("Hex string -> {}", input);
    println!("Base64 conversion -> {}", result);

    println!("First hex string -> {}", input1);
    println!("Second hex string -> {}", input2);
    println!("Bitwise XOR result -> {}", result2_string);
}

fn string_to_hex(hex_string: &str) -> Vec<u8> {
    hex_string.from_hex().unwrap()
}

fn utf8_to_base64(val: &[u8]) -> String {
    val.to_base64(base64::STANDARD)
}

fn bitwise_xor(buf_a: &[u8], buf_b: &[u8]) -> Vec<u8> {
    let mut v = Vec::new();
    let xiter = buf_b.iter().cycle();
    let pairs = buf_a.iter().zip(xiter);
    for (a, b) in pairs {
        v.push(a ^ b);
    }
    v
}

fn to_hex_string(bytes: Vec<u8>) -> String {
    let strs: Vec<String> = bytes.iter()
        .map(|b| format!("{:02X}", b))
        .collect();
    strs.join("")
}
