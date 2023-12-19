use std::collections::HashSet;
use std::fmt::Debug;
use std::fs;
use std::str::FromStr;

use regex::{Captures, Regex};

fn main() {
    let input = fs::read_to_string("src/input").unwrap();
    let cards: Vec<Card> = input.lines().map(Card::parse_line).collect();
    part1(&cards);
    part2(&cards);
}

#[derive(Debug)]
struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    my_numbers: Vec<u32>,
}

impl Card {
    fn parse_numbers<T: FromStr>(text: &str) -> Vec<T>
        where <T as FromStr>::Err: Debug {
        let number_regex = Regex::new(r"\d+").unwrap();
        number_regex.find_iter(text)
            .map(|s| s.as_str().parse::<T>().unwrap())
            .collect()
    }

    fn parse_regex_match_numbers<T: FromStr>(captures: &Captures, name: &str) -> Vec<T>
        where <T as FromStr>::Err: Debug {
        let numbers = captures.name(name).unwrap().as_str();
        Self::parse_numbers(numbers)
    }

    fn parse_line(line: &str) -> Card {
        let card_regex = Regex::new(r"^Card\s+(?<id>\d+):\s+(?<winning_numbers>[^|]+)\s*\|\s*(?<my_numbers>.+)\s*$").unwrap();
        let card_match = card_regex.captures(line).expect("invalid line");
        let id = card_match.name("id").expect("invalid card id");
        let id = id.as_str().parse::<u32>().unwrap();

        let winning_numbers = Self::parse_regex_match_numbers(&card_match, "winning_numbers");
        let my_numbers = Self::parse_regex_match_numbers(&card_match, "my_numbers");

        Card { id, winning_numbers, my_numbers }
    }

    fn matched_numbers(&self) -> Vec<u32> {
        HashSet::<u32>::from_iter(self.winning_numbers.iter().copied())
            .intersection(&HashSet::from_iter(self.my_numbers.iter().copied()))
            .map(|&x| x)
            .collect()
    }

    fn points(&self) -> u32 {
        let num_matched_numbers = self.matched_numbers().len() as u32;
        if num_matched_numbers > 0 {
            2u32.pow(num_matched_numbers - 1)
        } else {
            0
        }
    }
}


fn part1(cards: &Vec<Card>) {
    let points: u32 = cards.iter().map(Card::points).sum();

    println!("Part 1: {points}");
}


fn part2(cards: &Vec<Card>) {
    let matched_numbers: Vec<u32> = cards.iter().map(|card| card.matched_numbers().len() as u32).collect();
    let mut counts: Vec<u32> = Vec::with_capacity(matched_numbers.len());
    for _ in 0..matched_numbers.len() {
        counts.push(1);
    }

    for i in 0..cards.len() {
        for j in 0..matched_numbers[i] as usize {
            counts[i + j + 1] += counts[i];
        }
    }

    let num_cards: u32 = counts.iter().sum();
    println!("Part 2: {num_cards}");
}
