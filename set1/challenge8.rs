extern crate hex;
extern crate openssl;

mod utils;

use std::io::prelude::*;
use std::fs::File;
use openssl::memcmp::eq;
use utils::*;

fn main() {
    let strings = lines_from_file("data.txt");
    let mut hex = Vec::new();
    for i in 0..strings.len(){
        hex.push(hex::decode(&strings[i]).unwrap());
    }
    let aes_guess = ecb_detect(hex);
    println!("guess: {}",hex::encode(aes_guess));
}

fn ecb_detect(hex: Vec<Vec<u8>>) -> Vec<u8>{
    let mut best = (Vec::new(),0);
    let mut round_score = 0;
    for i in 0..hex.len(){
        round_score = duplicate_score(&hex[i]);
        if round_score > best.1{
            best.1 = round_score;
            best.0 = hex[i].clone();
        }
    }
    println!("score: {}",best.1);
    best.0
}

fn duplicate_score(text: &[u8]) -> u32{
    let mut score = 0;
    for i in 0..(text.len()/16){
        for z in (i+1)..(text.len()/16){
            if i*z*16 +16 < text.len() && eq(&text[16*i..i*16+16],&text[i*z*16..i*z*16+16]) == true{
                score +=1;
            }
        }
    }
    score
}