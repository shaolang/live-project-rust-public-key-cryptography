use std::io::{self, Write};
use rpkc;

fn main() {
    loop {
        let a = get_i64("Enter the first integer: ");
        let b = get_i64("Enter the first integer: ");
        let gcd = rpkc::gcd(a, b);
        let lcm = rpkc::lcm(a, b);

        println!("GCD({a}, {b}): {gcd}");
        println!("LCM({a}, {b}): {lcm}");
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

