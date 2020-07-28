pub fn prime_listing(ceiling_value: usize)->Vec<usize>{
    let mut prime_list = Vec::with_capacity(ceiling_value);
    for count in 1..ceiling_value{
        if is_prime(count) {prime_list.push(count)};
    };
    prime_list
}

fn is_prime(num: usize)->bool{
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
    if divider_count == 2 { true } else { false }
}