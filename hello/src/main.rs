use std::env;
use std::str::FromStr;

fn gcd(mut n: u64, mut m: u64) -> u64 {
    // Assert that both arguments are greater than zero.
    assert!(n > 0);
    assert!(m > 0);

    // Euclid's algorithm.
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    return n;
}

fn main() {
    let mut numbers = Vec::new();
    for arg in env::args().skip(1) {
        numbers.push(
            u64::from_str(&arg).expect("Unable to convert argument to unsigned 64 bit number"),
        );
    }

    if numbers.len() != 2 {
        eprintln!("Usage: hello X Y, where X and Y are u64 numbers");
        std::process::exit(1);
    }

    println!("Namaste, world!");
    println!(
        "The GCD of {} and {} is {}",
        numbers[0],
        numbers[1],
        gcd(numbers[0], numbers[1])
    );
}
