pub fn find_max_prime_factor(mut n: u128) -> u128 {
    // this is too difficult...
    if n <= 1 { return n; }

    let mut max_prime = 1;

    // Handle 2's factors
    while n % 2 == 0 {
        max_prime = 2;
        n /= 2;
    }

    // Handle 3's factors
    while n % 3 == 0 {
        max_prime = 3;
        n /= 3;
    }

    // Ultra-fast check for numbers up to sqrt using 6k+-1 optimization
    let mut i = 5;
    let mut skip = 2;

    // Only check up to a reasonable limit for trial division
    while i * i <= n && i <= 1_000_000 {
        while n % i == 0 {
            max_prime = i;
            n /= i;
        }
        i += skip;
        skip = 6 - skip;
    }

    // If remaining number is prime or couldn't be factored, it's the largest factor
    if n > 1 {
        max_prime = n;
    }

    max_prime
}