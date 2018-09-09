extern crate num_bigint;
extern crate num_traits;
extern crate hex;
extern crate rand;
extern crate gmp;

use num_bigint::BigUint;
use num_bigint::*;
use num_traits::*;
use num_bigint::Sign::*;
use rand::Rng;
use gmp::mpz::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use std::hash::Hash;

fn main() {

    //parameters initialization
    let mut rng = rand::thread_rng();
    let p_buff = "ffffffffffffffffc90fdaa22168c234c4c6628b80dc1cd129024e088a67cc74020bbea63b139b22514a08798e3404ddef9519b3cd3a431b302b0a6df25f14374fe1356d6d51c245e485b576625e7ec6f44c42e9a637ed6b0bff5cb6f406b7edee386bfb5a899fa5ae9f24117c4b1fe649286651ece45b3dc2007cb8a163bf0598da48361c55d39a69163fa8fd24cf5f83655d23dca3ad961c62f356208552bb9ed529077096966d670c354e4abc9804f1746c08ca237327ffffffffffffffff";
    let N = Mpz::from_str_radix(&p_buff, 16).unwrap();
    let g = Mpz::from(2 as u32);
    let k = Mpz::from(3 as u32);
    //BigInt::from(3).modpow();
    //let rem = BigInt::from_biguint(Plus,rng.gen_biguint(1000000));
    let I = b"person";
    let pass = b"password1234";
    let salt = rng.gen::<u64>();
    let mut digest = to_bytes(salt);
    digest.extend_from_slice(pass);
    let x: u64 = hash(&digest);
    let v = g.powm(&Mpz::from(x), &N);
    //for i in 0..10000{
    println!("1. client send username and public A");
    let a = Mpz::from_str_radix(&rng.gen_biguint(1024).to_str_radix(16),16).unwrap();
    let A = N.clone()*N.clone() /*g.powm(&a, &N)*/;
    println!("A: {}", A);

    println!("2. server send user's salt and public B");
    let b =  Mpz::from_str_radix(&rng.gen_biguint(1024).to_str_radix(16),16).unwrap();
    let B = (k.clone() * v.clone() + g.powm(&b, &N))%N.clone();
    println!("B: {}", B);

    println!("3. client and server computes session key");
    digest = A.to_str_radix(10).into_bytes();
    digest.extend_from_slice(&B.to_str_radix(16).into_bytes());
    let u: u64 = hash(&digest);
    println!("u: {:x}", u);

    println!("4. client computes session key");
    let mut base = B - k.clone() * g.powm(&Mpz::from(x), &N);
    let expo = a + Mpz::from(u) * Mpz::from(x);
    //println!("base: {:x}",base);
    //println!("exp: {:x}",expo);
    let S_c = (base).powm(&expo, &N);
    let K_c = hash(&S_c.to_str_radix(16).into_bytes());
    println!("S_c: {}", S_c);

    println!("5. server computes session key");
    let S_s = (A * v.powm(&Mpz::from(u), &N)).powm(&b, &N);
    let K_s = hash(&S_s.to_str_radix(16).into_bytes());
    println!("S_s: {}", S_s);

    println!("6. clients sends proof of session key");
    digest = to_bytes(salt);
    digest.extend_from_slice(&to_bytes(K_c));
    let val = hash(&digest);
    println!("val: {}", val);

    println!("7. server sends proof of session key");
    digest = to_bytes(salt);
    digest.extend_from_slice(&to_bytes(K_s));
    let val1 = hash(&digest);
    println!("val: {}", val1);
    //assert_eq!(val,val1);
    //}
}

fn to_bytes(n: u64) -> Vec<u8> {
    let mut byte = Vec::new();
    byte.push((n >> 56) as u8);
    byte.push((n >> 48) as u8);
    byte.push((n >> 40) as u8);
    byte.push((n >> 32) as u8);
    byte.push((n >> 24) as u8);
    byte.push((n >> 16) as u8);
    byte.push((n >> 8) as u8);
    byte.push(n as u8);
    byte
}

pub fn hash(input: &[u8]) -> u64 {
    let mut s = DefaultHasher::new();
    input.hash(&mut s);
    s.finish()
}

