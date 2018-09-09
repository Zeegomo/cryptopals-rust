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
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};


pub struct Server {
    pub salt: BigInt,
    pub x: u64,
    pub v: BigInt,
    pub N: BigInt,
    pub g: BigInt,
    pub k: BigInt,
    pub I: Vec<u8>,
    pub P: Vec<u8>,
    pub B: BigInt,
    pub u: u64,
    pub S: BigInt,
    pub K: u64,
    pub b: BigInt,
    pub pass: Vec<u8>,
    pub A: BigInt,
}

pub struct Client {
    pub g: BigInt,
    pub N: BigInt,
    pub k: BigInt,
    pub I: Vec<u8>,
    pub B: BigInt,
    pub A: BigInt,
    pub a: BigInt,
    pub x: u64,
    pub S: BigInt,
    pub K: u64,
    pub u: u64,
    pub salt: BigInt,
    pub pass: Vec<u8>,
}

impl Server {
    pub fn new() -> Server {
        let mut server = Server {
            salt: BigInt::zero(),
            x: 0,
            v: BigInt::zero(),
            N: BigInt::zero(),
            g: BigInt::zero(),
            k: BigInt::zero(),
            I: Vec::new(),
            P: Vec::new(),
            B: BigInt::zero(),
            A: BigInt::zero(),
            u: 0,
            S: BigInt::zero(),
            K: 0,
            b: BigInt::zero(),
            pass: Vec::new(),
        };
        server.init();
        server
    }

    fn init(&mut self) {
        let p_buff = "ffffffffffffffffc90fdaa22168c234c4c6628b80dc1cd129024e088a67cc74020bbea63b139b22514a08798e3404ddef9519b3cd3a431b302b0a6df25f14374fe1356d6d51c245e485b576625e7ec6f44c42e9a637ed6b0bff5cb6f406b7edee386bfb5a899fa5ae9f24117c4b1fe649286651ece45b3dc2007cb8a163bf0598da48361c55d39a69163fa8fd24cf5f83655d23dca3ad961c62f356208552bb9ed529077096966d670c354e4abc9804f1746c08ca237327ffffffffffffffff";
        let mut p_buff_u8 = Vec::new();
        for i in 0..p_buff.len() {
            p_buff_u8.push(u8::from_str_radix(&p_buff[i..i + 1], 16).unwrap());
        }
        self.N = BigInt::from_radix_be(Plus, &p_buff_u8, 16).unwrap();
        self.g = BigInt::from(2 as u32);
        self.k = BigInt::from(3 as u32);
    }
    pub fn start(&mut self) {
        let mut rng = rand::thread_rng();
        self.salt = BigInt::from(Plus, rng.gen_biguint(100));
        let mut digest = self.salt.to_bytes_be().1;
        digest.extend_from_slice(self.pass);
        self.x = hash(&digest);
        self.v = self.g.modpow(&BigInt::from(self.x), self.N);
    }


    pub fn reply_2(&mut self, I: Vec<u8>, A: BigInt) -> (BigInt, BigInt) {
        let mut rng = rand::thread_rng();

        self.I = I;
        self.A = A;
        self.b = BigInt::from_biguint(Plus, rng.gen_biguint(100));
        self.B = self.k * v + self.g.modpow(self.b, self.N);
        let mut digest = A.to_bytes_be().1;
        digest.extend_from_slice(&(self.B.to_bytes_be().1));
        self.u = hash(&digest);
        (self.salt, self.B)
    }

    pub fn reply_3(&mut self, other_k: u64) -> bool{
        let base = self.A*exp(self.v,self.u);
        self.S = base.modpow(self.b,self.N);
        self.K = hash(&(self.S.to_bytes_be().1[0..16]));
        if self.K == other_k{
            true
        }else{
            false
        }
    }
}

impl Client {
    pub fn new() -> Client {
        let mut client = Client {
            k: BigInt::zero(),
            g: BigInt::zero(),
            N: BigInt::zero(),
            I: Vec::new(),
            A: BigInt::zero(),
            a: BigInt::zero(),
            x: 0,
            S: BigInt::zero(),
            K: 0,
            B: BigInt::zero(),
            u: 0,
            salt: BigInt::zero(),
            pass: Vec::new(),
        };
        client.init();
        client
    }

    fn init(&mut self) {
        let p_buff = "ffffffffffffffffc90fdaa22168c234c4c6628b80dc1cd129024e088a67cc74020bbea63b139b22514a08798e3404ddef9519b3cd3a431b302b0a6df25f14374fe1356d6d51c245e485b576625e7ec6f44c42e9a637ed6b0bff5cb6f406b7edee386bfb5a899fa5ae9f24117c4b1fe649286651ece45b3dc2007cb8a163bf0598da48361c55d39a69163fa8fd24cf5f83655d23dca3ad961c62f356208552bb9ed529077096966d670c354e4abc9804f1746c08ca237327ffffffffffffffff";
        let mut p_buff_u8 = Vec::new();
        for i in 0..p_buff.len() {
            p_buff_u8.push(u8::from_str_radix(&p_buff[i..i + 1], 16).unwrap());
        }
        self.N = BigInt::from_radix_be(Plus, &p_buff_u8, 16).unwrap();
        self.g = BigInt::from(2 as u32);
        self.k = BigInt::from(3 as u32);
    }

    pub fn start_1(&mut self) -> (Vec<u8>, BigInt) {
        let mut rng = rand::thread_rng();
        self.a = BigInt::from(Plus, rng.gen_biguint(100));
        self.A = self.g.clone().modpow(self.a, self.N);
        self.I = vec![32, 32, 2];
        (self.I, self.A)
    }
    //TODO
    pub fn reply_3(&mut self, salt: BigInt, B: BigInt) -> u64{
        self.B = B;
        self.salt = salt;
        let mut digest = self.A.to_bytes_be().1;
        digest.extend_from_slice(&(B.to_bytes_be().1));
        self.u = hash(&digest);
        digest = salt;
        digest.extend_from_slice(&(self.pass));
        self.x = hash(&digest);
        let base = self.B - self.k * exp(self.g, self.x);
        let exp = self.a + self.u * BigInt::from(self.x);
        self.S = base.modpow(exp,self.N);
        self.K = hash(&(self.S.to_bytes_be().1[0..16]));
        hmac(self.salt.to_bytes_be().1,to_bytes(self.K))
    }
}

fn to_bytes(n: u64) -> Vec<u8>{
    let mut byte = Vec::new();
    byte.push(n>>56 as u8);
    byte.push(n>>48 as u8);
    byte.push(n>>40 as u8);
    byte.push(n>>32 as u8);
    byte.push(n>>24 as u8);
    byte.push(n>>16 as u8);
    byte.push(n>>8 as u8);
    byte.push(n as u8);
    byte
}


pub fn exp(base: BigInt, exp: u64) -> BigInt {
    let mut res = base.clone();
    for i in 0..exp {
        res *= base;
    }
    res
}

pub fn hash(input: &[u8]) -> u64 {
    let mut s = DefaultHasher::new();
    input.hash(&mut s);
    s.finish()
}

pub fn hmac(input: &[u8], key: &[u8]) -> u64{
    let mut s = DefaultHasher::new();
    let mut vec = key.to_vec();
    vec.extend_from_slice(input);
    input.hash(&mut s);
    s.finish()
}

