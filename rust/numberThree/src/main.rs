#![warn(dead_code)]

use std::io;
// use std::io::prelude::*;

mod primes;
mod passed_num;
use crate::passed_num::IsPrime;

fn main() {  
    let mut num_string = String::new();
    io::stdin()
        .read_line(&mut num_string)
        .expect("Failed to read number");

    let mut _num: usize = num_string.trim_end().parse().unwrap();
    let ceiling_value = _num * 2;
    let decision: IsPrime= passed_num::prime_decision(_num);
    match decision{
        IsPrime::Prime=> {println!("The largest prime factor of {number} is {number}", number= _num)},
        IsPrime::NotPrime=> {
            println!("The largest prime factor of {} is {}", _num, passed_num::_largest_prime_factor(_num, ceiling_value));
        },
    }
}