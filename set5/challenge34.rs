extern crate num_bigint;
extern crate num_traits;
extern crate hex;
extern crate rand;
extern crate openssl;

mod dh;

use num_bigint::{BigUint};
use num_bigint::*;
use num_traits::*;
use openssl::symm::{decrypt, Cipher, Crypter,encrypt};
use num_bigint::Sign::*;
use dh::*;

fn main() {
    let mut alice = Dh::new();
    let mut bob = Dh::new();
    let mut m = DhMitm::new();
    let a = alice.start_exchange();
    let a_m = m.forward_start(a.0,a.1,a.2);
    let b = bob.reply(a_m.0,a_m.1,a_m.2);
    let b_m = m.forward_reply(b);
    alice.other_key_update(b_m);
    let message = b"This layer is not secure";
    println!("message: {:?}",message);
    let ciphertext = alice.aes_cbc_from_key(message);
    let decrypt = m.crack_message(&ciphertext);
    println!("decrypted: {:?}",decrypt);
}
