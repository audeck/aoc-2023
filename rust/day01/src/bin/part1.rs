use std::fs;

fn extract_number(input_line: &str) -> u32 {
    let first_digit = input_line.chars().find(|c| c.is_digit(10)).unwrap();
    let last_digit = input_line.chars().rev().find(|c| c.is_digit(10)).unwrap();

    let number = format!("{first_digit}{last_digit}");
    return match number.parse() {
        Ok(num) => num,
        Err(_) => panic!("Failed to parse extracted number"),
    };
}

fn solution(input: String) -> u32 {
    return input.lines().map(|line| extract_number(line)).sum();
}

fn get_test_input() -> String {
    return "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
        .to_owned();
}

fn get_input() -> String {
    return match fs::read_to_string("src/bin/part1.txt") {
        Ok(input) => input,
        Err(_) => panic!("Failed to read part 1 input"),
    };
}

fn main() {
    let test_input = get_test_input();
    assert_eq!(solution(test_input), 142);

    let input = get_input();
    println!("");
    println!("Solution: {}", solution(input));
    println!("");
}
