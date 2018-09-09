extern crate num_bigint;
extern crate num_traits;
extern crate hex;
extern crate rand;
extern crate openssl;

use num_bigint::BigUint;
use num_bigint::*;
use num_traits::*;
use openssl::symm::{decrypt, Cipher, Crypter, encrypt};
use num_bigint::Sign::*;
use rand::Rng;

mod dh;

use dh::*;

fn main() {
    let mut alice = Dh::new();
    let mut bob = Dh::new();
    let mut m = DhMitm::new();

    //g == 1
    let a = alice.start_exchange();
    let a_m = (a.0.clone(),BigInt::one(),a.2.clone());
    let b = bob.reply(a_m.0,a_m.1,a_m.2);
    alice.other_key_update(b);
    let message =b"this is not safe";
    let ciphertext = alice.aes_cbc_from_key(&message[0..]);
    println!("message: {:?}",message);
    m.key = BigInt::one();
    let decrypt = m.crack_message(&ciphertext);
    println!("decrypted: {:?}",decrypt);


    //g == p
    let a = alice.start_exchange();
    let a_m = (a.0.clone(),a.0.clone(),a.2.clone());
    let b = bob.reply(a_m.0,a_m.1,a_m.2);
    alice.other_key_update(b);
    let ciphertext = alice.aes_cbc_from_key(b"this is not safe");
    println!("message: {:?}",b"this is not safe");
    m.key = BigInt::zero();
    let decrypt = m.crack_message(&ciphertext);
    println!("decrypted: {:?}",decrypt);

    //g == p-1
    let a = alice.start_exchange();
    let a_m = (a.0.clone(),a.0.clone()-1,a.2.clone());
    let b = bob.reply(a_m.0,a_m.1,a_m.2);
    alice.other_key_update(b);
    println!("a_key: {}",alice.key);
    let ciphertext = alice.aes_cbc_from_key(b"this is not safe");
    println!("message: {:?}",b"this is not safe");
    m.key = BigInt::one();
    let mut decrypt = Vec::new();
    match m.crack_message(&ciphertext) {
        Ok(T) => decrypt = T,
        Err(_) =>  {
            m.key = m.p.clone() -1;
            match m.crack_message(&ciphertext) {
                Ok(T) => decrypt = T,
                Err(_) => println!("some error occurred"),
            }
        }
    }
    let decrypt = m.crack_message(&ciphertext);
    println!("decrypted: {:?}",decrypt);
}
