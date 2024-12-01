use aoc2024::{read_file_to_string, read_lines_range};

pub fn solve() {
    println!("Day 02: Reading the entire file...");

    match read_file_to_string("day02.txt") {
        Ok(content) => {
            let word_count = count_words(&content);
            println!("Total word count: {}", word_count);
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }

    println!("Day 02: Reading a range of lines...");
    match read_lines_range("day02.txt", 2, 4) {
        Ok(lines) => {
            for (i, line) in lines.iter().enumerate() {
                println!("Line {}: {}", i + 2, line);
            }
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}

fn count_words(content: &str) -> usize {
    content.split_whitespace().count()
}
