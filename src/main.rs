use num_bigint::{BigUint, RandBigInt, ToBigUint};
use num_integer::Integer;
use num_traits::{One, Zero, CheckedSub}; // Import CheckedSub
use rand::thread_rng;
use rand::Rng; // Import Rng
use std::io;
use std::time::Instant; // Import Instant

// Function to compute the greatest common divisor (GCD)
fn gcd(a: &BigUint, b: &BigUint) -> BigUint {
    a.gcd(b)
}

// Function for modular exponentiation (base^exp % modulus)
fn modpow(base: &BigUint, exponent: &BigUint, modulus: &BigUint) -> BigUint {
    base.modpow(exponent, modulus)
}

// Classical period finding function (find smallest r > 0 such that a^r % n == 1)
// This is the part that a quantum computer speeds up significantly.
fn find_period_classical(a: &BigUint, n: &BigUint) -> Option<BigUint> {
    if gcd(a, n) != BigUint::one() {
        // 'a' shares a factor with 'n', this case is handled before calling find_period
        return None;
    }

    let one = BigUint::one();
    let mut r = BigUint::one();
    let mut x = a % n;

    // Limit the search for practical reasons in this classical simulation
    let limit = n * n; // A reasonable upper bound heuristic, though not guaranteed

    while x != one {
        x = (&x * a) % n;
        r += &one;
        if r > limit {
            // Period finding took too long, might be very large or 'a' was unlucky
            println!("Period finding exceeded limit for a = {}", a);
            return None;
        }
    }
    Some(r)
}

// Shor's algorithm implementation (using classical period finding)
fn shors_algorithm(n: &BigUint) -> Option<(BigUint, BigUint)> {
    if n.is_even() {
        return Some((2u32.to_biguint().unwrap(), n / 2u32));
    }
    if n <= &BigUint::one() {
        println!("Number must be greater than 1.");
        return None;
    }
    // Check if n is prime (using a simple primality test for demonstration)
    // A more robust primality test (like Miller-Rabin) should be used for larger numbers.
    // This basic check is omitted for brevity, assuming n is composite.

    let one = BigUint::one();
    let two = 2u32.to_biguint().unwrap();
    let mut rng = thread_rng();

    loop {
        // 1. Pick a random number 'a' such that 1 < a < n
        let a = rng.gen_biguint_range(&two, n);
        println!("Trying a = {}", a);

        // 2. Compute gcd(a, n)
        let common_divisor = gcd(&a, n);
        if common_divisor > one {
            println!("Found factor (GCD): {}", common_divisor);
            return Some((common_divisor.clone(), n / common_divisor));
        }

        // 3. Find the period 'r' of a^x mod n
        // *** This is where the Quantum Fourier Transform would be used on a quantum computer ***
        println!("Finding period classically (this is the slow part)...");
        let r_opt = find_period_classical(&a, n);

        if r_opt.is_none() {
            println!("Could not find period classically for a = {}. Trying another 'a'.", a);
            continue; // Try a different 'a'
        }
        let r = r_opt.unwrap();
        println!("Found period r = {}", r);

        // 4. Check if 'r' is even
        if r.is_odd() {
            println!("Period 'r' is odd. Trying another 'a'.");
            continue;
        }

        // 5. Check if a^(r/2) % n == n - 1 (or a^(r/2) == -1 mod n)
        let r_half = &r / &two;
        let term = modpow(&a, &r_half, n);

        let n_minus_1 = n - &one;
        if term == n_minus_1 {
            println!("a^(r/2) % n == -1 (mod n). Trying another 'a'.");
            continue;
        }

        // 6. Compute factors
        let factor1 = gcd(&(term.clone() + &one), n);
        // Use checked_sub which is now in scope
        let factor2 = gcd(&(term.checked_sub(&one).unwrap_or_else(|| n.clone() + term.clone() - &one)), n); // Handles potential underflow if term is 0 or 1

        if factor1 != one && factor1 != *n {
             println!("Found factor (Shor's): {}", factor1);
        }
         if factor2 != one && factor2 != *n {
             println!("Found factor (Shor's): {}", factor2);
             return Some((factor2.clone(), n / factor2));
        }

        // If factors are trivial (1 or n), try another 'a'
        println!("Found trivial factors. Trying another 'a'.");
    }
}

fn main() {
    println!("Enter the number (N) to factor:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n_str = input.trim();

    match BigUint::parse_bytes(n_str.as_bytes(), 10) {
        Some(n) => {
            if n < 4u32.to_biguint().unwrap() {
                 println!("Please enter a composite number greater than 3.");
                 return;
            }
            println!("Attempting to factor N = {}", n);

            // Start timing
            let start_time = Instant::now();

            let result = shors_algorithm(&n);

            // Calculate duration
            let duration = start_time.elapsed();

            match result {
                Some((p, q)) => {
                    println!("\nFactors found: {} and {}", p, q);
                    // Use references for multiplication within println!
                    println!("Verification: {} * {} = {}", p, q, &p * &q);
                }
                None => {
                    println!("\nFailed to find factors. The number might be prime or the algorithm failed (e.g., period finding limit exceeded).");
                }
            }
            // Print the duration
            println!("Computation took: {:?}", duration);
        }
        None => {
            println!("Invalid number input.");
        }
    }
}