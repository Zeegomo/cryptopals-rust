fn main() {
    let text = "YELLOW SUBMARINE";
    println!("{:?}",pad(text.as_bytes().to_vec(),10));
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