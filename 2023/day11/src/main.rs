use std::fs;

fn main() {
    let input = fs::read_to_string("src/input").unwrap();
    let galaxy = Galaxy::from(&input);
    part1(&galaxy);
}

struct Galaxy {
    map: Vec<Vec<char>>,
    empty_rows: Vec<usize>,
    empty_cols: Vec<usize>,
}

struct Position(usize, usize);

impl Position {
    fn manhattan_distance(&self, other: &Position) -> usize {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }
}

impl Galaxy {
    fn from(s: &str) -> Self {
        let map: Vec<_> = s
            .lines()
            .map(|line| line.trim().chars().collect::<Vec<char>>())
            .collect();
        let empty_rows = map
            .iter()
            .enumerate()
            .filter_map(|(row_idx, row)| {
                if row.iter().all(|&c| c == '.') {
                    Some(row_idx)
                } else {
                    None
                }
            })
            .collect();
        let empty_cols = (0..map[0].len())
            .filter_map(|col_idx| {
                if map
                    .iter()
                    .all(|row| row.iter().nth(col_idx).unwrap() == &'.')
                {
                    Some(col_idx)
                } else {
                    None
                }
            })
            .collect();
        Self {
            map,
            empty_rows,
            empty_cols,
        }
    }

    fn galaxy_positions(&self) -> Vec<Position> {
        self.map
            .iter()
            .enumerate()
            .map(|(row_idx, row)| {
                row.iter().enumerate().filter_map(move |(col_idx, ch)| {
                    if ch == &'#' {
                        let expanded_rows = self
                            .empty_rows
                            .iter()
                            .filter(|&empty_row_idx| empty_row_idx < &row_idx)
                            .count();
                        let expanded_cols = self
                            .empty_cols
                            .iter()
                            .filter(|&empty_col_idx| empty_col_idx < &col_idx)
                            .count();
                        Some(Position(row_idx + expanded_rows, col_idx + expanded_cols))
                    } else {
                        None
                    }
                })
            })
            .flatten()
            .collect()
    }

    fn manhattan_distance_sum(&self) -> usize {
        let galaxy_positions = self.galaxy_positions();
        galaxy_positions
            .iter()
            .enumerate()
            .map(|(idx, pos)| {
                galaxy_positions
                    .iter()
                    .skip(idx + 1)
                    .map(|other_pos| pos.manhattan_distance(other_pos))
            })
            .flatten()
            .sum()
    }
}

fn part1(galaxy: &Galaxy) {
    let manhattan_distance_sum = galaxy.manhattan_distance_sum();
    println!("Part 1: {manhattan_distance_sum}");
}

#[cfg(test)]
mod tests {
    use crate::Galaxy;
    use std::fs;

    fn read_galaxy(name: &str) -> Galaxy {
        let name = format!("src/{name}");
        let input = fs::read_to_string(name).expect("no galaxy found");
        Galaxy::from(&input)
    }

    #[test]
    fn test_input() {
        let galaxy = read_galaxy("test");
        assert_eq!(galaxy.empty_rows, vec![3, 7]);
        assert_eq!(galaxy.empty_cols, vec![2, 5, 8]);
    }

    #[test]
    fn test_distance_sum() {
        let galaxy = read_galaxy("test");
        assert_eq!(galaxy.manhattan_distance_sum(), 374);
    }
}
