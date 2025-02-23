use std::ops::Range;
use std::sync::Arc;
use std::thread;
use std::time::Instant;

pub fn find_max_prime_factor(mut number: u128) -> u128 {
    fn sub_problem(start: u128, mut end: u128) -> u128 {
        let mut max_prime = 0;

        // 处理所有2的因子
        while end % 2 == 0 {
            max_prime = 2;
            end /= 2;
        }

        // 处理奇数因子，从3开始，每次跳过偶数
        let mut factor = start;
        if factor % 2 == 0 {
            factor -= 1;
        }

        // overflow
        if factor as f64 > (end as f64).sqrt() {
            return max_prime;
        }

        while factor * factor <= end {
            while end % factor == 0 {
                max_prime = factor;
                end /= factor;
            }
            factor += 2; // 只检查奇数
        }

        // 如果剩余的 number 本身是一个大于1的素数
        if end > 1 {
            max_prime = end;
        }

        max_prime
    }

    fn partition(start: u128, end: u128, piece: u8) -> Vec<Range<u128>> {
        // Calculate the total size of the range
        let total_size = end - start;

        // Calculate the number of partitions
        let num_partitions = piece as usize;

        // Calculate the partition size
        let partition_size = total_size / num_partitions as u128;

        let mut partitions = Vec::new();
        let mut current_start = start;

        for _ in 0..num_partitions {
            // Determine the end of this partition
            let partition_end = std::cmp::min(current_start + partition_size, end);

            // Push the partition range into the vector
            partitions.push(current_start..partition_end);

            // Move the current start for the next partition
            current_start = partition_end;

            // If the current start reaches or exceeds the end, break
            if current_start >= end {
                break;
            }
        }

        // If there is remaining space, add the final partition
        if current_start < end {
            partitions.push(current_start..end);
        }

        partitions
    }

    let mut thread_factor =
        ((number as f64).sqrt() / (97993999919999958437f64).sqrt()).ceil() as u128;
    if thread_factor > 1 {
        thread_factor *= 3;
    }
    let threads = 1;
    println!("{}", threads);
    let start = 3;
    let end = number;
    let partitions = Arc::new(partition(start, end, threads));
    (0..threads)
        .map(|thread_num| {
            let partitions = partitions.clone();
            let handle = thread::spawn(move || {
                let t = Instant::now();
                let r = partitions.get(thread_num as usize).unwrap();
                let rs = sub_problem(r.start, r.end);
                let duration = t.elapsed();
                println!(
                    "Thread {}, range [{},{}), time elapsed: {}ms",
                    thread_num,
                    r.start,
                    r.end,
                    duration.as_millis()
                );
                rs
            });
            handle
        })
        .into_iter()
        .map(|t| {
            let r=t.join().unwrap();
            println!("{}",r);
            r
        })
        .max()
        .unwrap()
}
