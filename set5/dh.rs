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

pub struct Dh {
    pub private_key: BigInt,
    pub public_key: BigInt,
    pub other_public: BigInt,
    pub key: BigInt,
    pub p: BigInt,
    pub g: BigInt,
}

pub struct DhMitm {
    pub a_public_key: BigInt,
    pub b_public_key: BigInt,
    pub p: BigInt,
    pub g: BigInt,
    pub key: BigInt,
}

impl Dh {
    pub fn new() -> Dh {
        Dh { private_key: BigInt::one(), public_key: BigInt::one(), other_public: BigInt::one(), key: BigInt::one(), p: BigInt::one(), g: BigInt::one() }
    }

    pub fn start_exchange(&mut self) -> (BigInt, BigInt, BigInt) {
        let mut rng = rand::thread_rng();

        let p_buff = "ffffffffffffffffc90fdaa22168c234c4c6628b80dc1cd129024e088a67cc74020bbea63b139b22514a08798e3404ddef9519b3cd3a431b302b0a6df25f14374fe1356d6d51c245e485b576625e7ec6f44c42e9a637ed6b0bff5cb6f406b7edee386bfb5a899fa5ae9f24117c4b1fe649286651ece45b3dc2007cb8a163bf0598da48361c55d39a69163fa8fd24cf5f83655d23dca3ad961c62f356208552bb9ed529077096966d670c354e4abc9804f1746c08ca237327ffffffffffffffff";
        let mut p_buff_u8 = Vec::new();
        for i in 0..p_buff.len() {
            p_buff_u8.push(u8::from_str_radix(&p_buff[i..i + 1], 16).unwrap());
        }
        self.p = BigInt::from_radix_be(Plus, &p_buff_u8, 16).unwrap();
        self.g = BigInt::from(2 as u32);

        while self.public_key == BigInt::one() {
            self.private_key = rng.gen_bigint(1000); // rand
            self.public_key = modexp(&self.g, &self.private_key, &self.p);
        }
        (self.p.clone(), self.g.clone(), self.public_key.clone())
    }

    pub fn reply(&mut self, p: BigInt, g: BigInt, A: BigInt) -> BigInt {
        let mut rng = rand::thread_rng();

        self.p = p;
        self.g = g;
        self.other_public = A;
        while self.public_key == BigInt::one() {
            self.private_key = rng.gen_bigint(1000); // rand
            self.public_key = modexp(&self.g, &self.private_key, &self.p);
        }
        self.key = modexp(&self.other_public, &self.private_key, &self.p);
        self.public_key.clone()
    }

    pub fn other_key_update(&mut self, other: BigInt) {
        self.other_public = other;
        self.key = modexp(&self.other_public, &self.private_key, &self.p)
    }

    pub fn aes_cbc_from_key(&mut self, message: & [u8]) -> Vec<u8>{
        let iv = gen_rand_key();
        let cipher = Cipher::aes_128_ecb();
        let mut key = self.key.to_bytes_be().1;
        while key.len()<16{
            key.push(0);
        }
        let mut encrypt_vec = encrypt(
            cipher,
            &key[0..16],
            Some(&iv),
            & message,
            ).expect("error");
            println!("len: {}",encrypt_vec.len());
            encrypt_vec.extend_from_slice(&iv);
            encrypt_vec
    }
}

impl  DhMitm{
    pub fn new() -> DhMitm{
        DhMitm{a_public_key: BigInt::one(), b_public_key: BigInt::one(),p: BigInt::one(), g: BigInt::one(), key: BigInt::one()}
    }

    pub fn forward_start(&mut self,p: BigInt, g: BigInt, A: BigInt) -> (BigInt,BigInt,BigInt){
        self.p = p;
        self.g = g;
        self.a_public_key = A;
        (self.p.clone(),self.g.clone(),self.p.clone())
    }

    pub fn forward_reply(&mut self, B: BigInt) -> BigInt{
        self.b_public_key = B;
        self.key = BigInt::zero();
        self.p.clone()
    }

    pub fn crack_message(&self,message: &[u8]) -> Vec<u8>{
        let mut iv = &message[message.len()-16..];
        println!("len: {}",message.len());
        let mut plaintext = &message[..message.len()-16];
        println!("len: {}",plaintext.len());
        let mut key = self.key.to_bytes_be().1;
        while key.len()<16{
            key.push(0);
        }
        let cipher = Cipher::aes_128_ecb();
        decrypt(
            cipher,
            &key[0..16],
            Some(&iv),
            &plaintext,
        ).expect("error")

    }
}

fn gen_rand_key() -> Vec<u8>{
    let mut rng = rand::thread_rng();
    let mut vec = Vec::new();
    for i in 0..16{
        vec.push(rng.gen::<u8>());
    }
    vec
}

fn modexp(b: &BigInt, e: &BigInt, m: &BigInt) -> BigInt {
    let mut b = b.clone();
    let mut e = e.clone();
    let mut res = BigInt::one();
    let two = 2.to_bigint().unwrap();

    while e > BigInt::zero() {
        if e.clone() % two.clone() == BigInt::one() {
            res = (res.clone() * b.clone()) % m.clone();
        }

        b = (b.clone() * b.clone()) % m.clone();
        e = e.clone() / two.clone();
    }

    res % m
}