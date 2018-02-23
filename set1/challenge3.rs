mod utils;

use std::env;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use utils::*;

fn main() {

    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {

        let key = find_key(&hex_to_bytes(&args[1]));
        println!("key: {}", key);
        println!("{}",String::from_utf8(single_byte_xor(&hex_to_bytes(&args[1]),key)).unwrap());

    }
}

fn find_key(encrypted: &[u8]) -> u8 {
    let mut round_key = 0;
    let mut round_score = 0;
    let mut best = (0, 0);
    let mut xored = Vec::new();

    while round_key < 128 {

        round_score = 0;
        xored = single_byte_xor(&encrypted, round_key);
        round_score += get_score(&xored);

        if best.1 < round_score {
            best.0 = round_key;
            best.1 = round_score;
        }
        round_key += 1;
    }

    best.0
}