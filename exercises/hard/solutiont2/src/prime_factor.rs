use std::ops::Range;
use std::thread;

pub fn find_max_prime_factor(number: u128) -> u128 {
    let middle_point = (number as f64).sqrt().floor() as u128;

    fn is_prime(n: u128) -> bool {
        if n <= 1 {
            return false;
        }
        if n <= 3 {
            return true;
        }
        if n % 2 == 0 || n % 3 == 0 {
            return false;
        }

        let mut i = 5;
        while i * i <= n {
            if n % i == 0 || n % (i + 2) == 0 {
                return false;
            }
            i += 6;
        }
        true
    }

    fn parallel_computation(num: u128, range: Range<u128>) -> u128 {
        let max_number = 0u128;

        // Iterate in reverse order to find the largest prime factor sooner
        for k in range.rev().step_by(2) {  // Skip even numbers
            if num % k == 0 && is_prime(k) {
                return k;  // Largest prime found, return immediately
            }
        }

        max_number
    }

    let concurrency = 8;

    let mut from = middle_point;
    let mut to = number;
    let mut threads = vec![];
    let partition = (to - from) / concurrency;

    for t in 0..concurrency {
        to = from + partition * (t + 1);
        let handle = thread::spawn(move || {
            println!("Start thread {}", t + 1);
            let r = parallel_computation(number, from..to);
            println!("Finished thread {}", t + 1);
            r
        });
        threads.push(handle);
        from = to;
    }

    threads
        .into_iter()
        .map(|t| t.join().unwrap())
        .max()
        .unwrap()
}
