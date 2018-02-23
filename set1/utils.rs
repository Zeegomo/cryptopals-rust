use std::fs::File;
use std::io::prelude::*;

pub fn lines_from_file(filename: &str) -> Vec<String> {
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => panic!("no such file"),
    };
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .ok()
        .expect("failed to read!");
    let lines: Vec<String> = file_contents.split("\n")
        .map(|s: &str| s.to_string())
        .collect();
    lines
}


pub fn count_bit(bit: &mut [u8]) -> f32 {
    let mut counter = 0.0;
    for i in 0..bit.len() {
        counter += bit[i].count_ones() as f32;
    }
    counter
}


pub fn byte_xor(byte1: &[u8], byte2: &[u8]) -> Vec<u8> {
    let mut xor = Vec::new();
    for i in 0..byte1.len() {
        xor.push(byte1[i] ^ byte2[i]);
    }
    xor
}

pub fn get_text_from_file(filename: &str) -> Vec<u8>{
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => panic!("no such file"),
    };
    let mut text = String::new();
    file.read_to_string(&mut text)
        .ok()
        .expect("failed to read!");
    text.as_bytes().to_vec()
}

pub fn single_byte_xor(string: &[u8], key: u8) -> Vec<u8> {
    let mut bytes = Vec::new();
    for mut i in 0..string.len() {
        bytes.push(string[i] ^ key);
    }
    bytes
}

pub fn hex_to_bytes(string: &str) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();
    for i in 0..(string.len() / 2) {
        match u8::from_str_radix(&string[2 * i..2 * i + 2], 16) {
            Ok(n) => bytes.push(n),
            Err(_) => println!("Error in hex conversion"),
        }
    }
    bytes
}



pub fn get_score(xor: &Vec<u8>) -> u32 {
    let mut score = 0;

    for n in 0..xor.len() {

        if xor[n] == ("a").as_bytes()[0] || xor[n] == ("A").as_bytes()[0] {
            score += 8;
        }
        if xor[n] == ("e").as_bytes()[0] || xor[n] == ("E").as_bytes()[0] {
            score += 12;
        }
        if xor[n] == ("h").as_bytes()[0] || xor[n] == ("H").as_bytes()[0] {
            score += 6;
        }
        if xor[n] == ("i").as_bytes()[0] || xor[n] == ("I").as_bytes()[0] {
            score += 7;
        }
        if xor[n] == ("n").as_bytes()[0] || xor[n] == ("N").as_bytes()[0] {
            score += 7;
        }
        if xor[n] == ("o").as_bytes()[0] || xor[n] == ("O").as_bytes()[0] {
            score += 8;
        }
        if xor[n] == ("t").as_bytes()[0] || xor[n] == ("T").as_bytes()[0] {
            score += 8;
        }
        if xor[n] == ("s").as_bytes()[0] || xor[n] == ("S").as_bytes()[0] {
            score += 6;
        }
        if xor[n] == ("r").as_bytes()[0] || xor[n] == ("R").as_bytes()[0] {
            score += 6;
        }
        if xor[n] == ("d").as_bytes()[0] || xor[n] == ("D").as_bytes()[0] {
            score += 4;
        }
        if xor[n] == ("l").as_bytes()[0] || xor[n] == ("L").as_bytes()[0] {
            score += 4;
        }
        if xor[n] == ("c").as_bytes()[0] || xor[n] == ("C").as_bytes()[0] {
            score += 3;
        }
        if xor[n] == ("u").as_bytes()[0] || xor[n] == ("U").as_bytes()[0] {
            score += 3;
        }
        if xor[n] == (" ").as_bytes()[0] {
            score += 12;
        }
    }

    for n in 0..xor.len() {
        if xor[n] > 127 {
            score = 0;
        }
    }

    score
}
