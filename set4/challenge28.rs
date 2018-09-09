extern crate sha1;


fn main() {
    println!("sha1: {:?}",sha1_keyed(b"test"));

    println!("sha1: {:?}",sha1(b"test"));
}

fn sha1_keyed(message: &[u8]) -> [u8;20]{
    let mut sha1 = sha1::Sha1::new();
    let mut key = vec![0,22,223];
    key.extend_from_slice(message);
    sha1.update(&key);
    sha1.digest().bytes()
}

fn sha1(message: &[u8]) -> [u8;20]{
    let mut sha1 = sha1::Sha1::new();
    sha1.update(message);
    sha1.digest().bytes()
}