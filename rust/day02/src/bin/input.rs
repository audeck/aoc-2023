use std::fs;

#[derive(Debug, Clone, Copy)]
enum Color {
    Red,
    Green,
    Blue,
}

impl std::str::FromStr for Color {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "blue" => Ok(Color::Blue),
            _ => Err("Invalid color"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Hand {
    pub red_count: u32,
    pub green_count: u32,
    pub blue_count: u32,
}

impl Hand {
    fn new() -> Self {
        return Self {
            red_count: 0,
            green_count: 0,
            blue_count: 0,
        };
    }

    fn add(&mut self, color: Color, count: u32) {
        match color {
            Color::Red => self.red_count = count,
            Color::Green => self.green_count = count,
            Color::Blue => self.blue_count = count,
        }
    }
}

#[derive(Debug)]
pub struct Game {
    pub number: u32,
    pub hands: Vec<Hand>,
}

impl Game {
    pub fn new(number: u32) -> Self {
        return Self {
            number,
            hands: Vec::new(),
        };
    }

    pub fn push(&mut self, hand: Hand) {
        self.hands.push(hand);
    }
}

fn parse_line(line: &str) -> Game {
    let first_space = line.find(' ').unwrap();
    let first_colon = line.find(':').unwrap();

    let game_num: u32 = line[first_space + 1..first_colon].parse().unwrap();
    let mut game_data: Game = Game::new(game_num);
    let game_data_str = line[first_colon + 2..].to_owned();

    for hand in game_data_str.split(';') {
        let mut hand_data = Hand::new();

        for color_data in hand.split(',') {
            let mut color_iter = color_data.trim().split_whitespace();

            let count: u32 = color_iter.next().unwrap().parse().unwrap();
            let color: Color = color_iter.next().unwrap().parse().ok().unwrap();

            hand_data.add(color, count);
        }

        game_data.push(hand_data);
    }

    return game_data;
}

pub fn parse_input(input: String) -> Vec<Game> {
    let mut output = Vec::new();
    input.lines().for_each(|line| output.push(parse_line(line)));
    return output;
}

pub fn get_test_input() -> String {
    return "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
        .to_owned();
}

pub fn get_input() -> String {
    return fs::read_to_string("src/bin/input.txt").unwrap();
}

#[allow(dead_code)]
fn main() {}
