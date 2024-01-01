use std::fs;

fn main() {
    let patterns = read_patterns("input");
    part1(&patterns);
    part2(&patterns);
}

fn read_patterns(name: &str) -> Vec<Pattern> {
    let name = format!("src/{name}");
    let input = fs::read_to_string(name).unwrap();
    input
        .lines()
        .collect::<Vec<_>>()
        .split(|line| line.trim().is_empty())
        .map(|pattern| {
            pattern
                .into_iter()
                .map(|pat| pat.trim().to_owned())
                .collect()
        })
        .map(|pattern| Pattern { pattern })
        .collect()
}

struct Pattern {
    pattern: Vec<String>,
}

#[derive(Debug, PartialEq)]
enum MirrorLineType {
    Row,
    Column,
}

#[derive(PartialEq, Debug)]
struct MirrorLine {
    line_type: MirrorLineType,
    n: usize,
}

impl Pattern {
    fn num_diffs(s1: &str, s2: &str) -> usize {
        s1.chars().zip(s2.chars()).filter(|(x, y)| x != y).count()
    }

    fn search_lines(lines: &Vec<String>) -> Vec<usize> {
        let mut number_of_diffs = Vec::new();
        for (line_idx, _line) in lines.iter().enumerate().take(lines.len() - 1) {
            let first_part: Vec<_> = lines.iter().take(line_idx + 1).rev().collect();
            let second_part: Vec<_> = lines.iter().skip(line_idx + 1).collect();
            if first_part.len() > 0 && second_part.len() > 0 {
                let n = first_part
                    .iter()
                    .zip(second_part.iter())
                    .map(|(x, y)| Self::num_diffs(x, y))
                    .sum();
                number_of_diffs.push(n);
            }
        }
        number_of_diffs
    }

    fn find_idx_with_n_diffs(diffs: &Vec<usize>, num_diffs: usize) -> Option<usize> {
        diffs
            .iter()
            .enumerate()
            .find_map(|(idx, &diffs)| if diffs == num_diffs { Some(idx) } else { None })
    }

    fn search_horizontal_lines(&self, num_diffs: usize) -> Option<usize> {
        let diffs = &Self::search_lines(&self.pattern);
        Self::find_idx_with_n_diffs(diffs, num_diffs)
    }

    fn number_of_columns(&self) -> usize {
        self.pattern.first().unwrap_or(&String::from("")).len()
    }

    fn get_column(&self, col_idx: usize) -> String {
        assert!(col_idx < self.number_of_columns());
        self.pattern
            .iter()
            .map(|row| row.chars().nth(col_idx).unwrap())
            .collect()
    }

    fn search_vertical_lines(&self, num_diffs: usize) -> Option<usize> {
        let columns: Vec<_> = (0..self.number_of_columns())
            .map(|col_idx| self.get_column(col_idx))
            .collect();

        let diffs = &Self::search_lines(&columns);
        Self::find_idx_with_n_diffs(diffs, num_diffs)
    }

    fn mirror_line(&self, num_diffs: usize) -> Option<MirrorLine> {
        if let Some(row) = self.search_horizontal_lines(num_diffs) {
            Some(MirrorLine {
                line_type: MirrorLineType::Row,
                n: row,
            })
        } else if let Some(column) = self.search_vertical_lines(num_diffs) {
            Some(MirrorLine {
                line_type: MirrorLineType::Column,
                n: column,
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_lines() {
        assert_eq!(
            Pattern::find_idx_with_n_diffs(
                &Pattern::search_lines(&vec![String::from(".."), String::from("#.")]),
                0
            ),
            None
        );

        assert_eq!(
            Pattern::find_idx_with_n_diffs(
                &Pattern::search_lines(&vec![String::from(".."), String::from("..")]),
                0
            ),
            Some(0)
        );

        assert_eq!(
            Pattern::find_idx_with_n_diffs(
                &Pattern::search_lines(&vec![String::from(".#"), String::from(".#")]),
                0
            ),
            Some(0)
        );

        assert_eq!(
            Pattern::find_idx_with_n_diffs(
                &Pattern::search_lines(&vec![
                    String::from(".#"),
                    String::from("#."),
                    String::from("#."),
                    String::from(".#")
                ]),
                0
            ),
            Some(1)
        );
    }

    #[test]
    fn test_example() {
        let patterns = read_patterns("test");
        let [pat1, pat2] = &patterns[..] else {
            panic!("unexpected number of patterns")
        };
        assert_eq!(
            pat1.mirror_line(0),
            Some(MirrorLine {
                line_type: MirrorLineType::Column,
                n: 4
            })
        );
        assert_eq!(
            pat2.mirror_line(0),
            Some(MirrorLine {
                line_type: MirrorLineType::Row,
                n: 3
            })
        );

        assert_eq!(
            pat1.mirror_line(1),
            Some(MirrorLine {
                line_type: MirrorLineType::Row,
                n: 2
            })
        );
    }
}

fn part1(patterns: &Vec<Pattern>) {
    let mut cols_to_left = 0;
    let mut rows_above = 0;
    for pattern in patterns {
        match pattern.mirror_line(0) {
            Some(MirrorLine { line_type, n }) => match line_type {
                MirrorLineType::Column => cols_to_left += n + 1,
                MirrorLineType::Row => rows_above += n + 1,
            },
            None => panic!("no mirror found"),
        }
    }
    let res = cols_to_left + rows_above * 100;
    println!("Part 1: {res}");
}

fn part2(patterns: &Vec<Pattern>) {
    let mut cols_to_left = 0;
    let mut rows_above = 0;
    for pattern in patterns {
        match pattern.mirror_line(1) {
            Some(MirrorLine { line_type, n }) => match line_type {
                MirrorLineType::Column => cols_to_left += n + 1,
                MirrorLineType::Row => rows_above += n + 1,
            },
            None => panic!("no mirror found"),
        }
    }
    let res = cols_to_left + rows_above * 100;
    println!("Part 2: {res}");
}
