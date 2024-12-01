use aoc2024::{read_file_line_by_line, read_line}; // Use `aoc2024` instead of `crate::lib`
use std::collections::HashMap;

pub fn solve() {
    println!("Day 01: Processing input line by line...");
    let mut vec_a = Vec::new();
    let mut vec_b = Vec::new();
    let mut solution = 0;
    match read_file_line_by_line("day01.txt") {
        Ok(lines) => {
            let mut total = 0;
            for (i, line) in lines.enumerate() {
                println!("Line {}: {}", i + 1, line);
                let parts: Vec<&str> = line.split_whitespace().collect();

                if let (Some(a_str), Some(b_str)) = (parts.get(0), parts.get(1)) {
                    if let (Ok(a), Ok(b)) = (a_str.parse::<i32>(), b_str.parse::<i32>()) {
                        vec_a.push(a);
                        vec_b.push(b);
                    } else {
                        eprintln!("Could not parse numbers");
                    }
                } else {
                    eprintln!("Invalid format");
                }
                // total += line.parse::<i32>().unwrap_or(0); // Assume numbers for demonstration
            }
            //println!("Total sum of numbers: {}", total);
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }
    vec_a.sort();
    vec_b.sort();
    for i in 0..vec_a.len() {
        solution += (vec_a[i] - vec_b[i]).abs();
    }

    println!("solution: {:?}", solution);
    let mut map: HashMap<i32, i32> = vec_a.iter().map(|&key| (key, 0)).collect();
    for value in vec_b {
        if let Some(count) = map.get_mut(&value) {
            // If the number exists as a key, increment its value by 1
            *count += 1;
        }
    }

    // Print the updated map to see the result
    for (key, value) in &map {
        println!("Key: {}, Value: {}", key, value);
    }
    println!("........................");
    let mut simindex = 0;
    for ele in vec_a.iter() {
        if let Some(count) = map.get_mut(ele){
        println!("ele: {}, Value: {}", ele, *count);

        simindex += ele*(*count);
        }
    }
    println!("simindex : {:?} ", simindex);
}
