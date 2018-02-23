mod aes;
mod utils;

extern crate openssl;
extern crate base64;

use openssl::symm::{encrypt, Cipher, Crypter, decrypt, Mode};
use std::io::prelude::*;
use std::fs::File;
use utils::*;

fn main() {
    let input = base64::decode(lines_from_file("data.txt")[0].as_bytes()).unwrap();
    let key = b"YELLOW SUBMARINE";
    let iv = [0; 16];
    let ciphertext = aes::aes_cbc_encrypt(&input, &iv, key);
    println!("guessed: {:?}", String::from_utf8_lossy(&padding_oracle(&ciphertext, &iv, key)));
}

fn padding_oracle(ciphertext: &[u8], iv: &[u8], key: &[u8]) -> Vec<u8> {
    let mut guessed = Vec::new();
    let len = ciphertext.len();
    let mut padding_result = Vec::new();
    let mut first_block = iv.to_vec();
    first_block.extend_from_slice(&ciphertext[0..16]);
    padding_result = block_padding_oracle(&first_block, iv, key);
    for i in 0..16 {
        guessed.push(padding_result[i]);
    }
    for z in 0..ciphertext.len() / 16 {
        if 16 * z + 32 <= ciphertext.len() {
            padding_result = block_padding_oracle(&ciphertext[16 * z..16 * z + 32], iv, key);
            for i in 0..16 {
                guessed.push(padding_result[i]);
            }
        }
    }
    guessed
}

fn block_padding_oracle(ciphertext: &[u8], iv: &[u8], key: &[u8]) -> Vec<u8> {
    let mut found = false;
    let mut crafted = ciphertext.to_vec().clone();
    let mut guessed: Vec<u8> = Vec::new();
    let mut lol = Vec::new();
    let len = ciphertext.len();
    let mut pre_xor = 0;
    let mut i = 0;

    for z in 0..16 {
        found = false;

        while found == false {
            if crafted[len - 17 - z] < 255 {
                crafted[len - 17 - z] += 1;
            } else {
                crafted[len - 17 - z] = 0;
            }
            i += 1;
            found = aes::validate_padding(&aes::aes_cbc(&crafted, iv, key));
        }
        pre_xor = aes::byte_xor(&[crafted[len - 17 - z]], &[z as u8 + 1])[0];
        lol.push(pre_xor);

        for h in 0..(z + 1) {
            crafted[len - 17 - h] = aes::byte_xor(&[lol[h]], &[z as u8 + 2])[0];
        }
        guessed.insert(0, aes::byte_xor(&[pre_xor], &[ciphertext[len - 17 - z]])[0]);
    }
    guessed
}
