#![warn(dead_code)]

// use std::io;
// use std::io::prelude::*;

mod primes;
mod passed_num;
use crate::passed_num::IsPrime;

fn main() {  
    let _list: Vec<usize> = primes::prime_listing(100);
    let decision: IsPrime= passed_num::prime_decision(5);
    match decision{
        IsPrime::Prime=> {println!("The biggest prime factor is alo")},
        IsPrime::NotPrime=> {println!("ola")},
    }
}