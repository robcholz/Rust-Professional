pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
    fn fib(n: u32) -> u32 {
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return 1;
        }
        fib(n - 1) + fib(n - 2)
    }
    let mut sum = 0u32;
    let mut result = 0u32;
    let mut counter = 0u32;
    while result < threshold {
        result = fib(counter);
        if result >= threshold {
            return sum;
        }
        counter += 1;

        if result % 2 != 0 {
            sum += result;
        }
    }
    sum
}
