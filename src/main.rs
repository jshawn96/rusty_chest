extern crate rand;


use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

fn main() {
    let rand_str: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .collect();

    println!("{}", rand_str);
}