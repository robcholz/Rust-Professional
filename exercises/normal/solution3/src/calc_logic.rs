pub fn new_birthday_probability(n: u32) -> f64 {
    if n > 365 {
        return 1.0;
    }

    let mut prob_unique = 1.0;
    for i in 0..n {
        prob_unique *= (365 - i) as f64 / 365.0;
    }

    1.0 - prob_unique
}
