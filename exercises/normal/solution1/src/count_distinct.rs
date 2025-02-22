use std::collections::HashSet;

pub fn new_count_distinct(input_str: &str) -> usize {
    let tokens: Vec<&str> = input_str.split(',').collect();
    let mut map: HashSet<&str> = HashSet::with_capacity(tokens.len());
    for token in tokens {
        map.insert(token);
    }
    map.len()
}
