pub fn day01a(input: Vec<String>) -> i32 {
    input.iter()
        .map(|s| s.parse::<i32>().unwrap())
        .fold(0, |acc, i| acc + i)
}
