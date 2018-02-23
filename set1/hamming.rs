use utils::*;

pub fn guess_key_lenght(min: u32, max: u32, encrypted: &[u8]) -> (u32,f32, u32,f32, u32, f32) {
    let mut best = (0,10.0,0, 10.0,0, 10.0);
    let mut round_guess: f32 = 0.0;
    for i in (min as usize)..(max + 1) as usize {

        if 2 * i*20 < encrypted.len() {
            round_guess = normalized_hamming_distance(&encrypted[0..i*20], &encrypted[i..2 * i*20]);
            if round_guess < best.1 {
                best.4 = best.2;
                best.2 = best.0;
                best.0 = i as u32;
                best.5 = best.3;
                best.3 = best.1;
                best.1 = round_guess;
            } else if round_guess < best.3 {
                best.4 = best.2;
                best.2 = i as u32;
                best.5 = best.3;
                best.3 = round_guess;
            } else if round_guess < best.5 {
                best.4 = i as u32;
                best.5 = round_guess;
            }
       }
    }
    best
}

pub fn gen_keys(block: Vec<Vec<u8>>) -> Vec<u8>{
    let mut keys = Vec::new();
    for i in 0..block.len(){
        keys.push(guess_single_xor_key(&block[i]).0);
    }
    keys
}

pub fn normalized_hamming_distance(block1: &[u8], block2: &[u8]) -> f32 {
    let mut xor = byte_xor(block1, block2);
    let distance = count_bit(&mut xor);
    distance / (block1.len() as f32)
}

pub fn encrypt_repeating_xor(plaintext: &[u8], key: &[u8]) ->Vec<u8>{
    let keystream = gen_keystream_from_repeating_xor(key, plaintext.len());
    byte_xor(plaintext,&keystream)
}


pub fn gen_keystream_from_repeating_xor(key: &[u8], len: usize) -> Vec<u8>{
    let mut keystream = key.to_vec();
    for i in 0..len/key.len()+1{
        for i in 0..key.len(){
            keystream.push(key[i]);
        }
    }
    keystream
}
pub fn get_blocks(text: &[u8], key: u32) -> Vec<Vec<u8>>{
    let mut blocks = Vec::new();
    for i in 0..key{
        blocks.push(Vec::new());
    }
    for i in 0..(text.len()/key as usize)+1{
        for z in 0..key as usize{
            if i*key as usize +z < text.len(){
            blocks[z].push(text[i*key as usize+z]);
            }
        }
    }

    blocks
}

pub fn guess_single_xor_key(vect: &[u8]) -> (u8,u32) {
    let mut round_key = 0;
    let mut round_score = 0;
    let mut best = (0, 0);
    for i in 0..127 {
        let xored = single_byte_xor(&vect, round_key);
        round_score = get_score(&xored);

        if best.1 < round_score {
            best.0 = round_key;
            best.1 = round_score;
        }
        round_key += 1;
    }

    best
}

