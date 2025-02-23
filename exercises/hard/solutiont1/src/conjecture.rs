pub fn goldbach_conjecture() -> String {
    fn is_prime(n: u64) -> bool {
        if n <= 1 { return false; }
        if n <= 3 { return true; }
        if n % 2 == 0 || n % 3 == 0 { return false; }

        let mut i = 5;
        while i * i <= n {
            if n % i == 0 || n % (i + 2) == 0 {
                return false;
            }
            i += 6;
        }
        true
    }

    fn can_be_goldbach(n: u64) -> bool {
        // Check all possible squares
        let max_k = ((n as f64) / 2.0).sqrt().ceil() as u64;

        for k in 0..=max_k {
            let twice_square = 2 * k * k;
            if twice_square >= n { break; }

            // Check if n - 2k² is prime
            let remaining = n - twice_square;
            if is_prime(remaining) {
                return true;
            }
        }
        false
    }

    let mut found = Vec::new();
    let mut n = 3;

    while found.len() < 2 {
        if n % 2 == 1 && !is_prime(n) && !can_be_goldbach(n) {
            found.push(n);
        }
        n += 2;
    }

    format!("{},{}", found[0], found[1])
}