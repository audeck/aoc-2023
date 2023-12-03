use input::Game;

use crate::input::{get_input, get_test_input, parse_input};

mod input;

impl Game {
    fn is_valid(&self, red_cubes: u32, green_cubes: u32, blue_cubes: u32) -> bool {
        for hand in self.hands.clone() {
            if red_cubes < hand.red_count
                || green_cubes < hand.green_count
                || blue_cubes < hand.blue_count
            {
                return false;
            }
        }

        return true;
    }
}

fn solution(input: Vec<Game>) -> u32 {
    let red_cubes = 12;
    let green_cubes = 13;
    let blue_cubes = 14;

    return input
        .into_iter()
        .filter(|game| game.is_valid(red_cubes, green_cubes, blue_cubes))
        .map(|game| game.number)
        .sum();
}

fn main() {
    let test_input = parse_input(get_test_input());
    let test_solution = solution(test_input);
    assert_eq!(test_solution, 8);

    let input = parse_input(get_input());
    let solution = solution(input);
    println!("");
    println!("Solution: {solution}");
    println!("");
}
