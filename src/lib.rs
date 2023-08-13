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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gcd_on_48_and_18() {
        assert_eq!(gcd(48, 18), 6);
    }

    #[test]
    fn gcd_on_18_and_48_returns_same_answer_as_48_and_18() {
        assert_eq!(gcd(18, 48), 6);
    }

    #[test]
    fn gcd_on_two_prime_numbers_returns_1() {
        assert_eq!(gcd(7, 11), 1);
    }

    #[test]
    fn gcd_on_same_number_returns_the_number_itself() {
        assert_eq!(gcd(100, 100), 100);
    }

    #[test]
    fn gcd_absolutes_number_before_calculating() {
        assert_eq!(gcd(-48, 18), 6);
        assert_eq!(gcd(48, -18), 6);
    }

    #[test]
    fn lcm_on_18_and_12() {
        assert_eq!(lcm(12, 18), 36);
    }

    #[test]
    fn fast_exp_on_3_exp_6() {
        assert_eq!(fast_exp(3, 6), 729);
    }

    #[test]
    fn fast_exp_on_8_exp_9() {
        assert_eq!(fast_exp(8, 9), 134_217_728);
    }

    #[test]
    fn fast_exp_mod_test_from_live_project() {
        assert_eq!(fast_exp_mod(8, 6, 10), 4);
    }

    #[test]
    fn sieve_of_eratosthenes_returns_false_for_0_and_1() {
        assert_eq!(sieve_of_eratosthenes(1), vec![false, false]);
    }

    #[test]
    fn sieve_of_eratosthenes_returns_false_for_2_and_3() {
        assert_eq!(sieve_of_eratosthenes(3), vec![false, false, true, true]);
    }

    #[test]
    fn sieve_of_eratosthenes_for_numbers_up_to_25() {
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
}
