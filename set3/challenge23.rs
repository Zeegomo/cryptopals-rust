mod mersenne;
use mersenne::{MersenneTwister};



fn main() {
    let mut rand1 = MersenneTwister::new_from_seed(44332);
    let mut out = [0;624];

    for i in 0..624{
        out[i] = rand1.gen_rand();
    }

    let mut rand2 = MersenneTwister::clone(&out);
    for i in 0..100000{
        assert_eq!(rand1.gen_rand(),rand2.gen_rand());
    }

    println!("successful cloning");
}