use std::fs;

fn main() {
    let map = read_map("input");
    part1(&map);
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

    fn get_tilted_map(&self, direction: &Direction) -> Self {
        let mut rows = self.rows.clone();
        if direction == &Direction::North || direction == &Direction::South {
            for col_idx in 0..self.num_cols() {
                let mut col = self.column(col_idx);
                loop {
                    let mut swapped = false;
                    let mut empty_row_idx = None;
                    for row_idx in 0..self.num_rows() {
                        match col[row_idx] {
                            TileType::CubeRock => {
                                empty_row_idx.take();
                            }
                            TileType::Empty => {
                                empty_row_idx.get_or_insert(row_idx);
                            }
                            TileType::RoundedRock => {
                                if let Some(empty_row_idx) = empty_row_idx {
                                    col.swap(row_idx, empty_row_idx);
                                    swapped = true;
                                    break;
                                }
                            }
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
        }

        Self { rows }
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

fn part1(map: &Map) {
    let north_tilted_map = map.get_tilted_map(&Direction::North);
    let load = north_tilted_map.calculate_load(&Direction::North);
    println!("Part 1: {load}");
}
