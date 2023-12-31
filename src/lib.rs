use std::time::{SystemTime, UNIX_EPOCH};

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
    let base = num % modulus;

    (2..=pow).into_iter().fold(base, |acc, _| acc * base % modulus)
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

// milestone 5
// -----------

pub struct Prng {
    seed: u32,
}

impl Prng {
    pub fn new() -> Self {
        let mut prng = Self { seed: 0 };
        prng.randomize();

        prng
    }

    fn randomize(&mut self) {
        let millis = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();
        self.seed = millis as u32;
    }

    fn next_u32(&mut self) -> u32 {
        self.seed = self.seed.wrapping_mul(1_103_515_245).wrapping_add(12_345);
        self.seed %= 1 << 31;

        self.seed
    }

    fn next_f64(&mut self) -> f64 {
        let f = self.next_u32() as f64;
        f / (2147483647.0 + 1.0)
    }

    fn next_i64(&mut self, min: i64, max: i64) -> i64 {
        let range = (max - min) as f64;
        let result = min as f64 + range * self.next_f64();

        result as i64
    }
}

fn is_probably_prime(rng: &mut Prng, candidate: u32, num_tests: u32) -> bool {
    let pow = candidate as i64 - 1;

    for _ in 0..num_tests {
        let n = rng.next_i64(2, candidate as i64);

        if fast_exp_mod(n, pow, candidate as i64) != 1 {
            return false;
        }
    }

    true
}

pub fn find_prime(rng: &mut Prng, min: i64, max: i64, num_tests: u32) -> u32 {
    loop {
        let candidate = rng.next_i64(min, max) as u32;

        if is_probably_prime(rng, candidate, num_tests) {
            return candidate;
        }
    }
}

// milestone 6
// -----------

pub fn totient(p: i64, q: i64) -> i64 {
    lcm(p - 1, q - 1)
}

pub fn random_exponent(prng: &mut Prng, ln: i64) -> i64 {
    loop {
        let candidate = prng.next_i64(3, ln);

        if gcd(candidate, ln) == 1 {
            return candidate;
        }
    }
}

pub fn inverse_mod(a: i64, n: i64) -> i64 {
    let mut t = 0;
    let mut newt = 1;
    let mut r = n;
    let mut newr = a;

    while newr != 0 {
        let quotient = r / newr;
        (t, newt) = (newt, t - quotient * newt);
        (r, newr) = (newr, r - quotient * newr);
    }

    if r > 1 { return -1; }

    if t < 0 { t + n } else { t }
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
        assert_eq!(fast_exp_mod(8, 2, 10), 4);
        assert_eq!(fast_exp_mod(8, 3, 10), 2);
        assert_eq!(fast_exp_mod(8, 4, 10), 6);
        assert_eq!(fast_exp_mod(8, 5, 10), 8);
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

    #[test]
    fn test_is_probably_prime() {
        let mut rng = Prng::new();

        assert!(is_probably_prime(&mut rng, 5, 20));
        assert!(is_probably_prime(&mut rng, 19, 20));
        assert!(is_probably_prime(&mut rng, 7791799, 20));
    }

    #[test]
    fn test_totient() {
        assert_eq!(totient(3449, 5009), 2158448);
    }
}
