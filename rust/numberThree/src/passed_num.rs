use crate::primes::is_prime;
use crate::primes::prime_listing;

pub enum IsPrime{
    Prime,
    NotPrime,
}

pub fn prime_decision( num: usize ) -> IsPrime{
    if is_prime(num) { IsPrime::Prime } else { IsPrime::NotPrime }
}

pub fn _largest_prime_factor(num: usize, ceiling_value: usize)-> usize{
    let _list: Vec<usize> = prime_listing(ceiling_value);
    let mut prime_factor: usize = 0;
    for count in _list{
        if (num % count) == 0{
            prime_factor = count;
        };
    }
    prime_factor
}