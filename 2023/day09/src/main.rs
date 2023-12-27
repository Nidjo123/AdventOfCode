use std::fs;

fn main() {
    let input = fs::read_to_string("src/input").unwrap();
    let sequences = read_sequences(&input);
    part1(&sequences);
    part2(&sequences);
}

fn read_sequences(input: &str) -> Vec<Sequence> {
    input
        .lines()
        .map(|line| line.split_whitespace().map(|s| s.parse::<i64>().unwrap()))
        .map(|values| Sequence {
            values: values.collect(),
        })
        .collect()
}

type SeqValues = Vec<i64>;

struct Sequence {
    values: SeqValues,
}

impl Sequence {
    fn differences(values: &SeqValues) -> SeqValues {
        values
            .iter()
            .zip(values.iter().skip(1))
            .map(|(x, y)| y - x)
            .collect()
    }

    fn subsequences(&self) -> Vec<SeqValues> {
        let mut res = vec![self.values.clone()];
        while !res.last().unwrap().iter().all(|&x| x == 0) {
            res.push(Self::differences(res.last().unwrap()));
        }
        res
    }

    fn calc_history_value(&self, next_history: fn(i64, &SeqValues) -> i64) -> i64 {
        let subsequences = self.subsequences();
        subsequences.iter().rev().skip(2).fold(
            *subsequences[subsequences.len() - 2].last().unwrap(),
            next_history,
        )
    }

    fn next_history_value(&self) -> i64 {
        self.calc_history_value(|history, sequence| sequence.last().unwrap() + history)
    }

    fn previous_history_value(&self) -> i64 {
        self.calc_history_value(|history, sequence| sequence.first().unwrap() - history)
    }
}

#[test]
fn test_differences() {
    assert_eq!(
        Sequence::differences(&vec![0, 3, 6, 9, 12, 15]),
        vec![3, 3, 3, 3, 3]
    );
    assert_eq!(
        Sequence::differences(&vec![3, 3, 3, 3, 3]),
        vec![0, 0, 0, 0]
    );
    assert_eq!(
        Sequence::differences(&vec![10, 13, 16, 21, 30, 45]),
        vec![3, 3, 5, 9, 15]
    );
}

#[test]
fn test_forward_history() {
    let input = fs::read_to_string("src/test").unwrap();
    let sequences = read_sequences(&input);
    let histories: Vec<_> = sequences
        .iter()
        .map(|seq| seq.next_history_value())
        .collect();
    assert_eq!(histories, vec![18, 28, 68]);
}

#[test]
fn test_backward_history() {
    let input = fs::read_to_string("src/test").unwrap();
    let sequences = read_sequences(&input);
    let histories: Vec<_> = sequences
        .iter()
        .map(|seq| seq.previous_history_value())
        .collect();
    assert_eq!(histories, vec![-3, 0, 5]);
}

fn part1(sequences: &Vec<Sequence>) {
    let res: i64 = sequences.iter().map(|seq| seq.next_history_value()).sum();
    println!("Part 1: {res}");
}

fn part2(sequences: &Vec<Sequence>) {
    let res: i64 = sequences
        .iter()
        .map(|seq| seq.previous_history_value())
        .sum();
    println!("Part 2: {res}");
}
