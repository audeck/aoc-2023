use std::collections::HashMap;
use std::fs;

use lazy_static::lazy_static;

lazy_static! {
    static ref DIGIT_MAP: HashMap<&'static str, char> = {
        return HashMap::from([
            ("one", '1'),
            ("two", '2'),
            ("three", '3'),
            ("four", '4'),
            ("five", '5'),
            ("six", '6'),
            ("seven", '7'),
            ("eight", '8'),
            ("nine", '9'),
        ]);
    };
    static ref DIGIT_MAP_KEYS: Vec<String> = {
        return DIGIT_MAP
            .keys()
            .cloned()
            .map(|key| key.to_string())
            .collect();
    };
}

fn extract_number(input_line: String) -> u32 {
    let first_number = get_first_digit(input_line.clone(), false);
    let last_number = get_first_digit(input_line.chars().rev().collect(), true);

    let number = format!("{first_number}{last_number}");
    return match number.parse() {
        Ok(num) => num,
        Err(_) => panic!("Failed to parse extracted number"),
    };
}

fn get_first_digit(input_line: String, rev: bool) -> char {
    if input_line.is_empty() {
        panic!("Got an empty `input_line`");
    }

    // Check for text digits
    for digit in DIGIT_MAP_KEYS.iter() {
        // If `rev`, `search_digit` is `digit` backwards; otherwise just `digit`
        let search_digit = match rev {
            true => digit.chars().rev().collect::<String>(),
            false => digit.clone(),
        };

        if input_line.starts_with(search_digit.as_str()) {
            return *DIGIT_MAP.get(digit.as_str()).unwrap();
        }
    }

    // Check for numerical digits
    let first_char = input_line.chars().nth(0).unwrap();
    if first_char.is_ascii_digit() {
        return first_char;
    }

    return get_first_digit(input_line.chars().skip(1).collect(), rev);
}

fn solution(input: String) -> u32 {
    return input
        .lines()
        .map(|line| extract_number(line.to_string()))
        .sum();
}

fn get_test_input() -> String {
    return "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
        .to_owned();
}

fn get_input() -> String {
    return match fs::read_to_string("src/bin/part2.txt") {
        Ok(input) => input,
        Err(_) => panic!("Failed to read part 2 input"),
    };
}

fn main() {
    let test_input = get_test_input();
    assert_eq!(solution(test_input), 281);

    let input = get_input();
    println!("");
    println!("Solution: {}", solution(input));
    println!("");
}
