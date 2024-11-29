use aoc2024::{read_file_line_by_line, read_line}; // Use `aoc2024` instead of `crate::lib`

pub fn solve() {
    println!("Day 01: Processing input line by line...");

    match read_file_line_by_line("day01.txt") {
        Ok(lines) => {
            let mut total = 0;
            for (i, line) in lines.enumerate() {
                println!("Line {}: {}", i + 1, line);
                total += line.parse::<i32>().unwrap_or(0); // Assume numbers for demonstration
            }
            println!("Total sum of numbers: {}", total);
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }

    println!("Day 01: Reading a specific line...");
    match read_line("day01.txt", 2) {
        Ok(Some(line)) => println!("Line 2: {}", line),
        Ok(None) => println!("Line 2 does not exist."),
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}