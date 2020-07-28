// use std::io;
// use std::io::prelude::*;
// use std::env;

mod primes;

fn main() {
    let list: Vec<usize>= primes::prime_listing(10);
    println!("{:?}",list);
}