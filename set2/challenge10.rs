extern crate base64;
extern crate openssl;

mod utils;

use openssl::symm::{encrypt, Cipher, Crypter,decrypt, Mode};
use std::io::prelude::*;
use std::fs::File;
use utils::*;


fn main() {
    let mut ciphertext = get_text_from_file_base64("data.txt");
    let iv = [0;16];
    let key = b"YELLOW SUBMARINE";
    let plaintext = aes_cbc_decrypt(&ciphertext,&iv,key);
    println!("{}",String::from_utf8_lossy(&plaintext));
}

fn pad(text: Vec<u8>, length: u8) -> Vec<u8>{
    let mut padded = text.clone();
    let mut i = 1;
    while (length as i8*i-text.len() as i8) <0 {
        i += 1;
    }
    let padding = length*i as u8-text.len() as u8;

    for i in text.len()..(padding as usize +text.len()){
        padded.push(padding);
    }
    padded
}

fn aes_cbc_decrypt(text: &[u8], iv: &[u8], key: &[u8]) ->Vec<u8>{
    let mut decrypter  = Crypter::new(
        Cipher::aes_128_ecb(),
        Mode::Decrypt,
        key,
        None,
    ).unwrap();

    let mut last = iv;
    let mut decrypted = vec![0;32];
    let mut plaintext = Vec::new();
    let mut xored = Vec::new();
    let mut count = 0;
    for i in 0..text.len()/16{

        decrypter.update(&text[16*i..16*i+16],&mut decrypted);
        if i>0 {xored = byte_xor(&decrypted[16..],last);}
        else{xored = byte_xor(&decrypted[0..16],last);}
        last = &text[16*i..16*i+16];
        for z in 0..16 as usize{
            plaintext.push(xored[z]);
        }
    }
    plaintext
}