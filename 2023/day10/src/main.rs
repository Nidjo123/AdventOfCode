use std::collections::{HashSet, VecDeque};
use std::fs;

fn main() {
    let input = fs::read_to_string("src/input").unwrap();
    let map = Map::from(&input);
    part1(&map);
    part2(&map);
}

const POS_DIFFS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Tile {
    Ground,     // .
    Vertical,   // |
    Horizontal, // -
    NorthEast,  // L
    NorthWest,  // J
    SouthEast,  // F
    SouthWest,  // 7
    Start,      // S
    Pipeline,
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

    fn side_diffs(&self, prev_pos: &Position, pos: &Position) -> Vec<(isize, isize)> {
        use Tile::*;
        let (prev_row, prev_col) = (prev_pos.0, prev_pos.1);
        let (row, col) = (pos.0, pos.1);
        match self {
            Vertical => {
                if prev_row > row {
                    vec![(0, 1)]
                } else {
                    vec![(0, -1)]
                }
            }
            Horizontal => {
                if prev_col < col {
                    vec![(1, 0)]
                } else {
                    vec![(-1, 0)]
                }
            }
            NorthEast => {
                // L
                if prev_col > col {
                    vec![]
                } else {
                    vec![(1, 0), (0, -1)]
                }
            }
            NorthWest => {
                // J
                if prev_col < col {
                    vec![(0, 1), (1, 0)]
                } else {
                    vec![]
                }
            }
            SouthEast => {
                // F
                if prev_col > col {
                    vec![(-1, 0), (0, -1)]
                } else {
                    vec![]
                }
            }
            SouthWest => {
                // 7
                if prev_col < col {
                    vec![]
                } else {
                    vec![(0, 1), (-1, 0)]
                }
            }
            Start => Vec::from(POS_DIFFS),
            _ => vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    fn read_map(name: &str) -> Map {
        let path = format!("src/{name}");
        let input = fs::read_to_string(path).expect("unknown map");
        Map::from(&input)
    }

    #[test]
    fn test_map_reading() {
        let map1 = read_map("test1");
        assert_eq!(map1.rows(), 5);
        assert_eq!(map1.cols(), 5);
        assert_eq!(map1.find_start_position(), Some(Position(1, 1)));

        let map2 = read_map("test2");
        assert_eq!(map2.rows(), 5);
        assert_eq!(map2.cols(), 5);
        assert_eq!(map2.find_start_position(), Some(Position(2, 0)));
    }

    #[test]
    fn test_start_connectivity() {
        let map = read_map("test1");
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
        let map = read_map("test1");
        assert_eq!(map.max_distance_from_start(), 4);

        let map = read_map("test2");
        assert_eq!(map.max_distance_from_start(), 8);
    }

    #[test]
    fn test_enclosed_tiles() {
        assert_eq!(read_map("test1").count_enclosed_tiles(), 1);
        assert_eq!(read_map("test3").count_enclosed_tiles(), 4);
        assert_eq!(read_map("test4").count_enclosed_tiles(), 8);
        assert_eq!(read_map("test5").count_enclosed_tiles(), 10);
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
        self.tiles.first().map_or_else(|| 0, |v| v.len())
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

    fn validate_position(&self, new_pos: (isize, isize)) -> Option<Position> {
        if new_pos.0 < 0
            || new_pos.1 < 0
            || new_pos.0 >= self.rows() as isize
            || new_pos.1 >= self.cols() as isize
        {
            None
        } else {
            Some(Position(new_pos.0 as usize, new_pos.1 as usize))
        }
    }

    fn for_each_pipeline_element_bfs(&self, mut callback: impl FnMut(&Position, &Tile, usize)) {
        let start_pos = self.find_start_position().expect("no start position");
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((start_pos, 0));
        while !queue.is_empty() {
            if let Some((pos, distance)) = queue.pop_front() {
                let tile = self.get_tile(&pos);

                callback(&pos, tile, distance);

                for (row_diff, col_diff) in POS_DIFFS {
                    let new_pos = self
                        .validate_position((pos.0 as isize + row_diff, pos.1 as isize + col_diff));
                    if new_pos.is_none() {
                        continue;
                    }
                    let new_pos = new_pos.unwrap();
                    if !visited.contains(&new_pos)
                        && Tile::can_connect(tile, &pos, self.get_tile(&new_pos), &new_pos)
                    {
                        queue.push_back((new_pos, distance + 1));
                    }
                }
                visited.insert(pos);
            }
        }
    }

    fn max_distance_from_start(&self) -> usize {
        let mut max_distance = 0;
        self.for_each_pipeline_element_bfs(|_pos, _tile, distance| {
            max_distance = max_distance.max(distance)
        });
        max_distance
    }

    fn marked_pipeline_map(&self) -> Vec<Vec<Tile>> {
        let mut tiles = Vec::new();
        self.tiles.iter().for_each(|row| tiles.push(row.clone()));
        self.for_each_pipeline_element_bfs(|pos, _tile, _distance| {
            tiles[pos.0][pos.1] = Tile::Pipeline
        });
        tiles
    }

    fn for_each_pipeline_element_dfs(&self, mut callback: impl FnMut(&Position, &Position)) {
        let start_pos = self.find_start_position().expect("no start position");
        let mut visited = HashSet::new();
        let mut queue: Vec<(Option<Position>, Position)> = vec![(None, start_pos)];
        while !queue.is_empty() {
            if let Some((prev_pos, pos)) = queue.pop() {
                if let Some(prev_pos) = prev_pos {
                    callback(&prev_pos, &pos);
                }
                for (diff_row, diff_col) in POS_DIFFS {
                    let new_row = pos.0 as isize + diff_row;
                    let new_col = pos.1 as isize + diff_col;
                    let new_pos = self.validate_position((new_row, new_col));
                    if new_pos.is_none() {
                        continue;
                    }
                    let new_pos = new_pos.unwrap();
                    if !visited.contains(&new_pos)
                        && Tile::can_connect(
                            self.get_tile(&pos),
                            &pos,
                            self.get_tile(&new_pos),
                            &new_pos,
                        )
                    {
                        queue.push((Some(pos), new_pos));
                        break;
                    }
                }
                visited.insert(pos);
            }
        }
    }

    fn get_connected_tiles(
        &self,
        start_pos: &Position,
        marked_map: &Vec<Vec<Tile>>,
    ) -> HashSet<Position> {
        let mut connected_tiles = HashSet::from([*start_pos]);
        let mut queue = vec![*start_pos];
        while !queue.is_empty() {
            if let Some(pos) = queue.pop() {
                for (row_diff, col_diff) in POS_DIFFS {
                    if let Some(new_pos) = self
                        .validate_position((pos.0 as isize + row_diff, pos.1 as isize + col_diff))
                    {
                        if marked_map[new_pos.0][new_pos.1] != Tile::Pipeline
                            && !connected_tiles.contains(&new_pos)
                        {
                            queue.push(new_pos);
                        }
                    }
                }
                connected_tiles.insert(pos);
            }
        }
        connected_tiles
    }

    #[allow(unused)]
    fn print_marked_pipeline(&self) {
        println!("=========");
        for (row_idx, row) in self.marked_pipeline_map().iter().enumerate() {
            let row_chars = row
                .iter()
                .enumerate()
                .map(|(col_idx, &x)| match x {
                    Tile::Pipeline => "X",
                    _ => ".",
                })
                .collect::<Vec<_>>();
            println!("{}", row_chars.join(""));
        }
        println!("=========");
    }

    fn count_enclosed_tiles(&self) -> usize {
        let marked_map = self.marked_pipeline_map();
        let mut one_side_tiles = HashSet::new();
        self.for_each_pipeline_element_dfs(|prev_pos, pos| {
            let tile = self.get_tile(pos);
            for (diff_row, diff_col) in tile.side_diffs(prev_pos, pos) {
                if let Some(side_pos) =
                    self.validate_position((pos.0 as isize + diff_row, pos.1 as isize + diff_col))
                {
                    if marked_map[side_pos.0][side_pos.1] != Tile::Pipeline {
                        one_side_tiles.insert(side_pos);
                    }
                }
            }
        });

        let more_one_side_tiles: HashSet<_> = one_side_tiles
            .iter()
            .map(|pos| self.get_connected_tiles(pos, &marked_map))
            .flatten()
            .collect();
        one_side_tiles.extend(more_one_side_tiles);

        let mut pipeline_count: usize = 0;
        let mut other_side_tiles = HashSet::new();
        for (row_idx, row) in marked_map.iter().enumerate() {
            for (col_idx, _) in row.iter().enumerate() {
                if marked_map[row_idx][col_idx] == Tile::Pipeline {
                    pipeline_count += 1;
                    continue;
                }
                let position = Position(row_idx, col_idx);
                if !one_side_tiles.contains(&position) {
                    other_side_tiles.insert(position);
                }
            }
        }

        assert_eq!(
            one_side_tiles.len() + other_side_tiles.len() + pipeline_count,
            self.rows() * self.cols()
        );

        if one_side_tiles.iter().any(|&pos| {
            pos.0 == 0 || pos.1 == 0 || pos.0 == self.rows() - 1 || pos.1 == self.cols() - 1
        }) {
            other_side_tiles.len()
        } else {
            one_side_tiles.len()
        }
    }
}

fn part1(map: &Map) {
    let farthest_distance = map.max_distance_from_start();
    println!("Part 1: {farthest_distance}");
}

fn part2(map: &Map) {
    let enclosed_tile_count = map.count_enclosed_tiles();
    println!("Part 2: {enclosed_tile_count}");
}
