extern crate openssl;

use openssl::symm::{encrypt, Cipher, Crypter,decrypt, Mode};
use std::io::prelude::*;
use std::fs::File;
//iphertext: [32, 136, 124, 97, 212, 176, 15, 167, 9, 112, 240, 103, 131, 31, 215, 153,|| 253, 26, 84, 75, 222, 71, 255, 255, 66, 49, 153, 18, 25, 115, 185, 62,||| 36, 162, 83, 129, 135, 172, 231, 243, 113, 11, 115, 245, 13, 49, 9, 69,|||| 89, 193, 6, 255, 158, 206, 247, 104, 94, 67, 66, 88, 104, 184, 97, 57, 141, 69, 135, 13, 200, 32, 157, 44, 101, 110, 41, 71, 179, 128, 157, 241, 42, 6, 128, 168, 45, 114, 99, 46, 222, 181, 158, 87, 163, 192, 115, 255]

// xor: [0, 2, 11, 0, 4, 11, 83, 0, 64, 72, 64, 9]

fn main() {
    let mut plaintext = profile("234567".as_bytes());/*get_text_from_file()*//*vec![0,2,11,0,4,11,83,0,64,72,64,9,4,3,2,1]*/;
    //ciphertext = pad(ciphertext,16);
    let mut plaintext = b"yellow submarine high gasoline monster palpatine";
    let iv = b"YELLOW SUBMARINE";
    let key = b"YELLOW SUBMARINE";
    let mut ciphertext = aes_cbc_encrypt(plaintext,iv,key);

    println!("ciphertext: {:?}",ciphertext.len());
    for i in 16..32{
        ciphertext[i] = 0;
    }

    for i in 32..48{
        ciphertext[i] = ciphertext[i-32];
    }
    //println!("plaintext before: {:?}",plaintext);
    let plaintext = &aes_cbc(ciphertext,iv, key);
    println!("plaintext after: {:?}",plaintext);
    //println!("{:?}",plaintext);
    //println!("{}",73 as char);
    //print!("{}", String::from_utf8_lossy(&plaintext));
    /*for i in 0..plaintext.len(){
        println!("{}",plaintext[i] as char);
    }*/
    println!("xor: {:?}",String::from_utf8_lossy(&byte_xor(&[121, 101, 108, 108, 111, 119, 32, 115, 117, 98, 109, 97, 114, 105, 110, 101],&[32, 32, 32, 32, 32, 32, 0, 32, 32, 32, 32, 32, 32, 32, 32, 32])));

    println!("{}",String::from_utf8(plaintext.to_vec()).unwrap());
    //println!("{}",String::from_utf8(plaintext).unwrap());

}

fn profile(text: &[u8]) -> Vec<u8> {
    let mut input = "comment1=cooking%20MCs;userdata=".as_bytes().to_vec();
    input.extend_from_slice(text);
    input.extend_from_slice(";comment2=%20like%20a%20pound%20of%20bacon".as_bytes());
    input
}

fn pad(text: Vec<u8>, length: u8) -> Vec<u8>{
    let mut padded = text.clone();
    let mut i = 1;
    while (length as i8*i-text.len() as i8) <0 {
        i += 1;
    }
    let padding = length*i as u8-text.len() as u8;

    for i in text.len()..(padding as usize +text.len()){
        padded.push(padding);
    }
    padded
}

fn get_text_from_file() -> Vec<u8>{
    let mut file = match File::open("/home/zeegomo/Documents/hack/cryptopals/set2/challenge2/src/data.txt") {
        Ok(file) => file,
        Err(_) => panic!("no such file"),
    };
    let mut text = String::new();
    file.read_to_string(&mut text)
        .ok()
        .expect("failed to read!");
    text.as_bytes().to_vec()
    //base64::decode(&text).unwrap()
}

fn byte_xor(byte1: &[u8], byte2: &[u8]) -> Vec<u8> {
    if byte1.len() != byte2.len() {
        panic!("INCOMPATIBLE SIZES");
    }
    let mut xor = Vec::new();
    for i in 0..byte1.len() {
        xor.push(byte1[i] ^ byte2[i]);
    }
    xor
}

fn aes_cbc(text: Vec<u8>, iv: &[u8], key: &[u8]) ->Vec<u8>{

    let mut decrypter  = Crypter::new(
        Cipher::aes_128_ecb(),
        Mode::Decrypt,
        key,
        None,
    ).unwrap();


    let mut last = iv;
    let mut decrypted = vec![0;32];
    let mut plaintext = Vec::new();
    let mut xored = Vec::new();
    let mut count = 0;
    for i in 0..text.len()/16{
        //println!("{}",i);

        decrypter.update(&text[16*i..16*i+16],&mut decrypted);
        //println!("finalize error");
        //decrypter.finalize(&mut decrypted[count..]).unwrap();
        //decrypted = aes_ebc(&text[16*i..16*i+16],key);
        if i>0 {xored = byte_xor(&decrypted[16..],last);}
            else {xored = byte_xor(&decrypted[0..16],last);}
        //println!("text: {:?}",&text[16*i..16*i+16]);
        //println!("last: {:?}",last);
        //println!("decrypted: {:?}",decrypted);
        //println!("xored: {:?}",xored);
        last = &text[16*i..16*i+16];
        for z in 0..16 as usize{
            plaintext.push(xored[z]);
        }

    }
    plaintext
}

fn aes_cbc_encrypt(text: &[u8], iv: &[u8], key: &[u8]) -> Vec<u8>{
    let mut decrypter  = Crypter::new(
        Cipher::aes_128_ecb(),
        Mode::Encrypt,
        key,
        None,
    ).unwrap();
    let mut input = pad((&text).to_vec(),16);
    let mut ciphertext = Vec::new();
    let mut last = iv.to_vec();
    let mut encrypted = vec![0;32];
    let mut xored = Vec::new();

    for i in 0..input.len()/16{

        //println!("{}",i);
        xored = byte_xor(&input[16*i..16*i+16],&last);
        //println!("input size: {}",input.len());
        decrypter.update(&xored,&mut encrypted);
        //println!("encrypted: {:?}, len:{}",encrypted, encrypted.len());


        last.clear();
        for z in 0..16 as usize{

            ciphertext.push(encrypted[z]);
            last.push(encrypted[z]);

        }

    }
    ciphertext

}
/*
fn aes_ebc(block: &[u8], key: &[u8]) -> Vec<u8>{

    let cipher = Cipher::aes_128_ecb();
    let plaintext =  decrypt(
        cipher,
        &key,
        None,
        &block
    ).expect("AES_ECB error");
    plaintext


}*/