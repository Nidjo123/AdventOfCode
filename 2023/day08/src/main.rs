use std::collections::{HashMap, HashSet};
use std::fs;

use regex::Regex;

fn main() {
    let input = fs::read_to_string("src/input").unwrap();
    let data = parse_data(&input);
    part1(&data);
    part2(&data);
}

enum Direction {
    RIGHT,
    LEFT,
}

#[derive(Debug)]
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

struct Path {
    steps: Vec<String>,
}

impl Path {
    fn get_cycle_lengths(&self) -> Option<usize> {
        let mut visited_names = HashSet::new();
        for (idx, name) in self.steps.iter().enumerate() {
            if is_end_node(name) && visited_names.contains(name) {
                let first_pos = self.steps.iter().position(|x| x == name).unwrap();
                let length = idx - first_pos;
                let expected_next_cycle_start = idx + length;
                if expected_next_cycle_start < self.steps.len()
                    && self
                        .steps
                        .iter()
                        .skip(idx)
                        .take(length)
                        .zip(self.steps.iter().skip(first_pos).take(length))
                        .all(|(x, y)| x == y)
                {
                    return Some(length);
                }
            }
            visited_names.insert(name);
        }
        None
    }
}

fn is_end_node(name: &str) -> bool {
    name.ends_with("Z")
}

fn add_path_steps(nodes: &Vec<&Node>, paths: &mut Vec<Path>) {
    nodes
        .iter()
        .zip(paths.iter_mut())
        .for_each(|(&node, path)| path.steps.push(node.name.to_owned()));
}

fn prime_factors(mut n: usize) -> Vec<usize> {
    let original_n = n;
    let mut factors = Vec::new();
    let mut factor = 2;
    while n % factor == 0 {
        n /= factor;
        factors.push(factor);
    }
    factor = 3;
    while n > 1 {
        while n % factor == 0 {
            n /= factor;
            factors.push(factor);
        }
        factor += 2;
    }
    assert_eq!(factors.iter().product::<usize>(), original_n);
    factors
}

fn lcm(numbers: &Vec<usize>) -> usize {
    let mut prime_factor_counts: HashMap<usize, usize> = HashMap::new();
    for factors in numbers.iter().map(|&x| prime_factors(x)) {
        let mut counts = HashMap::new();
        for factor in factors {
            *counts.entry(factor).or_insert(0) += 1;
        }
        for (factor, count) in counts {
            let entry = prime_factor_counts.entry(factor).or_insert(count);
            *entry = count.max(*entry);
        }
    }
    let mut res = 1;
    for (factor, count) in prime_factor_counts {
        res *= factor.pow(count as u32);
    }
    res
}

#[test]
fn test_lcm() {
    assert_eq!(lcm(&vec![1]), 1);
    assert_eq!(lcm(&vec![1, 2, 3]), 6);
    assert_eq!(lcm(&vec![6, 2, 3]), 6);
    assert_eq!(lcm(&vec![5]), 5);
    assert_eq!(lcm(&vec![2, 3, 7]), 42);
}

fn part2(data: &Data) {
    let node_map = build_node_map(&data.nodes);
    let mut steps = 0;
    let mut nodes: Vec<_> = data
        .nodes
        .iter()
        .filter(|node| node.name.ends_with("A"))
        .collect();

    let mut paths: Vec<Path> = Vec::with_capacity(nodes.len());
    for _ in &nodes {
        paths.push(Path { steps: Vec::new() });
    }
    add_path_steps(&nodes, &mut paths);

    for direction in data.directions.iter().cycle() {
        let mut new_nodes = nodes
            .iter()
            .map(|node| match direction {
                Direction::RIGHT => *node_map.get(&node.right).unwrap(),
                Direction::LEFT => *node_map.get(&node.left).unwrap(),
            })
            .collect();
        nodes.clear();
        nodes.append(&mut new_nodes);
        steps += 1;
        add_path_steps(&nodes, &mut paths);
        if steps % 10000 == 0 {
            let cycle_lengths: Vec<_> = paths.iter().map(Path::get_cycle_lengths).collect();
            if cycle_lengths.iter().all(Option::is_some) {
                let cycle_lengths: Vec<_> = cycle_lengths.iter().map(|x| x.unwrap()).collect();
                let cycle_length_lcm = lcm(&cycle_lengths);
                println!("Part 2: {cycle_length_lcm}");
                break;
            }
        }
        if nodes.iter().all(|&node| is_end_node(&node.name)) {
            break;
        }
    }
}
