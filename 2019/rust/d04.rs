use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashSet;

type Point = (i32, i32);

fn manhattan_distance(p: &Point) -> u32 {
    p.0.abs() as u32 + p.1.abs() as u32
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn from_char(c: char) -> Direction {
	match c {
	    'U' => Direction::Up,
	    'R' => Direction::Right,
	    'D' => Direction::Down,
	    'L' => Direction::Left,
	    _ => panic!("unknown direction"),
	}
    }
}

struct Instr {
    dir: Direction,
    amt: i32,
}

impl Instr {
    fn from_string(s: String) -> Instr {
	let bytes = s.as_bytes();
	let dir = Direction::from_char(bytes[0] as char);
	let amt: i32 = String::from_utf8_lossy(&bytes[1..])
	    .parse()
	    .expect("invalid amount");
	Instr {dir, amt}
    }
}

struct Wire {
    instrs: Vec<Instr>,
}

fn next_point(p: &Point, dir: &Direction) -> Point {
    let (x, y) = *p;
    match dir {
	Direction::Left => (x - 1, y),
	Direction::Right => (x + 1, y),
	Direction::Up => (x, y + 1),
	Direction::Down => (x, y - 1),
    }
}

impl Wire {
    fn from_string(s: String) -> Wire {
	let mut instrs: Vec<Instr> = vec![];
	for instr in s.split(",") {
	    let instr = Instr::from_string(instr.to_string());
	    instrs.push(instr);
	}
	Wire {instrs}
    }

    fn get_points(&self) -> HashSet<Point> {
	let mut points = HashSet::new();
	let mut curr_point = (0, 0);
	for instr in &self.instrs {
	    for _ in 0..instr.amt {
		curr_point = next_point(&curr_point, &instr.dir);
		points.insert(curr_point);
	    }
	}
	points
    }

    fn steps_to_intersection(&self, p: &Point) -> u32 {
	let mut curr_point = (0, 0);
	let mut steps = 0;
	'outer: for instr in &self.instrs {
	    for _ in 0..instr.amt {
		curr_point = next_point(&curr_point, &instr.dir);
		steps += 1;
		if curr_point == *p {
		    break 'outer;
		}
	    }
	}
	steps
    }
}

fn find_closest_intersection(w: &Wire, v: &Wire) -> Option<u32> {
    let w_pts = w.get_points();
    let v_pts = v.get_points();
    let intersections = w_pts.intersection(&v_pts);
    intersections.map(|p| manhattan_distance(p))
	.min()
}

fn find_fewest_steps_to_intersection(w: &Wire, v: &Wire) -> Option<u32> {
    let w_pts = w.get_points();
    let v_pts = v.get_points();
    let intersections = w_pts.intersection(&v_pts);
    intersections.map(|p| w.steps_to_intersection(p) + v.steps_to_intersection(p))
	.min()
}

fn main() -> std::io::Result<()> {
    let file = File::open("d03.in")?;
    let buf_reader = BufReader::new(file);

    let mut wires: Vec<Wire> = vec![];
    for line in buf_reader.lines() {
	let wire = Wire::from_string(line.unwrap());
	wires.push(wire);
    }

    let closest_intersection_distance = find_closest_intersection(&wires[0], &wires[1]).unwrap();
    
    println!("Part 1: {}", closest_intersection_distance);

    let fewest_steps = find_fewest_steps_to_intersection(&wires[0], &wires[1]).unwrap();
    println!("Part 2: {}", fewest_steps);
    
    Ok(())
}

