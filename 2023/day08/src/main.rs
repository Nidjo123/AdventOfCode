use std::collections::HashMap;
use std::fs;

use regex::Regex;

fn main() {
    let input = fs::read_to_string("src/input").unwrap();
    let data = parse_data(&input);
    part1(&data);
}

enum Direction {
    RIGHT,
    LEFT,
}

struct Node {
    pub name: String,
    pub left: String,
    pub right: String,
}

struct Data {
    directions: Vec<Direction>,
    nodes: Vec<Node>,
}

fn build_node_map(nodes: &Vec<Node>) -> HashMap<String, &Node> {
    let mut map = HashMap::new();
    for node in nodes {
        map.insert(node.name.to_owned(), node);
    }
    map
}

fn parse_data(input: &str) -> Data {
    let mut lines = input.lines();
    let directions: Vec<_> = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'R' => Direction::RIGHT,
            'L' => Direction::LEFT,
            _ => panic!("unknown direction: {c}"),
        })
        .collect();
    let node_descriptions: Vec<_> = lines.skip(1).collect();

    let node_desc_regex =
        Regex::new(r"(?<name>\w+)\s*=\s*\((?<left>\w+)\s*,\s*(?<right>\w+)\)").unwrap();
    let nodes = node_descriptions
        .iter()
        .map(|&desc| {
            let captures = node_desc_regex.captures(desc).unwrap();
            let name = captures.name("name").unwrap().as_str().to_owned();
            let left = captures.name("left").unwrap().as_str().to_owned();
            let right = captures.name("right").unwrap().as_str().to_owned();
            Node { name, left, right }
        })
        .collect();

    Data { directions, nodes }
}

fn part1(data: &Data) {
    let node_map = build_node_map(&data.nodes);
    let mut steps = 0;
    let mut node = node_map.get("AAA").unwrap();
    for direction in data.directions.iter().cycle() {
        node = match direction {
            Direction::RIGHT => node_map.get(&node.right).unwrap(),
            Direction::LEFT => node_map.get(&node.left).unwrap(),
        };
        steps += 1;
        if node.name == "ZZZ" {
            break;
        }
    }
    println!("Part 1: {steps}");
}
