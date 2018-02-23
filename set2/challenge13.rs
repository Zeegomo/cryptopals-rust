extern crate openssl;
extern crate rand;

mod aes;

use self::openssl::symm::{encrypt, Cipher, Crypter, decrypt, Mode};
use rand::Rng;

fn main() {
    let key = b"dddddddddddddddd";
    let mut crafted = "1234567812admin\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11";
    let mut ciphertext = encrypt_aes_ecb(&profile_for(crafted), &key[0..]);
    let cracked = [86, 115, 233, 184, 157, 30, 173, 143, 47, 123, 122, 135, 148, 47, 27, 156, 98, 109, 27, 201, 35, 235, 210, 5, 202, 11, 236, 143, 95, 218, 199, 244, 81, 162, 206, 135, 77, 2, 41, 186, 211, 94, 48, 122, 32, 118, 151, 144];

    let mut plaintext = decrypt_aes_ecb(&ciphertext, &key[0..]);
    println!("crafted: {}", String::from_utf8(decrypt_aes_ecb(&cracked, &key[0..])).unwrap());
}


fn decrypt_aes_ecb(text: &[u8], key: &[u8]) -> Vec<u8> {
    let mut decrypter = Crypter::new(
        Cipher::aes_128_ecb(),
        Mode::Decrypt,
        key,
        None).unwrap();
    let mut decrypted = vec![0; 32];
    let mut plaintext = Vec::new();
    for i in 0..text.len() / 16 {
        decrypter.update(&text[16 * i..16 * i + 16], &mut decrypted);

        if i > 0 {
            for z in 16..32 {
                plaintext.push(decrypted[z]);
            }
        } else {
            for z in 0..16 {
                plaintext.push(decrypted[z]);
            }
        }
    }
    plaintext
}

fn encrypt_aes_ecb(text: &[u8], key: &[u8]) -> Vec<u8> {
    let text = aes::pad(text.to_vec(), 16);
    let mut encrypter = Crypter::new(
        Cipher::aes_128_ecb(),
        Mode::Encrypt,
        key,
        None, ).unwrap();
    let mut decrypted = vec![0; 32];
    let mut ciphertext = Vec::new();
    for i in 0..text.len() / 16 {
        encrypter.update(&text[16 * i..16 * i + 16], &mut decrypted);
        for z in 0..16 {
            ciphertext.push(decrypted[z]);
        }
    }
    ciphertext
}

fn profile_for(email: &str) -> Vec<u8> {
    //let rng = rand::thread_rng();
    let mut sanitized = "email=".as_bytes().to_vec();
    match email.find("=") {
        Some(_t) => (),
        None => sanitized.extend_from_slice(email.as_bytes()),
    };
    let slice = "&uid=10&role=user".as_bytes();
    sanitized.extend_from_slice(slice);
    sanitized
}

