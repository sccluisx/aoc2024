use aoc2024::{read_file_line_by_line, read_line}; // Use `aoc2024` instead of `crate::lib`
use std::collections::HashMap;

fn is_valid_sequence(levels: &[i32], inc: i32) -> bool {
    for i in 0..levels.len() - 1 {
        let cur = levels[i];
        let next = levels[i + 1];
        let diff = next - cur;

        if diff.abs() > 3 || diff == 0 {
            return false;
        }

        if inc == 1 && diff < 0 {
            return false;
        } else if inc == 0 && diff > 0 {
            return false;
        }
    }

    true
}


pub fn solve() {
    println!("Day 02: Processing input line by line...");
    let mut solution = 0;

    match read_file_line_by_line("day02_full.txt") {
        Ok(lines) => {
            let mut total = 0;
            'reports: for (i, line) in lines.enumerate() {
                println!("Line {}: {}", i + 1, line);
                let parts: Vec<&str> = line.split_whitespace().collect();
                let levels: Vec<i32> = parts
                    .iter() // Iterate over the parts
                    .filter_map(|&part| part.parse::<i32>().ok()) // Parse each part into an i32
                    .collect();
                let mut inc=0;
                match levels[1].cmp(&levels[0]) {
                    std::cmp::Ordering::Greater => inc = 1,
                    std::cmp::Ordering::Less => inc = 0,
                    std::cmp::Ordering::Equal => continue 'reports,
                }
                if inc == 0 {
                    println!("decreassing");
                } else {
                println!("increassing"); }

                for i in 0..levels.len() -1 {
                    let cur = levels[i];
                    let next = levels[i+1];
                    let diff = next - cur;
                    if diff.abs() > 3 || diff.abs() == 0 {
                        continue 'reports;
                    }
                    if inc == 1 && diff < 0 {
                        continue 'reports; // Sequence is not strictly increasing
                    } else if inc == 0 && diff > 0 {
                        continue 'reports; // Sequence is not strictly decreasing
                    }
                }
                solution += 1;
            }
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }
    println!("Total sum of numbers: {}", solution);

}
