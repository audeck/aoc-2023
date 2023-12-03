use input::Game;

use crate::input::{get_input, get_test_input, parse_input};

mod input;

impl Game {
    fn get_power(&self) -> u32 {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        for hand in self.hands.clone() {
            max_red = max_red.max(hand.red_count);
            max_green = max_green.max(hand.green_count);
            max_blue = max_blue.max(hand.blue_count);
        }

        return max_red * max_green * max_blue;
    }
}

fn solution(input: Vec<Game>) -> u32 {
    return input.into_iter().map(|game| game.get_power()).sum();
}

fn main() {
    let test_input = parse_input(get_test_input());
    let test_solution = solution(test_input);
    assert_eq!(test_solution, 2286);

    let input = parse_input(get_input());
    let solution = solution(input);
    println!("");
    println!("Solution: {solution}");
    println!("");
}
