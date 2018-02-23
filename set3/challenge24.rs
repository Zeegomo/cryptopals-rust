mod mersenne;
mod utils;

use mersenne::MersenneTwister;
use utils::*;

fn main() {
    let plaintext = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    println!("plaintext: {}",plaintext);
    let ciphertext = byte_xor(plaintext.as_bytes(), &gen_mersenne_key(plaintext.len() as u32, 65535));
    let decrypt = byte_xor(&ciphertext, &gen_mersenne_key(ciphertext.len() as u32, 65535));
    println!("decrypted: {}", String::from_utf8_lossy(&decrypt));
    brute_force(&ciphertext);
}

fn brute_force(ciphertext: &[u8]) {
    let mut decrypt = Vec::new();
    for i in 0..65536 {
        decrypt = byte_xor(&ciphertext, &gen_mersenne_key(ciphertext.len() as u32, i));
        if decrypt[decrypt.len() - 1] == 97 && decrypt[decrypt.len() - 2] == 97 && decrypt[decrypt.len() - 3] == 97{
            println!("seed: {}", i);
            println!("decrypt: {}", String::from_utf8_lossy(&decrypt));
            break;
        }
    }
}


fn gen_mersenne_key(len: u32, seed: u32) -> Vec<u8> {
    const MASK_1: u32 = 0xFF000000;
    const MASK_2: u32 = 0x00FF0000;
    const MASK_3: u32 = 0x0000FF00;
    const MASK_4: u32 = 0x000000FF;

    let mut rand = MersenneTwister::new_from_seed(seed);

    let mut key = Vec::new();
    let mut round_rand = 0;
    for i in 0..len / 4 + 1 {
        round_rand = rand.gen_rand();

        key.push(byte_group(round_rand, 0));
        key.push(byte_group(round_rand, 1));
        key.push(byte_group(round_rand, 2));
        key.push(byte_group(round_rand, 3));
    }
    key
}


fn byte_group(y: u32, position: u8) -> u8 {
    let y = convert_to_bits(y);
    let mut group = 0;
    bits_to_u32(&y[position as usize * 8..position as usize * 8 + 8]) as u8
}