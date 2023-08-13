// milestone 1
// -----------
pub fn gcd(mut a: i64, mut b: i64) -> i64 {
    a = a.abs();
    b = b.abs();

    loop {
        let remainder = a % b;

        if remainder == 0 {
            return b;
        }

        a = b;
        b = remainder;
    }
}

pub fn lcm(a: i64, b: i64) -> i64 {
    b / gcd(a, b) * a
}


// milestone 2
// -----------
pub fn fast_exp(mut num: i64, mut pow: i64) -> i64 {
    let mut result = 1;

    while pow >= 1 {
        if pow % 2 == 1 {
            result *= num;
        }

        pow /= 2;
        num *= num;
    }

    result
}

pub fn fast_exp_mod(num: i64, pow: i64, modulus: i64) -> i64 {
    fast_exp(num, pow) % modulus
}

// milestone 3
// -----------
pub fn sieve_of_eratosthenes(max: usize) -> Vec<bool> {
    let mut is_primes = [false; 1].repeat(max + 1);

    if max < 2 { return is_primes; }

    is_primes[2] = true;
    for i in (3..=max).step_by(2) {
        is_primes[i] = true;
    }

    for i in (3..=max).step_by(2) {
        if is_primes[i] {
            for j in (i * 2..=max).step_by(i) {
                is_primes[j] = false;
            }
        }
    }

    is_primes
}

pub fn print_sieve(sieve: &mut Vec<bool>) {
    let odd_primes = sieve_to_primes(sieve.clone());
    let nums = odd_primes
        .into_iter()
        .map(|e| e.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    println!("{nums}");
}

pub fn sieve_to_primes(sieve: Vec<bool>) -> Vec<i64> {
    let mut primes = (3..sieve.len())
        .step_by(2)
        .filter(|i| sieve[*i])
        .map(|i| i as i64)
        .collect::<Vec<i64>>();

    if sieve.len() > 2 {
        primes.insert(0, 2);
    }

    primes
}

pub fn print_numbers(primes: &mut Vec<i64>) {
    for prime in primes {
        print!("{prime} ");
    }
    println!("");
}

// milestone 4
// -----------

pub fn find_factors(mut num: i64) -> Vec<i64> {
    let mut factors = Vec::new();

    while num % 2 == 0 {
        factors.push(2);
        num /= 2;
    }

    for n in (3..=num).step_by(2) {
        while num % n == 0 {
            factors.push(n);
            num /= n;
        }
    }

    factors
}

pub fn find_factors_sieve(primes: &[i64], mut num: i64) -> Vec<i64> {
    let mut factors = Vec::new();

    for prime in primes {
        while num % prime == 0 {
            factors.push(*prime);
            num /= prime;
        }
    }

    factors
}

pub fn multiply_vector(nums: &[i64]) -> i64 {
    nums.iter()
        .fold(1, |acc, e| acc * e)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(18, 48), 6);
        assert_eq!(gcd(7, 11), 1);
        assert_eq!(gcd(100, 100), 100);
        assert_eq!(gcd(-48, 18), 6);
        assert_eq!(gcd(48, -18), 6);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(12, 18), 36);
    }

    #[test]
    fn test_fast_exp() {
        assert_eq!(fast_exp(3, 6), 729);
        assert_eq!(fast_exp(8, 9), 134_217_728);
    }

    #[test]
    fn test_fast_exp_mod() {
        assert_eq!(fast_exp_mod(8, 6, 10), 4);
    }

    #[test]
    fn test_sieve_of_eratosthenes() {
        assert_eq!(sieve_of_eratosthenes(1), vec![false, false]);
        assert_eq!(sieve_of_eratosthenes(3), vec![false, false, true, true]);
        assert_eq!(
            sieve_of_eratosthenes(25),
            vec![false,                             // 0
                 false, true, true, false, true,    // 1 - 5
                 false, true, false, false, false,  // 6 - 10
                 true, false, true, false, false,  // 11 - 15
                 false, true, false, true, false,  // 16 - 20
                 false, false, true, false, false]  // 20 - 25
        );
    }

    #[test]
    fn test_find_factors() {
        assert_eq!(find_factors(6), vec![2, 3]);
        assert_eq!(find_factors(7), vec![7]);
        assert_eq!(find_factors(26), vec![2, 13]);
        assert_eq!(find_factors(63), vec![3, 3, 7]);
    }

    #[test]
    fn test_find_factors_sieve() {
        let primes = sieve_to_primes(sieve_of_eratosthenes(50_000_000));

        assert_eq!(find_factors_sieve(&primes, 6), vec![2, 3]);
        assert_eq!(find_factors_sieve(&primes, 7), vec![7]);
        assert_eq!(find_factors_sieve(&primes, 26), vec![2, 13]);
        assert_eq!(find_factors_sieve(&primes, 63), vec![3, 3, 7]);
        assert_eq!(
            find_factors_sieve(&primes,312680865509917 ),
            vec![7791799, 40129483]
        );
    }

    #[test]
    fn test_multiply_vector() {
        assert_eq!(multiply_vector(&[2, 3]), 6);
        assert_eq!(multiply_vector(&[2, 2, 3, 5]), 60);
    }
}
