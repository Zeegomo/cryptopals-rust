extern crate base64;
extern crate openssl;
extern crate rand;

mod aes;
mod utils;

use std::io::prelude::*;
use std::fs::File;
use rand::Rng;
use openssl::symm::{encrypt, Cipher, Crypter, decrypt, Mode};
use openssl::memcmp::eq;


fn main() {
    let key = aes::random_key();
    let secret = utils::get_text_from_file("data.txt");
    let guessed = byte_ecb(&secret, &key);
    println!("guessed: {}", String::from_utf8_lossy(base64::decode(&guessed).unwrap()));
}

fn byte_ecb(text: &[u8], key: &[u8]) -> Vec<u8> {
    let message_length = aes::pad(text.to_vec(), 16).len();
    let mut crafted = Vec::new();
    for _i in 0..message_length {
        crafted.push(0);
    }

    let message_length = aes::pad(text.to_vec(), 16).len();
    crafted.remove(0);
    crafted.extend_from_slice(text);

    let mut dict = Vec::new();
    let mut guessed = Vec::new();
    let mut ciphertext = aes::encrypt_aes_ecb(&crafted, key);
    let mut brute_force = Vec::new();
    for _i in 0..message_length {
        brute_force.push(0);
    }
    while crafted.len() >= message_length {

        for i in 0..256 {
            brute_force.pop();
            brute_force.push(i as u8);
            dict = aes::encrypt_aes_ecb(&brute_force, key);
            if dict == &ciphertext[0..dict.len()] {
                guessed.push(i as u8);
            }
        }

        crafted.remove(0);
        ciphertext = aes::encrypt_aes_ecb(&crafted,key);
        brute_force.clear();
        for z in 0..message_length {
            if z < crafted.len(){brute_force.push(crafted[z]);}
        }
    }
    guessed
}
