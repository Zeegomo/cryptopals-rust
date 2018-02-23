extern crate base64;

mod hamming;
mod utils;

use std::fs::File;
use std::io::prelude::*;
use hamming::*;
use utils::*;

fn main() {
    println!("reading file...");
    let text1 = get_text_from_file("/home/zeegomo/Documents/hack/cryptopals/set1/challenge6/src/data.txt");

    println!("decoding base64...");
    let text1_bytes = base64::decode(&text1).unwrap();
    println!("guessing key length...");
    let best_key = hamming::guess_key_lenght(2,40,&text1_bytes);
    println!("guessed key length: {}, {}, {}",best_key.0,best_key.2,best_key.4);

    let blocks1 = hamming::get_blocks(&text1_bytes,best_key.0);
    let blocks2 = hamming::get_blocks(&text1_bytes,best_key.2);
    let blocks3 = hamming::get_blocks(&text1_bytes,best_key.4);

    println!("generating probable keys...");
    let keys1 = hamming::gen_keys(blocks1);
    let keys2 = hamming::gen_keys(blocks2);
    let keys3 = hamming::gen_keys(blocks3);

    println!("decrypting...");
    let plaintext1 = hamming::encrypt_repeating_xor(&text1_bytes,&keys1);
    let plaintext2 = hamming::encrypt_repeating_xor(&text1_bytes,&keys2);
    let plaintext3 = hamming::encrypt_repeating_xor(&text1_bytes,&keys3);

    println!("PROBABLE MATCH FOUND: {}", String::from_utf8(plaintext1).unwrap());
    println!();
    println!();
    println!("PROBABLE MATCH FOUND: {}", String::from_utf8(plaintext2).unwrap());
    println!();
    println!();
    println!("PROBABLE MATCH FOUND: {}", String::from_utf8(plaintext3).unwrap());


}

