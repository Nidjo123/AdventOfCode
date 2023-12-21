use regex::Regex;
use std::fmt::Debug;
use std::fs;
use std::str::FromStr;

fn main() {
    let input = fs::read_to_string("src/input").unwrap();
    let race_data = parse_data(&input);
    part1(&race_data);
}

#[derive(Debug)]
struct RaceData {
    time: u32,
    record_distance: u32,
}

impl RaceData {
    fn ways_to_win(&self) -> u32 {
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
        let times: Vec<u32> = parse_ints(times);

        let record_distances = distances_regex
            .captures(distances_line)
            .unwrap()
            .name("distances")
            .unwrap()
            .as_str();
        let record_distances: Vec<u32> = parse_ints(record_distances);

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
