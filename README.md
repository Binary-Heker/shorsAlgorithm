# Shor's Algorithm (Classical Simulation) in Rust

This repository contains a Rust implementation of the steps involved in Shor's algorithm for integer factorization.

**Important Note:** This implementation uses a *classical* method for the period-finding step. Therefore, it **does not** exhibit the exponential speedup provided by a true quantum computer. It serves as an educational tool to understand the classical components and the overall structure of Shor's algorithm.

## Why is Shor's Algorithm Important?

Shor's algorithm, developed by Peter Shor in 1994, is a **quantum algorithm** renowned for its ability to find the prime factors of a large integer exponentially faster than any known classical algorithm.

Its significance stems primarily from its implications for **cryptography**:

1.  **Breaking RSA:** Many widely used public-key cryptosystems, most notably RSA, rely on the practical difficulty of factoring large composite numbers using classical computers. The security of RSA keys depends on the assumption that factoring the product of two large primes is computationally infeasible within a reasonable timeframe.
2.  **Quantum Threat:** A sufficiently large and stable quantum computer running Shor's algorithm could theoretically break current RSA encryption standards relatively quickly, rendering sensitive data protected by RSA vulnerable.
3.  **Driving Post-Quantum Cryptography:** The existence of Shor's algorithm is a major driving force behind the research and development of **post-quantum cryptography (PQC)** – cryptographic algorithms designed to be secure against attacks from both classical and quantum computers.

While this code simulates the algorithm's steps classically, the period-finding function (`find_period_classical`) represents the computational bottleneck that a quantum computer overcomes using the Quantum Fourier Transform (QFT).

## Code Structure

*   `src/main.rs`: Contains the main Rust code, including:
    *   `gcd`: Computes the greatest common divisor.
    *   `modpow`: Computes modular exponentiation.
    *   `find_period_classical`: Classically finds the period `r` such that `a^r ≡ 1 (mod n)`. **This is the slow part.**
    *   `shors_algorithm`: Implements the main logic of Shor's algorithm, calling the helper functions.
    *   `main`: Handles user input, calls the algorithm, and times the execution.

## Dependencies

This project uses the following Rust crates:

*   `num-bigint`: For handling arbitrarily large integers.
*   `num-integer`: Provides integer traits like GCD.
*   `num-traits`: Provides numeric traits like `One`, `Zero`, `CheckedSub`.
*   `rand`: For random number generation.

These are listed in the `Cargo.toml` file.

## Building and Running

1.  **Install Rust:** If you haven't already, install Rust and Cargo: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
2.  **Clone the repository (if applicable):**
    ```bash
    git clone https://github.com/Binary-Heker/shorsAlgorithm
    cd shorsAlgorithm
    ```
3.  **Build:**
    ```bash
    cargo build
    ```
4.  **Run:**
    ```bash
    cargo run
    ```
    The program will prompt you to enter a number (N) to factor.


**Example:**

```
Enter the number (N) to factor:
4819
Attempting to factor N = 4819
Trying a = 656
Finding period classically (this is the slow part)...
Found period r = 30
a^(r/2) % n == -1 (mod n). Trying another 'a'.
Trying a = 3325
Finding period classically (this is the slow part)...
Found period r = 780
Found factor (Shor's): 61
Found factor (Shor's): 79

Factors found: 79 and 61
Verification: 79 * 61 = 4819
Computation took: 2.392041ms
```
*(Note: The specific 'a' value chosen and the computation time will vary)*.