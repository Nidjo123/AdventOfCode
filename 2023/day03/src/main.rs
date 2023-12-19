use regex::Regex;

fn main() {
    let input = std::fs::read_to_string("src/input").unwrap();
    part1(&input);
}

#[derive(Debug)]
struct Location {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct PartNumber {
    loc: Location,
    length: usize,
    number: usize,
}

impl PartNumber {
    fn is_adjacent_to(&self, loc: &Location) -> bool {
        if self.loc.y.abs_diff(loc.y) <= 1 {
            (self.loc.x..self.loc.x + self.length).any(|x| {
                x.abs_diff(loc.x) <= 1
            })
        } else {
            false
        }
    }
}

fn part1(input: &str) {
    let int_regex = Regex::new(r"\d+").unwrap();
    let part_numbers: Vec<PartNumber> = input.lines()
        .enumerate()
        .map(|(y, line)| {
            int_regex.find_iter(line).map(|m| PartNumber {
                loc: Location { x: m.start(), y },
                length: m.range().len(),
                number: m.as_str().parse::<usize>().unwrap(),
            }).collect::<Vec<_>>()
        })
        .flatten()
        .collect();

    let symbol_regex = Regex::new(r"[^\d.]").unwrap();
    let symbols: Vec<Location> = input.lines()
        .enumerate()
        .map(|(y, line)|
            symbol_regex.find_iter(line).map(move |m| Location { x: m.start(), y })
        ).flatten()
        .collect();

    let part_num_sum: usize = part_numbers.iter().filter_map(|part_number| {
        if symbols.iter().any(|symbol_loc| part_number.is_adjacent_to(symbol_loc)) {
            Some(part_number.number)
        } else {
            None
        }
    }).sum();

    println!("Part 1: {part_num_sum}");
}
