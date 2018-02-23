extern crate base64;

use std::env;
use base64::encode;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        let base64 = hex_to_base64(&args[1]);
        println!("{}",&args[1]);
        println!("{}", base64);
    }
}

fn hex_to_base64(hex: &str) -> String {
    let mut bytes = Vec::new();
    for i in 0..(hex.len()/2){
        let res = u8::from_str_radix(&hex[2*i..2*i+2],16);
        match res {
            Ok(n) => bytes.push(n),
            Err(e) => println!("Error: {}", e),
        }
    }

    encode(&bytes)
}

