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
#[cfg(test)]
mod tests {
    use super::*;

    fn get_input(input: Vec<&str>) -> Vec<String> {
        input.into_iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn test_day01a() {
        let input = vec!("+1", "+1", "+1");
        assert_eq!(day01a(get_input(input)), 3);
    }

    #[test]
    fn test_day01b() {
        let input = vec!("+3", "+3", "+4", "-2", "-4");
        assert_eq!(day01b(get_input(input)), 10);
    }

}
