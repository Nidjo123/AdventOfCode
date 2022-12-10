use std::fs;
use std::collections::{VecDeque, HashMap};

const MEM_SIZE: usize = 100000;

const ADD_OP: u32 = 1;
const MUL_OP: u32 = 2;
const IN_OP: u32 = 3;
const OUT_OP: u32 = 4;
const JIFT_OP: u32 = 5;
const JIFF_OP: u32 = 6;
const LT_OP: u32 = 7;
const EQ_OP: u32 = 8;
const ARB_OP: u32 = 9;
const HALT_OP: u32 = 99;

#[derive(PartialEq,Debug)]
enum Op {
    Add,
    Mul,
    Input,
    Output,
    JmpIfTrue,
    JmpIfFalse,
    LessThan,
    Equals,
    AdjustRelBase,
    Halt,
}

impl Op {
    fn from_value(op: u32) -> Op {
	match op % 100 {
	    ADD_OP => Op::Add,
	    MUL_OP => Op::Mul,
	    IN_OP => Op::Input,
	    OUT_OP => Op::Output,
	    JIFT_OP => Op::JmpIfTrue,
	    JIFF_OP => Op::JmpIfFalse,
	    LT_OP => Op::LessThan,
	    EQ_OP => Op::Equals,
            ARB_OP => Op::AdjustRelBase,
	    HALT_OP => Op::Halt,
	    _ => panic!("unknown opcode"),
	}
    }
}

enum ParamMode {
    Position,
    Immediate,
    Relative,
}

impl ParamMode {
    fn from_value(mode: u32) -> ParamMode {
	match mode {
	    0 => ParamMode::Position,
	    1 => ParamMode::Immediate,
            2 => ParamMode::Relative,
	    _ => panic!("unknown parameter mode"),
	}
    }
}

struct CPU {
    pc: usize,
    base_offset: i32,
    mem: Vec<i128>,
    outputs: Vec<i128>,
    inputs: VecDeque<i128>,
}

impl CPU {
    fn unpack_instr(&self) -> ([ParamMode; 3], Op) {
	let val: u32 = self.mem[self.pc] as u32;
	let op = Op::from_value(val % 100);
	let mode_a = ParamMode::from_value(val / 100 % 10);
	let mode_b = ParamMode::from_value(val / 1000 % 10);
	let mode_c = ParamMode::from_value(val / 10000);
	([mode_a, mode_b, mode_c], op)
    }

    fn get_value(&self, val: i128, mode: &ParamMode) -> i128 {
	match mode {
	    ParamMode::Position => self.mem[val as usize],
	    ParamMode::Immediate => val,
            ParamMode::Relative => {
                let location = (self.base_offset + val as i32) as usize;
                self.mem[location]
            }
	}
    }

    fn get_values(&self, modes: &[ParamMode; 3]) -> (i128, i128) {
	let a = self.get_value(self.mem[self.pc + 1], &modes[0]);
	let b = self.get_value(self.mem[self.pc + 2], &modes[1]);
	(a, b)
    }

    fn set_value(&mut self, val: i128, pos: i128,  mode: &ParamMode) {
        match mode {
	    ParamMode::Position => self.mem[pos as usize] = val,
	    ParamMode::Immediate => (),
            ParamMode::Relative => {
                let location = (self.base_offset + pos as i32) as usize;
                self.mem[location] = val;
            }
	}
    }

    fn add_input(&mut self, x: i128) {
	self.inputs.push_back(x);
    }

    fn set_mem_size(&mut self, mem_size: usize) {
        assert!(self.mem.len() <= mem_size);
        self.mem.resize(mem_size, 0);
    }

    fn step(&mut self) -> bool {
	let op = Op::from_value(self.mem[self.pc] as u32);
        //println!("pc: {} [{:?} ({})]", self.pc, op, self.mem[self.pc]);
	match op {
	    Op::Add => {
		let (modes, _) = self.unpack_instr();
		let (a, b) = self.get_values(&modes);
                self.set_value(a + b, self.mem[self.pc + 3], &modes[2]);
		self.pc += 4;
		false
	    },
	    Op::Mul => {
		let (modes, _) = self.unpack_instr();
		let (a, b) = self.get_values(&modes);
                self.set_value(a * b, self.mem[self.pc + 3], &modes[2]);
		self.pc += 4;
		false
	    },
	    Op::Input => {
		if let Some(x) = self.inputs.pop_front() {
                    let (modes, _) = self.unpack_instr();
                    self.set_value(x, self.mem[self.pc + 1], &modes[0]);
		    self.pc += 2;
		    false
		} else {
		    true
		}
	    },
	    Op::Output => {
                let (modes, _) = self.unpack_instr();
		let a = self.get_value(self.mem[self.pc + 1], &modes[0]);
		self.outputs.push(a);
		self.pc += 2;
		false
	    },
	    Op::JmpIfTrue => {
		let (modes, _) = self.unpack_instr();
		let a = self.get_value(self.mem[self.pc + 1], &modes[0]);
		let b = self.get_value(self.mem[self.pc + 2], &modes[1]);
		if a != 0 {
		    self.pc = b as usize;
		} else {
		    self.pc += 3;
		}
		false
	    },
	    Op::JmpIfFalse => {
		let (modes, _) = self.unpack_instr();
		let a = self.get_value(self.mem[self.pc + 1], &modes[0]);
		let b = self.get_value(self.mem[self.pc + 2], &modes[1]);
		if a == 0 {
		    self.pc = b as usize;
		} else {
		    self.pc += 3;
		}
		false
	    },
	    Op::LessThan => {
		let (modes, _) = self.unpack_instr();
		let (a, b) = self.get_values(&modes);
		let res = if a < b {
                    1
		} else {
		    0
		};
                self.set_value(res, self.mem[self.pc + 3], &modes[2]);
		self.pc += 4;
		false
	    },
	    Op::Equals => {
		let (modes, _) = self.unpack_instr();
		let (a, b) = self.get_values(&modes);
		let res = if a == b {
                    1
		} else {
		    0
		};
                self.set_value(res, self.mem[self.pc + 3], &modes[2]);
		self.pc += 4;
		false
	    },
            Op::AdjustRelBase => {
		let (modes, _) = self.unpack_instr();
                let a = self.get_value(self.mem[self.pc + 1], &modes[0]);
                self.base_offset += a as i32;
		self.pc += 2;
		false
            }
	    _ => true
	}
    }

    fn run(&mut self){
	while !self.step() {
	    continue
	}
    }
}

type Location = (i32, i32);

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn step(&self, pos: Location) -> Location {
	let (x, y) = pos;
	match self {
	    Direction::Up => (x, y + 1),
	    Direction::Right => (x + 1, y),
	    Direction::Down => (x, y - 1),
	    Direction::Left => (x - 1, y),
	}
    }

    fn turn(&self, dir: u8) -> Direction {
	match dir {
	    0 => match self {
		Direction::Up => Direction::Left,
		Direction::Right => Direction::Up,
		Direction::Down => Direction::Right,
		Direction::Left => Direction::Down,
	    },
	    1 => match self {
		Direction::Up => Direction::Right,
		Direction::Right => Direction::Down,
		Direction::Down => Direction::Left,
		Direction::Left => Direction::Up,
	    },
	    _ => panic!("invalid dir"),
	}
    }
}

fn part1(instrs: &[i128]) -> usize {
    let mut cpu = CPU {pc: 0,
                       base_offset: 0,
		       mem: instrs.to_vec(),
		       outputs: Vec::new(),
		       inputs: VecDeque::new()};
    cpu.set_mem_size(MEM_SIZE);

    let mut pos = (0, 0);
    let mut dir = Direction::Up;
    let mut visited: HashMap<Location, i128> = HashMap::new();

    loop {
	let color = visited.entry(pos).or_insert(0);
	cpu.add_input(*color);
	cpu.run();
	let (_, op) = cpu.unpack_instr();
	if op == Op::Halt {
	    break visited.len()
	}
	let len = cpu.outputs.len();
	if let Some(&new_color) = cpu.outputs.get(len - 2) {
	    *color = new_color;
	}
	if let Some(&new_direction) = cpu.outputs.get(len - 1) {
	    dir = dir.turn(new_direction as u8);
	}
	pos = dir.step(pos);
    }
}

fn part2(instrs: &[i128]) {
    let mut cpu = CPU {pc: 0,
                       base_offset: 0,
		       mem: instrs.to_vec(),
		       outputs: Vec::new(),
		       inputs: VecDeque::new()};
    cpu.set_mem_size(MEM_SIZE);

    let mut pos = (0, 0);
    let mut dir = Direction::Up;
    let mut visited: HashMap<Location, i128> = HashMap::new();

    visited.insert(pos, 1);
    loop {
	let color = visited.entry(pos).or_insert(0);
	cpu.add_input(*color);
	cpu.run();
	let (_, op) = cpu.unpack_instr();
	if op == Op::Halt {
	    break;
	}
	let len = cpu.outputs.len();
	if let Some(&new_color) = cpu.outputs.get(len - 2) {
	    *color = new_color;
	}
	if let Some(&new_direction) = cpu.outputs.get(len - 1) {
	    dir = dir.turn(new_direction as u8);
	}
	pos = dir.step(pos);
    }

    let max_pos = visited.iter()
	.fold((0, 0), |acc, ((x, y), _v)| (acc.0.max(*x), acc.1.max(*y)));
    let min_pos = visited.iter()
	.fold((0, 0), |acc, ((x, y), _v)| (acc.0.min(*x), acc.1.min(*y)));
    let (dim_x, dim_y) = (max_pos.0 - min_pos.0, max_pos.1 - min_pos.1);

    for y in 0..=dim_y {
	for x in 0..dim_x {
	    let col = visited.get(&(x + min_pos.0, max_pos.1 - y)).unwrap_or(&0);
	    if *col == 1i128 {
		print!("{}", col);
	    } else {
		print!(" ");
	    }
	}
	println!();
    }
}

fn main() -> std::io::Result<()> {
    let instructions = fs::read_to_string("d11.in")?;
    let instructions: Vec<i128> = instructions.trim()
	.split(",")
	.map(|x| x.parse().expect("failed to parse number"))
	.collect();

    let ans1 = part1(&instructions);
    println!("Part 1: {}", ans1);

    println!("Part 2:");
    part2(&instructions);

    Ok(())
}
