use std::io::{self, Write};
use std::time::Instant;
use rpkc;

const NUM_TESTS: i64 = 20;

fn main() {
    let mut rng = rpkc::Prng::new();

    let probability = 100.0 - 100.0 / rpkc::fast_exp(2, NUM_TESTS) as f64;
    println!("Probability: {}%\n", probability);

    loop {
        let num_digits = get_i64("# Digits (max 9): ");
        if num_digits < 1 { break; }

        let mut min = 10i64.pow((num_digits - 1) as u32);
        let max = 10 * min;

        if min == 1 { min = 2; }

        println!("Prime: {}", rpkc::find_prime(&mut rng, min, max, NUM_TESTS as u32));
    }
}


fn get_i64(prompt: &str) -> i64 {
    print!("{prompt}");
    io::stdout().flush().unwrap();

    let mut str_value = String::new();
    io::stdin()
        .read_line(&mut str_value)
        .expect("Error reading input.");

    let trimmed = str_value.trim();

    trimmed.parse::<i64>().expect("Error parsing integer")
}


fn simple_benchmark(primes: &[i64]) {
    // find the factors the slow way
    let start1 = Instant::now();
    let num = 44711100255155897;
    let mut factors1 = rpkc::find_factors(num);
    let duration1 = start1.elapsed();

    println!("find_factors: {duration1:?} seconds");
    rpkc::print_numbers(&mut factors1);
    println!("Product: {}", rpkc::multiply_vector(&factors1));

    // use Euler's sieve to find the factors
    let start2 = Instant::now();
    let num = 312680865509917;
    let mut factors2 = rpkc::find_factors_sieve(&primes, num);
    let duration2 = start2.elapsed();

    println!("find_factors: {duration2:?} seconds");
    rpkc::print_numbers(&mut factors2);
    println!("Product: {}", rpkc::multiply_vector(&factors2));
}
