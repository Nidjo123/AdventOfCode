use std::collections::{HashSet, VecDeque};
use std::fs;

fn main() {
    let input = fs::read_to_string("src/input").unwrap();
    let map = Map::from(&input);
    part1(&map);
}

const POS_DIFFS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

#[derive(Debug, PartialEq, Eq, Hash)]
enum Tile {
    Ground,     // .
    Vertical,   // |
    Horizontal, // -
    NorthEast,  // L
    NorthWest,  // J
    SouthEast,  // F
    SouthWest,  // 7
    Start,      // S
}

impl Tile {
    fn can_connect(tile: &Tile, pos: &Position, other_tile: &Tile, other_pos: &Position) -> bool {
        let row_diff = pos.0.abs_diff(other_pos.0);
        let col_diff = pos.1.abs_diff(other_pos.1);
        if !((row_diff == 1 && col_diff == 0) || (row_diff == 0 && col_diff == 1)) {
            return false;
        }
        use Tile::*;
        let (row, col) = (pos.0, pos.1);
        let (other_row, other_col) = (other_pos.0, other_pos.1);
        match (&tile, &other_tile) {
            (Vertical, Vertical | Start) => col == other_col,
            (Vertical, SouthWest | SouthEast) => row > other_row,
            (Vertical, NorthWest | NorthEast) => row < other_row,
            (Horizontal, Horizontal | Start) => row == other_row,
            (Horizontal, SouthEast | NorthEast) => col > other_col,
            (Horizontal, SouthWest | NorthWest) => col < other_col,
            (NorthEast, Start) => row > other_row || col < other_col,
            (NorthWest, Start) => row > other_row || col > other_col,
            (SouthEast, Start) => row < other_row || col < other_col,
            (SouthWest, Start) => row < other_row || col > other_col,
            (NorthEast, NorthWest) => col < other_col,
            (NorthEast, SouthEast) => row > other_row,
            (SouthEast, SouthWest) => col < other_col,
            (NorthEast, SouthWest) => row > other_row || col < other_col,
            (NorthWest, SouthWest) => row > other_row,
            (NorthWest, SouthEast) => row > other_row || col > other_col,
            (NorthEast, NorthEast) => false,
            (NorthWest, NorthWest) => false,
            (SouthEast, SouthEast) => false,
            (SouthWest, SouthWest) => false,
            (Vertical, Horizontal) => false,
            (Ground, _) | (_, Ground) => false,
            (_, _) => Self::can_connect(other_tile, other_pos, tile, pos),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    fn read_test_map1() -> Map {
        let input = fs::read_to_string("src/test1").unwrap();
        Map::from(&input)
    }

    fn read_test_map2() -> Map {
        let input = fs::read_to_string("src/test2").unwrap();
        Map::from(&input)
    }

    #[test]
    fn test_map_reading() {
        let map1 = read_test_map1();
        assert_eq!(map1.rows(), 5);
        assert_eq!(map1.cols(), 5);
        assert_eq!(map1.find_start_position(), Some(Position(1, 1)));

        let map2 = read_test_map2();
        assert_eq!(map2.rows(), 5);
        assert_eq!(map2.cols(), 5);
        assert_eq!(map2.find_start_position(), Some(Position(2, 0)));
    }

    #[test]
    fn test_start_connectivity() {
        let map = read_test_map1();
        let start_pos = map.find_start_position().unwrap();
        let start_tile = map.get_tile(&start_pos);
        assert_eq!(start_tile, &Tile::Start);

        let right_pos = Position(start_pos.0, start_pos.1 + 1);
        let right_tile = map.get_tile(&right_pos);
        assert_eq!(right_tile, &Tile::Horizontal);
        assert!(Tile::can_connect(
            start_tile, &start_pos, right_tile, &right_pos
        ));

        let left_pos = Position(start_pos.0, start_pos.1 - 1);
        let left_tile = map.get_tile(&left_pos);
        assert_eq!(left_tile, &Tile::SouthWest);
        assert!(!Tile::can_connect(
            start_tile, &start_pos, left_tile, &left_pos
        ));

        let up_pos = Position(start_pos.0 - 1, start_pos.1);
        let up_tile = map.get_tile(&up_pos);
        assert_eq!(up_tile, &Tile::NorthEast);
        assert!(!Tile::can_connect(start_tile, &start_pos, up_tile, &up_pos));

        let down_pos = Position(start_pos.0 + 1, start_pos.1);
        let down_tile = map.get_tile(&down_pos);
        assert_eq!(down_tile, &Tile::Vertical);
        assert!(Tile::can_connect(
            start_tile, &start_pos, down_tile, &down_pos
        ));
    }

    #[test]
    fn test_farthest_distance() {
        let map = read_test_map1();
        assert_eq!(map.find_farthest_tile_from_start(), 4);

        let map = read_test_map2();
        assert_eq!(map.find_farthest_tile_from_start(), 8);
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Position(usize, usize);

struct Map {
    tiles: Vec<Vec<Tile>>,
}

impl Map {
    fn from(s: &str) -> Self {
        let rows: Vec<Vec<Tile>> = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' => Tile::Ground,
                        '|' => Tile::Vertical,
                        '-' => Tile::Horizontal,
                        'L' => Tile::NorthEast,
                        'J' => Tile::NorthWest,
                        'F' => Tile::SouthEast,
                        '7' => Tile::SouthWest,
                        'S' => Tile::Start,
                        _ => panic!("unknown tile: '{c}'"),
                    })
                    .collect()
            })
            .collect();
        if rows.len() > 1 {
            let first_row_len = rows.first().unwrap().len();
            assert!(rows.iter().all(|row| row.len() == first_row_len));
        }
        Self { tiles: rows }
    }

    fn rows(&self) -> usize {
        self.tiles.len()
    }
    fn cols(&self) -> usize {
        self.tiles[0].len()
    }

    fn get_tile(&self, pos: &Position) -> &Tile {
        assert!(pos.0 < self.rows() && pos.1 < self.cols());
        &self.tiles[pos.0][pos.1]
    }

    fn find_start_position(&self) -> Option<Position> {
        for (row_idx, row) in self.tiles.iter().enumerate() {
            if let Some(col_idx) = row.iter().position(|tile| tile == &Tile::Start) {
                return Some(Position(row_idx, col_idx));
            }
        }
        None
    }

    fn find_farthest_tile_from_start(&self) -> usize {
        let start_pos = self.find_start_position().expect("no start position");
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((start_pos, 0));
        let mut max_distance = 0;
        while !queue.is_empty() {
            if let Some((pos, distance)) = queue.pop_front() {
                let tile = self.get_tile(&pos);
                for (row_diff, col_diff) in POS_DIFFS {
                    let new_pos = (pos.0 as isize + row_diff, pos.1 as isize + col_diff);
                    if new_pos.0 < 0
                        || new_pos.1 < 0
                        || new_pos.0 >= self.rows() as isize
                        || new_pos.1 >= self.cols() as isize
                    {
                        continue;
                    }

                    let new_pos = Position(new_pos.0 as usize, new_pos.1 as usize);
                    if !visited.contains(&new_pos)
                        && Tile::can_connect(tile, &pos, self.get_tile(&new_pos), &new_pos)
                    {
                        queue.push_back((new_pos, distance + 1));
                    }
                }
                max_distance = max_distance.max(distance);
                visited.insert(pos);
            }
        }
        max_distance
    }
}

fn part1(map: &Map) {
    let farthest_distance = map.find_farthest_tile_from_start();
    println!("Part 1: {farthest_distance}");
}
