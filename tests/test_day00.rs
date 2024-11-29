use std::fs;

#[test]
fn test_day00_read_file() {
    // Use the correct file path
    let test_file = format!("{}/input/day00.txt", env!("CARGO_MANIFEST_DIR"));
    println!("Resolved file path: {}", test_file);

    // Read the file content
    let content = fs::read_to_string(&test_file)
        .expect("Failed to read the input file");

    // Update the expected content to match the file content
    assert_eq!(
        content,
        "Test content for Day 00.",
        "File content does not match expected content"
    );
}

#[test]
fn test_day00_temp_file() {
    // Create a temporary file path
    let temp_file = format!("{}/test_day00_temp.txt", env!("CARGO_MANIFEST_DIR"));
    let test_content = "Temporary test content for Day 00.";

    // Write the test content to the temporary file
    fs::write(&temp_file, test_content).expect("Failed to create temporary test file");
    println!("Temporary file created at: {}", temp_file);

    // Verify that the temporary file exists
    assert!(
        std::path::Path::new(&temp_file).exists(),
        "Temporary file was not created"
    );

    // Read the content of the temporary file
    let content = fs::read_to_string(&temp_file)
        .expect("Failed to read the temporary test file");

    // Assert that the content matches
    assert_eq!(
        content, test_content,
        "Temporary file content does not match expected content"
    );

    // Clean up the temporary file
    fs::remove_file(&temp_file).expect("Failed to delete temporary test file");
}