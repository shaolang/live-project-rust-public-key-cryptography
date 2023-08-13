use std::io::{self, Write};
use rpkc;

fn main() {
    loop {
        let num = get_i64("Enter num: ");
        let pow = get_i64("Enter pow: ");
        let modulus = get_i64("Enter modulus: ");
        let num_pow = rpkc::fast_exp(num, pow);
        let num_pow_mod = rpkc::fast_exp_mod(num, pow, modulus);

        println!("fast_exp({num}, {pow}): {num_pow}");
        println!("fast_exp_mod({num}, {pow}, {modulus}): {num_pow_mod}");
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

