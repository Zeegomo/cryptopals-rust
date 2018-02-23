extern crate openssl;
extern crate base64;

mod aes;
mod utils;

use openssl::symm::{encrypt, Cipher, Crypter, decrypt, Mode};
use utils::*;

fn main() {
    let mut input_encoded = lines_from_file("data.txt");
    let mut input = Vec::new();
    for i in 0..input_encoded.len(){
        input.push(base64::decode(&input_encoded[i]).unwrap());
    }
    let iv = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
    let key = b"YELLOW SUBMARINE";
    for i in 0..input.len(){
        input[i] = aes::aes_ctr_encrypt(&input[i],&iv,key);
    }
    let mut block = Vec::new();
    for i in 0..20{
        block.push(Vec::new());
        for z in 0..input.len(){
            block[i].push(input[z][i]);
        }
    }
    let keystream = guess_keystream(block);
    println!("keystream: {:?}, len: {}",keystream,keystream.len());
    for i in 0..input.len(){
        println!("plaintext: {}",String::from_utf8_lossy(&byte_xor(&keystream,&input[i])));
    }

}

fn guess_keystream(text: Vec<Vec<u8>>) -> Vec<u8>{
    let mut round_score = 0;
    let mut best_score = 0;
    let mut best_key = 0;
    let mut xor = Vec::new();
    let mut keystream = Vec::new();
    for i in 0..text.len(){
        best_score = 0;
        best_key = 0;
        for z in 0..255{
            xor = single_byte_xor(&text[i],z as u8);
            round_score = get_score(&xor);
            if round_score > best_score{
                best_key = z;
                best_score = round_score;
            }
        }
        keystream.push(best_key);
    }
    keystream
}