use regex::Regex;
use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("src/input").unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let first_digit_regex = Regex::new(r#"^\D*(\d)"#).unwrap();
    let last_digit_regex = Regex::new(r#"(\d)\D*$"#).unwrap();

    let part1 = input.lines().fold(0, |acc, line| {
        let first_digit = first_digit_regex.captures(line).unwrap();
        let last_digit = last_digit_regex.captures(line).unwrap();
        let number = format!("{}{}", &first_digit[1], &last_digit[1]);
        let number: u32 = number.parse().unwrap();
        acc + number
    });
    println!("Part 1: {part1}");
}

fn part2(input: &str) {
    let digit_strings = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    let mut digit_names: Vec<&str> = digit_strings.keys().map(|x| *x).collect();
    digit_names.push(r"\d");
    let digit_pattern = digit_names.join("|");
    let first_digit_regex = Regex::new(&format!("({digit_pattern})(.)*$")).unwrap();
    let last_digit_regex = Regex::new(&format!("^(.)*({digit_pattern})")).unwrap();

    let part2 = input.lines().fold(0, |acc, line| {
        let first_digit = first_digit_regex.captures(line).unwrap();
        let first_digit = &first_digit[1];
        let last_digit = last_digit_regex.captures(line).unwrap();
        let last_digit = &last_digit[2];

        let get_digit = |digit_string: &str| {
            if let Some(digit) = digit_strings.get(digit_string) {
                *digit
            } else {
                digit_string.parse::<i32>().unwrap()
            }
        };

        let first_digit = get_digit(first_digit);
        let last_digit = get_digit(last_digit);
        let number = format!("{first_digit}{last_digit}");
        let number: u32 = number.parse().unwrap();
        acc + number
    });

    println!("Part 2: {part2}");
}
