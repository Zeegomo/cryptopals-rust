use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

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
	let frequency: HashMap<u8, u32> = [
		(b'a', 8),
		(b'e', 12),
		(b'h', 6),
		(b'i', 7),
		(b'n', 7),
		(b'o', 8),
		(b't', 8),
		(b's', 6),
		(b'r', 6),
		(b'd', 4),
		(b'l', 4),
		(b'c', 3),
		(b'u', 3),
		(b' ', 12),
	]
	.iter().cloned().collect();

	// Check if there are only ASCII values
	if xor.len() == 0 || !xor.iter().all(|c| c < &127) {
		return 0;
	}

	xor.iter().map(|b| frequency.get(&b.to_ascii_lowercase()).unwrap_or(&0)).sum()
}
