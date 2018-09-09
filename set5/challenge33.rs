extern crate num_bigint;
extern crate num_traits;
extern crate hex;
extern crate rand;

use num_bigint::{BigUint};
use num_bigint::*;
use num_traits::*;
use num_bigint::Sign::*;

fn main() {
    let mut rng = rand::thread_rng();

    let p_buff = "ffffffffffffffffc90fdaa22168c234c4c6628b80dc1cd129024e088a67cc74020bbea63b139b22514a08798e3404ddef9519b3cd3a431b302b0a6df25f14374fe1356d6d51c245e485b576625e7ec6f44c42e9a637ed6b0bff5cb6f406b7edee386bfb5a899fa5ae9f24117c4b1fe649286651ece45b3dc2007cb8a163bf0598da48361c55d39a69163fa8fd24cf5f83655d23dca3ad961c62f356208552bb9ed529077096966d670c354e4abc9804f1746c08ca237327ffffffffffffffff";
    let mut p_buff_u8 = Vec::new();
    for i in 0..p_buff.len(){
        p_buff_u8.push(u8::from_str_radix(&p_buff[i..i+1],16).unwrap());
    }

    let p = BigInt::from_radix_be(Plus,&p_buff_u8,16).unwrap();

    let g = BigInt::from(2 as u32);

    let mut a = BigInt::one();
    let mut b = BigInt::one();
    let mut A = BigInt::one();
    let mut B = BigInt::one();


    while A == BigInt::one() || B == BigInt::one() {
        a = rng.gen_bigint(1000); // rand
        b = rng.gen_bigint(1000);


        A = modexp(&g, &a, &p);
        B = modexp(&g, &b, &p);
    }
    println!("A: {}",A);
    println!("B: {}",B);

    let s1 = modexp(&p,&b,&p);
    let s2 = modexp(&p,&a,&p);
    //let d = BigInt::from(12 as u32);
    //let t = BigInt::from(4 as u32);
    //let y = BigInt::from(21 as u32);
    println!("s1: {}",s1);
    println!("s2: {}",s2);
}



fn modexp(b: &BigInt, e: &BigInt, m: &BigInt) -> BigInt{
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