mod aes;

extern crate base64;
extern crate rand;

use std::fs::File;
use std::io::prelude::*;
use rand::Rng;

fn main() {
    let plaintext = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".as_bytes();
    let ciphertext = random_cipher_encryption(&plaintext);
    let ecb = detection_oracle(&ciphertext);
    if ecb {
        println!("ecb");
    }else{
        println!("cbc");
    }
}

fn detection_oracle(ciphertext: &[u8]) -> bool{
    let score = aes::detect_mode(ciphertext);
    let mut ecb = false;
    if score >= 2{
        ecb = true;
    }
    ecb
}

fn random_cipher_encryption(text: &[u8]) -> Vec<u8>{
    let mut rng = rand::thread_rng();
    let key = random_key();
    let iv = random_key();
    let input = aes::pad(scramble_input(text),16);
    let mut ciphertext = Vec::new();
    if rng.gen::<u8>() % 2 == 0{
        println!("encrypting cbc");
        ciphertext = aes::encrypt_aes_cbc(&input,&key,&iv);
    }else{
        println!("encrypting ecb");
        ciphertext = aes::encrypt_aes_ecb(&input,&key);
    }
    ciphertext
}

fn random_key() -> [u8;16]{
    let mut rng = rand::thread_rng();
    let mut key = [0;16];
    for i in 0..16{
        key[i] = rng.gen::<u8>();
    }
    key
}

fn scramble_input(text: &[u8])-> Vec<u8>{
    //let range = Range::new(5,10);
    let mut scrambled =  Vec::new();
    let mut rng = rand::thread_rng();
    let bytes_num: u8 = rng.gen_range(5,10);
    for i in 0..bytes_num{
        scrambled.push(rng.gen::<u8>());
    }
    for i in 0 as usize..text.len(){
        scrambled.push(text[i]);
    }
    for i in 0..bytes_num{
        scrambled.push(rng.gen::<u8>());
    }
    scrambled
}
