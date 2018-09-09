mod md4;

use md4::*;
use std::u32;
fn main() {
    let val = "Rosetta Code";
    println!("md4(\"{}\") = {}", val, digest_to_str(&md4(val)));
    println!("secret: {}",key_md4(val.as_bytes()));
    println!("craft: {}",craft_signature());
    println!("append: {}",key_md4())
}

fn pad(input: &[u8]) -> Vec<u32>{
    let mut bytes = input.to_vec();
    let initial_bit_len = (bytes.len() << 3) as u64;

    bytes.push(0x80_u8);
    while (bytes.len() % 64) != 56 {
        bytes.push(0_u8);
    }

    // Everything after this operates on 32-bit words, so reinterpret the buffer.
    let mut w = convert_byte_vec_to_u32(bytes);

    // Step 2. Append length
    // A 64-bit representation of b (the length of the message before the padding bits were added)
    // is appended to the result of the previous step, low-order bytes first.
    w.push(initial_bit_len as u32); // Push low-order bytes first
    w.push((initial_bit_len >> 32) as u32);

    w
}

fn key_md4(str : &[u8]) -> String{
    let mut val = vec![78,88,97];
    val.extend_from_slice(str);
    digest_to_str(&md4(val))
}

fn craft_signature() -> String{
    let val = b";admin=true";
    let a = md4(b"Rosetta Code".to_vec());
    let mut md4_state = [0;4];
    let mut padded_message = &;
    for i in 0..100{
        md4_state = md4_from_values(val.to_vec(),u32::to_le(a[0]),u32::to_le(a[1]),u32::to_le(a[2]),u32::to_le(a[3]),pad(i));
    }
    digest_to_str(&md4_state)
}