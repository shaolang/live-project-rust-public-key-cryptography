use std::io::{self, Write};
use std::time::Instant;
use rpkc;

fn main() {
    let primes = rpkc::sieve_to_primes(rpkc::sieve_of_eratosthenes(50_000_000));

    simple_benchmark(&primes);

    loop {
        let n = get_i64("Number: ");
        let mut factors = rpkc::find_factors_sieve(&primes, n);
        print!("Factors: ");
        rpkc::print_numbers(&mut factors);
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
