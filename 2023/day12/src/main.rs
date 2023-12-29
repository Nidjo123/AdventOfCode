use std::fs;

fn main() {
    let records = read_records("input");
    part1(&records);
}

fn read_records(name: &str) -> Vec<ConditionRecord> {
    let filename = format!("src/{name}");
    let input = fs::read_to_string(filename).expect("no records found");
    parse_records(&input)
}

fn parse_records(input: &str) -> Vec<ConditionRecord> {
    input.lines().map(ConditionRecord::from_str).collect()
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

impl Condition {
    fn from_char(c: char) -> Self {
        use Condition::*;
        match c {
            '.' => Operational,
            '#' => Damaged,
            '?' => Unknown,
            _ => panic!("unknown condition: {c}"),
        }
    }
}

#[derive(Debug)]
struct ConditionRecord {
    states: Vec<Condition>,
    damaged_groups: Vec<usize>,
}

impl ConditionRecord {
    fn from_str(s: &str) -> Self {
        let split: Vec<_> = s.split_whitespace().collect();
        if let [states, damaged_groups] = split[..] {
            let states = states.chars().map(Condition::from_char).collect();
            let damaged_groups = damaged_groups
                .split(',')
                .map(|x| x.parse().unwrap())
                .collect();
            ConditionRecord {
                states,
                damaged_groups,
            }
        } else {
            panic!("invalid data format: {s}")
        }
    }

    fn verify(&self, states: &Vec<Condition>) -> bool {
        states
            .split(|c| c == &Condition::Operational)
            .filter(|x| !x.is_empty())
            .map(|x| x.len())
            .collect::<Vec<_>>()
            == self.damaged_groups
    }

    fn fill_and_count(&self, states: &mut Vec<Condition>) -> usize {
        if let Some(unknown_index) = states.iter().position(|c| c == &Condition::Unknown) {
            let res = [Condition::Operational, Condition::Damaged]
                .into_iter()
                .map(|condition| {
                    states[unknown_index] = condition;
                    self.fill_and_count(states)
                })
                .sum();
            states[unknown_index] = Condition::Unknown;
            res
        } else {
            if self.verify(&states) {
                1
            } else {
                0
            }
        }
    }

    fn count_possible_arrangements(&self) -> usize {
        let mut states = self.states.clone();
        self.fill_and_count(&mut states)
    }
}

fn part1(records: &Vec<ConditionRecord>) {
    let res: usize = records
        .iter()
        .map(ConditionRecord::count_possible_arrangements)
        .sum();
    println!("Part 1: {res}");
}

#[cfg(test)]
mod tests {
    use crate::{read_records, Condition, ConditionRecord};

    #[test]
    fn test_verify() {
        let record = ConditionRecord {
            states: vec![Condition::Operational],
            damaged_groups: vec![],
        };
        assert!(record.verify(&record.states));

        let record = ConditionRecord {
            states: vec![Condition::Operational],
            damaged_groups: vec![1],
        };
        assert!(!record.verify(&record.states));

        let record = ConditionRecord {
            states: vec![Condition::Damaged],
            damaged_groups: vec![1],
        };
        assert!(record.verify(&record.states));

        let record = ConditionRecord {
            states: vec![Condition::Damaged, Condition::Damaged],
            damaged_groups: vec![1],
        };
        assert!(!record.verify(&record.states));

        let record = ConditionRecord {
            states: vec![Condition::Operational, Condition::Damaged],
            damaged_groups: vec![1],
        };
        assert!(record.verify(&record.states));

        let record = ConditionRecord {
            states: vec![Condition::Damaged, Condition::Operational],
            damaged_groups: vec![1],
        };
        assert!(record.verify(&record.states));

        let record = ConditionRecord {
            states: vec![Condition::Damaged, Condition::Damaged],
            damaged_groups: vec![2],
        };
        assert!(record.verify(&record.states));
    }

    #[test]
    fn test_simple() {
        let record = ConditionRecord {
            states: vec![Condition::Unknown],
            damaged_groups: vec![],
        };
        assert_eq!(record.count_possible_arrangements(), 1);

        let record = ConditionRecord {
            states: vec![Condition::Unknown],
            damaged_groups: vec![1],
        };
        assert_eq!(record.count_possible_arrangements(), 1);

        let record = ConditionRecord {
            states: vec![Condition::Damaged],
            damaged_groups: vec![1],
        };
        assert_eq!(record.count_possible_arrangements(), 1);
    }

    #[test]
    fn test_example() {
        let records = read_records("test");
        let results: Vec<_> = records
            .iter()
            .map(ConditionRecord::count_possible_arrangements)
            .collect();
        assert_eq!(results, vec![1, 4, 1, 1, 4, 10]);
    }
}
