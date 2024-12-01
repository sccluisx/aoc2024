use aoc2024::{read_file_line_by_line, read_line}; // Use `aoc2024` instead of `crate::lib`

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

    // println!("Day 01: Reading a specific line...");
    //match read_line("day01.txt", 2) {
    //    Ok(Some(line)) => println!("Line 2: {}", line),
    //    Ok(None) => println!("Line 2 does not exist."),
    //    Err(e) => eprintln!("Error reading file: {}", e),
    // }1
}
