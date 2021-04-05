pub fn nth(n: u32) -> u32 {

    let mut vector = Vec::new();
    let mut number = 2;
    let mut is_not_prime;
    let mut num_primes = 0;

    if n > 0{
        while num_primes < n {
            is_not_prime = false;
            number = number +1;
            let mut k = 2;
            while k < number && is_not_prime == false {
                if number%k == 0 {
                    is_not_prime = true;
                }
                k = k + 1;
            }
            if is_not_prime == false {
                vector.push(number);
                num_primes = num_primes + 1;
            }
            
        }

        let prime = vector[vector.len()-1];
        return prime as u32;
    }
    else if n == 0 {
        return 2;
    }
    else {
        return 0;
    }
}
