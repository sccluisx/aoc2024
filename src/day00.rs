use aoc2024::read_file_to_string;

pub fn solve() {
    println!("Day 00: Reading input file...");

    match read_file_to_string("day00.txt") {
        Ok(content) => println!("File Content:\n{}", content),
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}