mod aes;

fn main() {
    let mut padding = aes::pad("1234567891234".as_bytes().to_vec(),16);
    println!("{:?}",padding);
    padding[13] = 2;
    validate_padding(&padding);
}

fn validate_padding(text: &[u8]) {
    let len = text.len();
    let last = text[len-1];
    println!("len: {}, last: {}",len,last);
    if last as usize >= len {
        panic!("INVALID PADDING");
    }
    for i in len-last as usize..len{
        if text[i] != last{
            panic!("INVALID PADDING");
        }
    }
}