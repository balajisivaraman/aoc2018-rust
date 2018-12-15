use std::collections::HashSet;

pub fn day01a(input: Vec<String>) -> i32 {
    input.iter()
        .map(|s| s.parse::<i32>().unwrap())
        .sum()
}

pub fn day01b(input: Vec<String>) -> i32 {
    let mut seen_numbers = HashSet::new();
    let mut acc = 0;
    for num in input.into_iter().map(|s| s.parse::<i32>().unwrap()).cycle() {
        acc += num;
        if seen_numbers.contains(&acc) {
            return acc;
        } else {
            seen_numbers.insert(acc);
        }
    }
    0
}
