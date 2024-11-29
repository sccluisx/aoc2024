use my_competitive_solutions::day01::process_input;

#[test]
fn test_day01_sum_numbers() {
    let input = "1\n2\n3\n4";
    assert_eq!(process_input(input), 10);
}

#[test]
fn test_day01_empty_input() {
    let input = "";
    assert_eq!(process_input(input), 0);
}