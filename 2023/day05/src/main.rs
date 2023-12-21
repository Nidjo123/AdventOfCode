use std::collections::HashMap;
use std::fmt::Debug;
use std::fs;
use std::ops::Range;
use std::str::FromStr;

use regex::Regex;

fn main() {
    let input = fs::read_to_string("src/input").unwrap();
    let data = parse_data(&input);
    part1(&data);
    part2(&data);
}

#[derive(Debug)]
struct RangeMap {
    source_range: Range<u64>,
    destination: u64,
}

impl RangeMap {
    fn new(source: u64, destination: u64, length: u64) -> Self {
        Self {
            source_range: source..source + length,
            destination,
        }
    }

    fn map(&self, number: u64) -> Option<u64> {
        if self.source_range.contains(&number) {
            Some(self.destination + number - self.source_range.start)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Map {
    source_name: String,
    destination_name: String,
    ranges: Vec<RangeMap>,
}

impl Map {
    fn new() -> Self {
        Self {
            source_name: String::new(),
            destination_name: String::new(),
            ranges: Vec::new(),
        }
    }

    fn map(&self, number: u64) -> u64 {
        for range in &self.ranges {
            if let Some(new_number) = range.map(number) {
                return new_number;
            }
        }
        number
    }
}

#[derive(Debug)]
struct Data {
    seeds: Vec<u64>,
    maps: HashMap<String, Map>,
}

impl Data {
    fn new() -> Self {
        Self {
            seeds: Vec::new(),
            maps: HashMap::new(),
        }
    }

    fn map(&self, src_category: &str, number: u64) -> Option<(&str, u64)> {
        match self.maps.get(src_category) {
            Some(map) => Some((&map.destination_name, map.map(number))),
            None => None,
        }
    }
}

fn parse_ints<T: FromStr>(text: &str) -> Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    let ints_regex = Regex::new(r"\d+").unwrap();
    ints_regex
        .find_iter(text)
        .map(|num| num.as_str().parse().unwrap())
        .collect()
}

fn parse_data(input: &str) -> Data {
    let mut data = Data::new();
    let seeds_regex = Regex::new(r"seeds:(?<seed_numbers>.+)").unwrap();
    let map_names_regex = Regex::new(r"(?<src_name>\w+)-to-(?<dst_name>\w+)\s+map:").unwrap();
    let last_map =
        input
            .lines()
            .filter(|s| !s.trim().is_empty())
            .fold(Map::new(), |mut map: Map, line| {
                if let Some(seed_capture) = seeds_regex.captures(line) {
                    let seeds: Vec<u64> =
                        parse_ints(seed_capture.name("seed_numbers").unwrap().as_str());
                    data.seeds.extend(seeds);
                } else if let Some(map_names_capture) = map_names_regex.captures(line) {
                    if !map.source_name.is_empty() && !map.destination_name.is_empty() {
                        data.maps.insert(map.source_name.to_owned(), map);
                    }
                    let source_name = map_names_capture
                        .name("src_name")
                        .unwrap()
                        .as_str()
                        .to_owned();
                    let destination_name = map_names_capture
                        .name("dst_name")
                        .unwrap()
                        .as_str()
                        .to_owned();
                    return Map {
                        source_name,
                        destination_name,
                        ranges: Vec::new(),
                    };
                } else if let [dst, src, len] = parse_ints(line)[..] {
                    map.ranges.push(RangeMap::new(src, dst, len));
                }
                map
            });
    data.maps.insert(last_map.source_name.to_owned(), last_map);
    data
}

fn part1(data: &Data) {
    let mut locations = Vec::new();
    for seed in &data.seeds {
        let mut category = "seed";
        let mut number = *seed;

        while let Some((category_, number_)) = data.map(category, number) {
            category = category_;
            number = number_;
        }
        locations.push(number);
    }
    let min_location = locations.iter().min().unwrap();
    println!("Part 1: {min_location}");
}

fn part2(data: &Data) {
    let mut min_location = None;
    for (seed_start, length) in data
        .seeds
        .iter()
        .step_by(2)
        .zip(data.seeds.iter().skip(1).step_by(2))
    {
        println!("{seed_start} {length}");
        for seed in *seed_start..seed_start + length {
            let mut category = "seed";
            let mut number = seed;

            while let Some((category_, number_)) = data.map(category, number) {
                category = category_;
                number = number_;
            }
            min_location = match min_location {
                None => Some(number),
                Some(loc) => Some(number.min(loc)),
            };
        }
    }
    let min_location = min_location.unwrap_or(0);
    println!("Part 2: {min_location}");
}
