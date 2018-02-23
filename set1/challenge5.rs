mod utils;

use std::io::prelude::*;
use std::fs::File;
use utils::*;

fn main() {

    let text_in_bytes = get_text_from_file("data.txt");
    let key: [u8;3] = [73,67,69];
    let keystream = gen_keystream(&key, text_in_bytes.len());
    println!("keystream: {:?}",keystream);
    let plaintext = byte_xor(&text_in_bytes,&keystream);

    for i in 0..plaintext.len(){
        print!("{:x}",plaintext[i]);
    }
}

fn gen_keystream(key: &[u8], len: usize) -> Vec<u8>{
    let mut keystream = key.to_vec();
    for i in 0..len/key.len()+1{
        for i in 0..key.len(){
            keystream.push(key[i]);
        }
    }
    keystream
}