pub struct MersenneTwister {
    pub mt: [u32; 624],
    pub index: u32,
}

impl MersenneTwister {
    pub fn new_from_seed(seed: u32) -> MersenneTwister {
        let mut mersenne = MersenneTwister { mt: [0; 624], index: 0 };
        let mut y: u64 = 0;
        mersenne.mt[0] = seed & 0xffffffff;
        for i in 1..624 {
            y = 69069 as u64 * mersenne.mt[i - 1] as u64;
            mersenne.mt[i] = (y & 0xffffffff as u64) as u32;
        }
        mersenne
    }


    pub fn new_from_state(state: &[u32]) -> MersenneTwister {
        let mut newstate = [0; 624];
        for i in 0..624 {
            newstate[i] = state[i];
        }
        MersenneTwister { mt: newstate, index: 0 }
    }

    fn permute_state(&mut self) {
        let n: u32 = 624;
        let m: u32 = 397;

        const UP_MASK: u32 = 0x80000000;
        const LW_MASK: u32 = 0x7fffffff;

        let mut y = 0;
        let mag01 = [0, 0x9908b0df];

        for i in 0..227 {
            y = (self.mt[i] & UP_MASK) | (self.mt[i + 1] & LW_MASK);
            self.mt[i] = self.mt[i + m as usize] ^ (y >> 1) ^ mag01[(y & 0x1) as usize];
        }

        for i in 227..623 {
            y = (self.mt[i] & UP_MASK) | (self.mt[i + 1] & LW_MASK);
            self.mt[i] = self.mt[i + m as usize - n as usize] ^ (y >> 1) ^ mag01[(y & 0x1) as usize];
        }

        y = (self.mt[n as usize - 1] & UP_MASK) | (self.mt[0] & LW_MASK);
        self.mt[n as usize - 1] = self.mt[m as usize - 1] ^ (y >> 1) ^ mag01[(y & 0x1) as usize];
    }


    pub fn clone(out: &[u32]) -> MersenneTwister {
        if out.len() != 624 {
            panic!("Output len error");
        }

        let mut state = [0; 624];

        for i in 0..624 {
            state[i] = untemper(out[i]);
        }
        MersenneTwister::new_from_state(&state)
    }

    pub fn gen_rand(&mut self) -> u32 {
        if self.index == 0 {
            self.permute_state();
        }

        let y = temper(self.mt[self.index as usize]);
        self.index = (self.index + 1) % 624;
        y
    }
}

pub fn temper(mut y: u32) -> u32 {
    y ^= y >> 11;
    y ^= (y << 7) & 0x9d2c5680;
    y ^= (y << 15) & 0xefc60000;
    y ^= (y >> 18);

    y
}


pub fn untemper(y: u32) -> u32 {
    let w = 32;
    let n = 624;
    let m = 397;
    let r = 31;
    let a: u32 = 0x9908b0df;
    let u = 11;
    let d: u32 = 0xffffffff;
    let s = 7;
    let b: u32 = 0x9d2c5680;
    let t = 15;
    let c: u32 = 0xefc60000;
    let l = 18;
    let x_3 = recover_xor(y, l);
    let x_2 = recover_xor_and_c(x_3, t);
    let x_1 = recover_xor_and_b(x_2, s);
    let x_0 = recover_xor(x_1, u);
    x_0
}

pub fn recover_xor_and_c(y: u32, shift: u8) -> u32 {
    let y_bits = convert_to_bits(y as u32);

    let mut x = [0; 32];
    let mut y = [0; 32];

    for i in 15..32 {
        x[i] = y_bits[i];
    }
    for i in 13..15 {
        x[i] = y_bits[i] ^ x[i + shift as usize];
    }
    for i in 10..13 {
        x[i] = y_bits[i];
    }
    for i in 4..10 {
        x[i] = y_bits[i] ^ x[i + shift as usize];
    }
    x[3] = y_bits[3];
    for i in 0..3 {
        x[i] = y_bits[i] ^ x[i + shift as usize];
    }

    bits_to_u32(&x)
}

pub fn recover_xor_and_b(y: u32, shift: u8) -> u32 {
    let y_bits = convert_to_bits(y as u32);

    let mut x = [0; 32];
    let mut y = [0; 32];

    for i in 25..32 {
        x[i] = y_bits[i];
        y[i] = y_bits[i];
    }
    x[24] = y_bits[24] ^ y[24 + shift as usize];
    y[24] = x[24];

    x[23] = y_bits[23];
    y[23] = y_bits[23];

    for i in 21..23 {
        x[i] = y_bits[i] ^ y[i + shift as usize];
        y[i] = x[i];
    }
    x[20] = y_bits[20];
    y[20] = y_bits[20];

    x[19] = y_bits[19] ^ y[19 + shift as usize];
    y[19] = x[19];

    x[18] = y_bits[18];
    y[18] = y_bits[18];

    x[17] = y_bits[17] ^ y[17 + shift as usize];
    y[17] = x[17];

    for i in 14..17 {
        x[i] = y_bits[i];
        y[i] = y_bits[i];
    }

    for i in 12..14 {
        x[i] = y_bits[i] ^ y[i + shift as usize];
        y[i] = x[i];
    }

    x[11] = y_bits[11];
    y[11] = y_bits[11];

    x[10] = y_bits[10] ^ y[10 + shift as usize];
    y[10] = x[10];

    for i in 8..10 {
        x[i] = y_bits[i];
        y[i] = y_bits[i];
    }

    x[7] = y_bits[7] ^ y[7 + shift as usize];
    y[7] = x[7];

    x[6] = y_bits[6];
    y[6] = y_bits[6];

    for i in 3..6 {
        x[i] = y_bits[i] ^ y[i + shift as usize];
        y[i] = x[i];
    }

    for i in 1..3 {
        x[i] = y_bits[i];
        y[i] = y_bits[i];
    }

    x[0] = y_bits[0] ^ y[0 + shift as usize];
    y[0] = x[0];

    bits_to_u32(&x)
}


pub fn bits_to_u32(bits: &[u8]) -> u32 {
    let two: u32 = 2;
    let mut x = 0;
    for i in 0..bits.len() {
        x += two.pow(bits.len() as u32 - 1 - i as u32) * bits[i] as u32;
    }
    x
}

pub fn convert_to_bits(a: u32) -> Vec<u8> {
    let mut x = a;
    let mut vec = Vec::new();
    while x > 0 {
        vec.insert(0, (x % 2) as u8);
        x /= 2;
    }
    while vec.len() < 32 {
        vec.insert(0, 0);
    }

    vec
}


pub fn recover_xor(y: u32, shift: u8) -> u32 {
    let y_bits = convert_to_bits(y as u32);
    let mut x = Vec::new();
    let mut y = Vec::new();
    //println!("y: {:?}", y_bits);
    //println!("x: {:?}", x);
    for i in 0..shift as usize {
        x.push(y_bits[i]);
        y.push(y_bits[i]);
    }
    //println!("x: {:?}", x);

    for i in shift as usize..y_bits.len() {
        x.push(y_bits[i] ^ y[i - shift as usize]);
        y.push(x[i]);
    }
    //println!("x: {:?}", x);


    bits_to_u32(&x)
}
