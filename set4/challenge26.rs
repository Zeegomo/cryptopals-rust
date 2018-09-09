extern crate openssl;

use openssl::symm::{encrypt, Cipher, Crypter,decrypt, Mode};
use std::io::prelude::*;
use std::fs::File;
//iphertext: 99, 111, 109, 109, 101, 110, 116, 49, 61, 99

//mettere meglio a punto
fn main() {
    let key = b"YELLOW SUBMARINE";
    let iv = [0;16];
    let mut text = b"comment1=cooking%20MCs;userdata=;comment2=%20like%20a%20pound%20of%20bacon";
    let mut text = [97, 100, 109, 105, 110, 61, 116, 114, 117, 101];


    for i in 0..10{
        text[i] ^= 10;
    }
    let mut ciphertext = aes_ctr_encrypt(&text,&iv,key);
    println!("ciphertext: {:?}",ciphertext);
    let mut decrypted = aes_ctr_encrypt(&ciphertext,&iv,key);
    println!("plaintext: {:?}",decrypted);
    //let injected = byte_xor(b"admin=true",&byte_xor(&ciphertext[0..11],&decrypted[0..11]));
    for i in 0..10{
        ciphertext[i] ^= 10;
    }
    decrypted = aes_ctr_encrypt(&ciphertext,&iv,key);
    println!("plaintext: {:?}",String::from_utf8_lossy(&decrypted));
}



fn aes_ctr_encrypt(plaintext: &[u8], iv: &[u8], key: &[u8]) -> Vec<u8> {
    let len = pad(plaintext,16).len()/16;
    let keystream = gen_keystream(len,iv,key);
    let ciphertext = byte_xor(plaintext,&keystream);
    ciphertext
}

fn gen_keystream(len: usize, iv: &[u8], key: &[u8]) -> Vec<u8> {
    let mut keystream = Vec::new();
    let mut aes = Vec::new();
    let mut nonce = iv.to_vec().clone();
    for i in 0..len {
        aes = aes_ebc(&nonce,key);
        for z in 0..16{
            keystream.push(aes[z]);
        }
        nonce[8] += 1;
        //println!("nonce: {}",nonce[7]);
    }
    keystream
}

fn profile(text: &[u8]) -> Vec<u8> {
    let mut input = "comment1=cooking%20MCs;userdata=".as_bytes().to_vec();
    input.extend_from_slice(text);
    input.extend_from_slice(";comment2=%20like%20a%20pound%20of%20bacon".as_bytes());
    input
}

fn pad(text: &[u8], length: u8) -> Vec<u8>{
    let mut padded = text.to_vec().clone();
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

fn get_text_from_file() -> Vec<u8>{
    let mut file = match File::open("/home/zeegomo/Documents/hack/cryptopals/set2/challenge2/src/data.txt") {
        Ok(file) => file,
        Err(_) => panic!("no such file"),
    };
    let mut text = String::new();
    file.read_to_string(&mut text)
        .ok()
        .expect("failed to read!");
    text.as_bytes().to_vec()
    //base64::decode(&text).unwrap()
}

fn byte_xor(byte1: &[u8], byte2: &[u8]) -> Vec<u8> {

    let mut xor = Vec::new();
    for i in 0..byte1.len() {
        xor.push(byte1[i] ^ byte2[i]);
    }
    xor
}

fn aes_ebc(block: &[u8], key: &[u8]) -> Vec<u8>{

    let cipher = Cipher::aes_128_ecb();
    let plaintext =  encrypt(
        cipher,
        &key,
        None,
        &block
    ).expect("AES_ECB error");
    plaintext


}