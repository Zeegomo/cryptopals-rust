extern crate openssl;
extern crate base64;

mod aes;
mod utils;

use openssl::symm::{encrypt, Cipher, Crypter, decrypt, Mode};
use utils::*;
fn main() {
    let key = b"YELLOW SUBMARINE";
    let nonce = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
    let ciphertex = get_text_from_file_base64("data.txt");
    let plaintext = aes_ctr_encrypt(&ciphertex,&nonce,key);
    println!("plaintext: {:?}",String::from_utf8_lossy(&plaintext)/);
}

fn aes_ctr_encrypt(plaintext: &[u8], iv: &[u8], key: &[u8]) -> Vec<u8> {
    let len = aes::pad(plaintext,16).len()/16;
    let keystream = gen_ctr_keystream(len,iv,key);
    let ciphertext = byte_xor(plaintext,&keystream);
    ciphertext
}

fn gen_ctr_keystream(len: usize, iv: &[u8], key: &[u8]) -> Vec<u8> {
    let mut keystream = Vec::new();
    let mut aes = Vec::new();
    let mut nonce = iv.to_vec().clone();
    for i in 0..len {
        aes = aes::encrypt_aes_ecb(&nonce,key);
            for z in 0..16{
                keystream.push(aes[z]);
            }
        iterate_nonce(&mut nonce);
    }
    keystream
}

fn iterate_nonce(nonce: &mut [u8]){
    for i in 8..16{
        if nonce[i] == 255{
            nonce[i] = 0;
            nonce[(i+1)%16] += 1;
        }
    }
    nonce[8] += 1;
}