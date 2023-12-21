use regex::Regex;
use std::fmt::Debug;
use std::fs;
use std::str::FromStr;

fn main() {
    let input = fs::read_to_string("src/input").unwrap();
    let race_data = parse_data(&input);
    part1(&race_data);
    part2(&race_data);
}

#[derive(Debug)]
struct RaceData {
    time: u64,
    record_distance: u64,
}

impl RaceData {
    fn ways_to_win(&self) -> u32 {
        // TODO: solve the quadratic equation
        let mut ways = 0;
        for start_speed in 1..self.time {
            if start_speed * (self.time - start_speed) > self.record_distance {
                ways += 1;
            }
        }
        ways
    }
}

fn parse_ints<T: FromStr>(text: &str) -> Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    text.split_whitespace()
        .map(|num_text| num_text.parse().unwrap())
        .collect()
}

fn parse_data(input: &str) -> Vec<RaceData> {
    let times_regex = Regex::new(r"Time:\s*(?<times>[\d\s]+)").unwrap();
    let distances_regex = Regex::new(r"Distance:\s*(?<distances>[\d\s]+)").unwrap();

    let lines: Vec<&str> = input.lines().collect();
    if let [times_line, distances_line] = lines[..] {
        let times = times_regex
            .captures(times_line)
            .unwrap()
            .name("times")
            .unwrap()
            .as_str();
        let times: Vec<u64> = parse_ints(times);

        let record_distances = distances_regex
            .captures(distances_line)
            .unwrap()
            .name("distances")
            .unwrap()
            .as_str();
        let record_distances: Vec<u64> = parse_ints(record_distances);

        times
            .iter()
            .zip(record_distances)
            .map(|(&time, record_distance)| RaceData {
                time,
                record_distance,
            })
            .collect()
    } else {
        panic!("unknown data format");
    }
}

fn part1(races: &Vec<RaceData>) {
    let solution: u32 = races.iter().map(RaceData::ways_to_win).product();
    println!("Part 1: {solution}");
}

fn concat_numbers(a: u64, mut b: u64) -> u64 {
    let mut b_digits: Vec<u64> = Vec::new();
    while b > 0 {
        b_digits.push(b % 10);
        b /= 10;
    }
    b_digits.iter().rev().fold(a, |acc, digit| acc * 10 + digit)
}

#[test]
fn test_concat_numbers() {
    assert_eq!(concat_numbers(10, 10), 1010);
    assert_eq!(concat_numbers(12, 3456), 123456);
}

fn part2(races: &Vec<RaceData>) {
    let race_data = races.iter().fold(
        RaceData {
            time: 0,
            record_distance: 0,
        },
        |mut acc, rd| {
            acc.time = concat_numbers(acc.time, rd.time);
            acc.record_distance = concat_numbers(acc.record_distance, rd.record_distance);
            acc
        },
    );
    println!("Part 2: {}", race_data.ways_to_win());
}
