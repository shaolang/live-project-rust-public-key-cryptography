use std::io::{self, Write};
use rpkc;

fn main() {
    let max = get_i64("Max: ");
    let mut sieve = rpkc::sieve_of_eratosthenes(max as usize);

    if max < 1000 {
        rpkc::print_sieve(&mut sieve);
    }

    let mut primes = rpkc::sieve_to_primes(sieve);
    if max < 1000 {
        rpkc::print_numbers(&mut primes);
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

