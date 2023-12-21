use regex::Regex;

fn main() {
    let input = std::fs::read_to_string("src/input").unwrap();
    let (part_numbers, symbols) = parse_data(&input);
    part1(&part_numbers, &symbols);
    part2(&part_numbers, &symbols);
}

#[derive(Debug)]
struct Location {
    x: usize,
    y: usize,
}

struct Symbol {
    loc: Location,
    symbol: char,
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
            (self.loc.x..self.loc.x + self.length).any(|x| x.abs_diff(loc.x) <= 1)
        } else {
            false
        }
    }
}

fn parse_data(input: &str) -> (Vec<PartNumber>, Vec<Symbol>) {
    let int_regex = Regex::new(r"\d+").unwrap();
    let part_numbers: Vec<PartNumber> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            int_regex
                .find_iter(line)
                .map(|m| PartNumber {
                    loc: Location { x: m.start(), y },
                    length: m.range().len(),
                    number: m.as_str().parse::<usize>().unwrap(),
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();

    let symbol_regex = Regex::new(r"[^\d.]").unwrap();
    let symbols: Vec<Symbol> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            symbol_regex.find_iter(line).map(move |m| Symbol {
                loc: Location { x: m.start(), y },
                symbol: m.as_str().as_bytes()[0] as char,
            })
        })
        .flatten()
        .collect();

    (part_numbers, symbols)
}

fn part1(part_numbers: &Vec<PartNumber>, symbols: &Vec<Symbol>) {
    let part_num_sum: usize = part_numbers
        .iter()
        .filter_map(|part_number| {
            if symbols
                .iter()
                .any(|symbol| part_number.is_adjacent_to(&symbol.loc))
            {
                Some(part_number.number)
            } else {
                None
            }
        })
        .sum();

    println!("Part 1: {part_num_sum}");
}

fn part2(part_numbers: &Vec<PartNumber>, symbols: &Vec<Symbol>) {
    let gear_ratio_sums: usize = symbols
        .iter()
        .filter(|symbol| symbol.symbol == '*')
        .map(|symbol| {
            let adjacent_part_numbers: Vec<usize> = part_numbers
                .iter()
                .filter_map(|part_number| {
                    if part_number.is_adjacent_to(&symbol.loc) {
                        Some(part_number.number)
                    } else {
                        None
                    }
                })
                .collect();
            if adjacent_part_numbers.len() == 2 {
                adjacent_part_numbers.iter().product()
            } else {
                0
            }
        })
        .sum();

    println!("Part 2: {gear_ratio_sums}");
}
