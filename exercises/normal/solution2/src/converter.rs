pub fn convert_base(num_str: &str, to_base: u32) -> String {
    let base = &num_str[num_str.find('(').unwrap() + 1..num_str.find(')').unwrap()]
        .parse::<u32>()
        .unwrap();
    let number = &num_str[0..num_str.find('(').unwrap()];
    let dec = i64::from_str_radix(number, *base).unwrap() as u64;

    let mut num = dec;
    let base = to_base;
    let mut result = String::new();
    let digits = "0123456789ABCDEF".chars().collect::<Vec<_>>();

    if num == 0 {
        return "0".to_string();
    }

    while num > 0 {
        result.push(digits[(num % base as u64) as usize].to_ascii_lowercase());
        num /= base as u64;
    }

    result.chars().rev().collect()
}
