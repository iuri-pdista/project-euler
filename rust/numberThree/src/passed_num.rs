use crate::primes::is_prime;

pub enum IsPrime{
    Prime,
    NotPrime,
}

pub fn prime_decision( num: usize ) -> IsPrime{
    if is_prime(num) { IsPrime::Prime } else { IsPrime::NotPrime }
}