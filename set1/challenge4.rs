mod utils;

use utils::*;

fn main() {
    let lines = lines_from_file("data.txt");
    let mut round_guess = (0, 0);
    let mut best_guess = (Vec::new(), 0, 0);
    for i in 0..lines.len() {
        let vect = hex_to_bytes(&lines[i]);
        round_guess = guess(&vect);


        if round_guess.1 > best_guess.2 {
            best_guess.0 = vect;
            best_guess.1 = round_guess.0;
            best_guess.2 = round_guess.1;
        }
    }
    match String::from_utf8(single_byte_xor(&best_guess.0, best_guess.1)) {
        Ok(n) => {println!("probable match at: {}", n); println!("score: {}", best_guess.2)},
        Err(_) => {
            println!("Error formatting code");
            println!("{:?}", &best_guess.0)
        }
    };
}


fn guess(vect: &[u8]) -> (u8, u32) {
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
