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

// pub fn fast_exp_mod(mut num: i64, mut pow: i64, modulus: i64) -> i64 {
// }

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
}
