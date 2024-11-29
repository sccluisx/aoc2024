use crate::lib::{read_input, parse_numbers};

pub fn solve() {
    let input = read_input("day01.txt");
    let numbers = parse_numbers(&input);
    let result = sum_numbers(&numbers);
    println!("Day 1 Solution: {}", result);
}

fn sum_numbers(numbers: &[i32]) -> i32 {
    numbers.iter().sum()
}

// Optionally make the core logic public for testing
pub fn process_input(input: &str) -> i32 {
    let numbers = parse_numbers(input);
    sum_numbers(&numbers)
}