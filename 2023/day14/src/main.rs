use std::fs;

fn main() {
    let map = read_map("input");
    part1(&map);
    part2(&map);
}

fn read_map(name: &str) -> Map {
    let filename = format!("src/{name}");
    let input = fs::read_to_string(filename).expect("map not found");
    Map::from_str(&input)
}

#[derive(PartialEq, Copy, Clone)]
enum TileType {
    Empty,
    RoundedRock,
    CubeRock,
}

#[derive(PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

const CYCLE: [Direction; 4] = [
    Direction::North,
    Direction::West,
    Direction::South,
    Direction::East,
];

struct Map {
    rows: Vec<Vec<TileType>>,
}

impl Map {
    fn from_str(input: &str) -> Self {
        let mut map = Map { rows: Vec::new() };
        for line in input.lines() {
            let row = line
                .trim()
                .chars()
                .map(|ch| match ch {
                    '.' => TileType::Empty,
                    'O' => TileType::RoundedRock,
                    '#' => TileType::CubeRock,
                    _ => panic!("unknown tile type: {ch}"),
                })
                .collect();
            map.rows.push(row);
        }
        map
    }

    fn num_rows(&self) -> usize {
        self.rows.len()
    }

    fn num_cols(&self) -> usize {
        self.rows.first().map_or_else(|| 0, |row| row.len())
    }

    fn column(&self, col_idx: usize) -> Vec<TileType> {
        self.rows
            .iter()
            .map(|row| row.iter().nth(col_idx).unwrap().clone())
            .collect()
    }

    fn get_search_range(&self, direction: &Direction) -> Box<dyn Iterator<Item = usize>> {
        match &direction {
            Direction::North => Box::new(0..self.num_rows()),
            Direction::East => Box::new((0..self.num_cols()).rev()),
            Direction::South => Box::new((0..self.num_rows()).rev()),
            Direction::West => Box::new(0..self.num_cols()),
        }
    }

    fn update_state(vec: &mut Vec<TileType>, empty_idx: &mut Option<usize>, idx: usize) -> bool {
        match vec[idx] {
            TileType::CubeRock => {
                empty_idx.take();
                false
            }
            TileType::Empty => {
                empty_idx.get_or_insert(idx);
                false
            }
            TileType::RoundedRock => {
                if let Some(empty_idx) = empty_idx {
                    vec.swap(idx, *empty_idx);
                    true
                } else {
                    false
                }
            }
        }
    }

    fn get_tilted_map(&self, direction: &Direction) -> Self {
        let mut rows = self.rows.clone();
        if direction == &Direction::North || direction == &Direction::South {
            for col_idx in 0..self.num_cols() {
                let mut col = self.column(col_idx);
                loop {
                    let mut swapped = false;
                    let mut empty_row_idx = None;
                    for row_idx in self.get_search_range(direction) {
                        if Self::update_state(&mut col, &mut empty_row_idx, row_idx) {
                            swapped = true;
                            break;
                        }
                    }
                    if !swapped {
                        break;
                    }
                }
                for (row, tile_type) in rows.iter_mut().zip(col.into_iter()) {
                    row[col_idx] = tile_type;
                }
            }
        } else {
            for row_idx in 0..self.num_rows() {
                let mut row = self.rows[row_idx].clone();
                loop {
                    let mut swapped = false;
                    let mut empty_col_idx = None;
                    for col_idx in self.get_search_range(direction) {
                        if Self::update_state(&mut row, &mut empty_col_idx, col_idx) {
                            swapped = true;
                            break;
                        }
                    }
                    if !swapped {
                        break;
                    }
                }
                rows[row_idx].swap_with_slice(&mut row[..]);
            }
        }

        Self { rows }
    }

    fn get_cycle_tilted(&self) -> Self {
        CYCLE
            .iter()
            .skip(1)
            .fold(self.get_tilted_map(&CYCLE[0]), |map, direction| {
                map.get_tilted_map(direction)
            })
    }

    fn calculate_load(&self, direction: &Direction) -> usize {
        let mut load = 0;
        if direction == &Direction::North {
            for (row_idx, row) in self.rows.iter().enumerate() {
                for tile_type in row {
                    if tile_type == &TileType::RoundedRock {
                        load += self.num_rows() - row_idx;
                    }
                }
            }
        } else {
            todo!("only implemented for North")
        }
        load
    }

    #[allow(unused)]
    fn print(&self) {
        for row in &self.rows {
            for tile_type in row {
                let ch = match tile_type {
                    TileType::Empty => '.',
                    TileType::CubeRock => '#',
                    TileType::RoundedRock => 'O',
                };
                print!("{ch}");
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{read_map, Direction, CYCLE};

    #[test]
    fn test_example_part1() {
        let map = read_map("test");
        let north_tilted_map = map.get_tilted_map(&Direction::North);
        assert_eq!(north_tilted_map.calculate_load(&Direction::North), 136);
    }

    #[test]
    fn test_example_part2() {
        let mut map = read_map("test");
        map = map.get_cycle_tilted();
        assert_eq!(map.calculate_load(&Direction::North), 87);
        map = map.get_cycle_tilted();
        assert_eq!(map.calculate_load(&Direction::North), 69);
        map = map.get_cycle_tilted();
        assert_eq!(map.calculate_load(&Direction::North), 69);
    }
}

fn part1(map: &Map) {
    let north_tilted_map = map.get_tilted_map(&Direction::North);
    let load = north_tilted_map.calculate_load(&Direction::North);
    println!("Part 1: {load}");
}

fn part2(map: &Map) {
    let north_tilted_map = map.get_tilted_map(&Direction::North);
    let load = north_tilted_map.calculate_load(&Direction::North);
    println!("Part 2: {load}");
}
