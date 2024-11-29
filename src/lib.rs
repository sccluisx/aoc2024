use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// Reads the entire file into a single string.
pub fn read_file_to_string(filename: &str) -> io::Result<String> {
    let path = format!("input/{}", filename);
    std::fs::read_to_string(&path)
}

/// Returns an iterator to read the file line by line.
pub fn read_file_line_by_line(filename: &str) -> io::Result<impl Iterator<Item = String>> {
    let path = format!("input/{}", filename);
    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    Ok(reader.lines().filter_map(|line| line.ok()))
}

/// Reads a specific line from the file by its number (1-indexed).
pub fn read_line(filename: &str, line_number: usize) -> io::Result<Option<String>> {
    let path = format!("input/{}", filename);
    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    Ok(reader
        .lines()
        .enumerate()
        .find_map(|(idx, line)| if idx + 1 == line_number { line.ok() } else { None }))
}

/// Reads a range of lines from the file (1-indexed, inclusive).
pub fn read_lines_range(filename: &str, start: usize, end: usize) -> io::Result<Vec<String>> {
    let path = format!("input/{}", filename);
    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    Ok(reader
        .lines()
        .enumerate()
        .filter_map(|(idx, line)| {
            if idx + 1 >= start && idx + 1 <= end {
                line.ok()
            } else {
                None
            }
        })
        .collect())
}