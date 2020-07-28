#![warn(dead_code)]

use std::io;
use std::io::prelude::*;

mod primes;
mod passed_num;
use crate::passed_num::IsPrime;

fn main() {  
    let mut num_string = String::new();
    io::stdin()
        .read_line(&mut num_string)
        .expect("Failed to read number");

    let mut _num: usize = num_string.trim_end().parse().unwrap();
    let _list: Vec<usize> = primes::prime_listing(100);
    let decision: IsPrime= passed_num::prime_decision(_num);
    match decision{
        IsPrime::Prime=> {println!("The biggest prime factor is {}", _num)},
        IsPrime::NotPrime=> {println!("ola")},
    }
}