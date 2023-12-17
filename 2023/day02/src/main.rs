use std::collections::HashMap;
use std::fs;
use regex::Regex;

#[derive(PartialEq, Eq, Hash, Debug)]
enum Color {
    RED,
    GREEN,
    BLUE,
}

type Hand = HashMap<Color, u32>;

struct Game {
    hands: Vec<Hand>,
}

impl Game {
    fn parse(hands_text: &str) -> Self {
        let mut hands = Vec::new();
        let hand_regex = Regex::new(r"(?<number>(\d)+)\s+(?<color>red|green|blue)").unwrap();
        for hand_text in hands_text.split(';') {
            let mut balls = HashMap::new();
            for hand in hand_regex.captures_iter(hand_text) {
                let number_of_balls = &hand["number"].parse::<u32>().unwrap();
                let color = match &hand["color"] {
                    "red" => Color::RED,
                    "green" => Color::GREEN,
                    "blue" => Color::BLUE,
                    _ => panic!("unknown color")
                };
                balls.insert(color, *number_of_balls);
            }
            hands.push(balls);
        }
        Game { hands }
    }
}

fn main() {
    let input = fs::read_to_string("src/input").unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let balls = HashMap::from([
        (Color::RED, 12),
        (Color::GREEN, 13),
        (Color::BLUE, 14),
    ]);

    let mut valid_games = 0;
    for line in input.lines() {
        let line_parts: Vec<&str> = line.split(':').collect();
        let game_regex = Regex::new(r"Game\s+(?<game_number>(\d)+):").unwrap();
        let game_number = game_regex.captures(&line).unwrap();
        let game_number = &game_number["game_number"].parse::<u32>().unwrap();
        let game = Game::parse(&line_parts[1]);

        let mut valid = true;
        'hand_iter: for hand in &game.hands {
            for (color, number_of_balls) in hand.iter() {
                if number_of_balls > balls.get(color).unwrap() {
                    valid = false;
                    break 'hand_iter;
                }
            }
        }
        if valid {
            valid_games += game_number;
        }
    }

    println!("Part 1: {valid_games}");
}

fn part2(input: &str) {
    let mut sum = 0;
    for line in input.lines() {
        let line_parts: Vec<&str> = line.split(':').collect();
        let game = Game::parse(&line_parts[1]);


        let mut ball_count = HashMap::new();
        for hand in &game.hands {
            for (color, &number_of_balls) in hand.iter() {
                let count = ball_count.entry(color).or_insert(0);
                *count = number_of_balls.max(*count);
            }
        }

        sum += ball_count.values().product::<u32>();
    }

    println!("Part 2: {sum}");
}