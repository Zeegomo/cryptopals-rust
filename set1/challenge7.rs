extern crate openssl;
extern crate base64;

mod utils;

use openssl::symm::{decrypt, Cipher, Crypter};
use utils::*;

fn main() {
    let data = base64::decode(&get_text_from_file("data.txt")).unwrap();
    let plaintext = aes_ecb_decrypt(b"YELLOW SUBMARINE",&data);
    println!("plaintext: {}",String::from_utf8_lossy(&plaintext));
}

fn aes_ecb_decrypt(key: &[u8], data: &[u8]) -> Vec<u8>{
    let cipher = Cipher::aes_128_ecb();
    decrypt(
        cipher,
        key,
        None,
        &data
    ).expect("Error decrypting")
}