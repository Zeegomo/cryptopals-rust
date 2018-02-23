mod mersenne;

extern crate rand;

use std::{thread, time};
use std::time::{SystemTime, UNIX_EPOCH};
use mersenne::MersenneTwister;
use rand::Rng;

fn main() {

    let mut rng = rand::thread_rng();
    let mut time = time::Duration::from_millis(rng.gen_range(40, 100));

    thread::sleep(time);

    let mut randmt = 0;
    let mut start = SystemTime::now();
    let mut since_the_epoch = start.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

        time = time::Duration::from_millis(rng.gen_range(4000, 10000));
        thread::sleep(time);

        start = SystemTime::now();
        since_the_epoch = start.duration_since(UNIX_EPOCH).expect("warp");
        println!("time: {}",since_the_epoch.as_secs());

        let mut rand = MersenneTwister::new_from_seed(since_the_epoch.as_secs() as u32);

        thread::sleep(time);
        randmt = rand.gen_rand();
        println!("rand: {}", randmt);


        start = SystemTime::now();
        since_the_epoch = start.duration_since(UNIX_EPOCH).expect("warp");
        guess_seed(randmt,since_the_epoch.as_secs() as u32 );

}


fn guess_seed(rand: u32, time: u32){
    println!("starting seed search...");
    let mut randmt = MersenneTwister::new_from_seed(0);
    for i in time-100000..time{
        randmt = MersenneTwister::new_from_seed(i);
        if rand == randmt.gen_rand(){
            println!("seed: {}", i);
            break;
        }
    }
}
