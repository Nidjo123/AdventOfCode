use std::fs;

fn main() {
    let patterns = read_patterns("input");
    part1(&patterns);
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
    fn search_lines(lines: &Vec<String>) -> Option<usize> {
        for (line_idx, _line) in lines.iter().enumerate().take(lines.len() - 1) {
            let first_part: Vec<_> = lines.iter().take(line_idx + 1).rev().collect();
            let second_part: Vec<_> = lines.iter().skip(line_idx + 1).collect();
            if first_part.len() > 0
                && second_part.len() > 0
                && first_part
                    .iter()
                    .zip(second_part.iter())
                    .all(|(x, y)| x == y)
            {
                return Some(line_idx);
            }
        }
        None
    }

    fn search_horizontal_lines(&self) -> Option<usize> {
        Self::search_lines(&self.pattern)
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

    fn search_vertical_lines(&self) -> Option<usize> {
        let columns: Vec<_> = (0..self.number_of_columns())
            .map(|col_idx| self.get_column(col_idx))
            .collect();
        Self::search_lines(&columns)
    }

    fn mirror_line(&self) -> Option<MirrorLine> {
        if let Some(row) = self.search_horizontal_lines() {
            Some(MirrorLine {
                line_type: MirrorLineType::Row,
                n: row,
            })
        } else if let Some(column) = self.search_vertical_lines() {
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
            Pattern::search_lines(&vec![String::from(".."), String::from("#.")]),
            None
        );

        assert_eq!(
            Pattern::search_lines(&vec![String::from(".."), String::from("..")]),
            Some(0)
        );

        assert_eq!(
            Pattern::search_lines(&vec![String::from(".#"), String::from(".#")]),
            Some(0)
        );

        assert_eq!(
            Pattern::search_lines(&vec![
                String::from(".#"),
                String::from("#."),
                String::from("#."),
                String::from(".#")
            ]),
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
            pat1.mirror_line(),
            Some(MirrorLine {
                line_type: MirrorLineType::Column,
                n: 4
            })
        );
        assert_eq!(
            pat2.mirror_line(),
            Some(MirrorLine {
                line_type: MirrorLineType::Row,
                n: 3
            })
        );
    }
}

fn part1(patterns: &Vec<Pattern>) {
    let mut cols_to_left = 0;
    let mut rows_above = 0;
    for pattern in patterns {
        match pattern.mirror_line() {
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
