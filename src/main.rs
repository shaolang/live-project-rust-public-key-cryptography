use std::io::{self, Write};
use rpkc;

const NUM_TESTS: i64 = 20;

fn main() {
    let mut rng = rpkc::Prng::new();
    let p = rpkc::find_prime(&mut rng, 1_000, 10_000, NUM_TESTS as u32) as i64;
    let q = rpkc::find_prime(&mut rng, 1_000, 10_000, NUM_TESTS as u32) as i64;
    let n = p * q;
    let ln = rpkc::totient(p, q);
    let e = rpkc::random_exponent(&mut rng, ln);
    let d = rpkc::inverse_mod(e, ln);

    println!("*** Public ***");
    println!("   Public key modulus: {n}");
    println!("Public key exponent e: {e}");
    println!("\n*** Private ***");
    println!("Primes: {p}, {q}");
    println!("  λ(n): {ln}");
    println!("     d: {d}");

    loop {
        let message = get_i64("\n   Message: ");

        if message < 2 || message > n - 1 { break; }

        let encrypted = rpkc::fast_exp_mod(message, e, n);
        let decrypted = rpkc::fast_exp_mod(encrypted, d, n);

        println!("Ciphertext: {encrypted}");
        println!(" Plaintext: {decrypted}");
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
