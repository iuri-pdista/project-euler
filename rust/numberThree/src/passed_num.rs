use crate::primes::is_prime;
use crate::primes::prime_listing;

pub enum IsPrime{
    Prime,
    NotPrime,
}

pub fn prime_decision( num: usize ) -> IsPrime{
    if is_prime(num) { IsPrime::Prime } else { IsPrime::NotPrime }
}

pub fn _largest_prime_factor(num: usize, ceiling_value: usize)->usize{
    let _list: Vec<usize> = prime_listing(ceiling_value);
    let mut vec_size = _list.len();
    vec_size -= 1;
    loop{
        if (num % _list[vec_size]) == 0 {
            let _largest_prime_factor = _list[vec_size];
        } else {
            vec_size -= 1;
        };
    }
}