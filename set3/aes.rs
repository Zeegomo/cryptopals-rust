extern crate openssl;
extern crate base64;

use self::openssl::symm::{encrypt, Cipher, Crypter, decrypt, Mode};
use self::openssl::memcmp::eq;

pub fn validate_padding(text: &[u8]) {
    let len = text.len();
    let last = text[len-1];
    println!("len: {}, last: {}",len,last);
    if last as usize >= len {
        panic!("INVALID PADDING");
    }
    for i in len-last as usize..len{
        if text[i] != last{
            panic!("INVALID PADDING");
        }
    }
}

pub fn encrypt_aes_ecb(text: &[u8], key: &[u8]) -> Vec<u8> {
    let cipher = Cipher::aes_128_ecb();
    let plaintext = encrypt(
        cipher,
        key,
        None,
        text,
    ).expect("AES_128_ECB encryption error");
    plaintext[..text.len()].to_vec()
}

pub fn encrypt_aes_cbc(text: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
    let mut encrypted = iv.to_vec();
    let mut ciphertext = Vec::new();

    for i in 0..text.len() / 16 {
        encrypted = encrypt_aes_ecb(&byte_xor(&text[16 * i..16 * i + 16], &encrypted), key);
        {
            for z in 0..16 {
                ciphertext.push(encrypted[z]);
            }
        }
    }
    ciphertext
}

fn byte_xor(byte1: &[u8], byte2: &[u8]) -> Vec<u8> {
    let mut xor = Vec::new();
    for i in 0..byte1.len() {
        xor.push(byte1[i] ^ byte2[i]);
    }
    xor
}

pub fn pad(text: Vec<u8>, length: u8) -> Vec<u8> {
    let mut padded = text.clone();
    let mut i: i32 = 1;
    while (length as i32 * i - text.len() as i32) < 0 {
        i += 1;
    }
    let padding = (length as u32 * i as u32 - text.len() as u32) as u8;
    for i in text.len()..(padding as usize + text.len()) {
        padded.push(padding);
    }

    padded
}

pub fn detect_mode(ciphertext: &[u8]) -> i32 {
    let score = duplicate_score(ciphertext);
    score as i32 / ciphertext.len() as i32
}

fn duplicate_score(text: &[u8]) -> u32 {
    let mut count = 0;
    for i in 0..text.len() {
        for z in 2..16 {
            if i + 16 + z < text.len() && &text[i..i + z] == &text[i + 16..i + 16 + z] {
                count += z as u32;
            }
        }
    }
    count
}

