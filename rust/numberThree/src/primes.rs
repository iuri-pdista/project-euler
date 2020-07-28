pub fn prime_listing(ceiling_value: usize)->Vec<usize>{
    let mut prime_list = Vec::with_capacity(ceiling_value);
    prime_list.push(1);
    for count in 0..ceiling_value{
        if is_prime(count) {prime_list.push(count)};
    };
    prime_list
}

pub fn is_prime(num: usize)->bool{
    let is_prime: bool;
    is_prime = prime_decision(num);
    if is_prime { true } else { false }
}

fn prime_decision(num:usize)->bool{
    let mut divider_count = 0;
    for i in 1..num{
        if (num % i) == 0{
            divider_count += 1;
        };
    };
    if divider_count == 1 { true } else { false }
}