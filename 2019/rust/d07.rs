use std::fs;
use std::collections::{VecDeque, HashSet};

const ADD_OP: u32 = 1;
const MUL_OP: u32 = 2;
const IN_OP: u32 = 3;
const OUT_OP: u32 = 4;
const JIFT_OP: u32 = 5;
const JIFF_OP: u32 = 6;
const LT_OP: u32 = 7;
const EQ_OP: u32 = 8;
const HALT_OP: u32 = 99;

#[derive(PartialEq)]
enum Op {
    Add,
    Mul,
    Input,
    Output,
    JmpIfTrue,
    JmpIfFalse,
    LessThan,
    Equals,
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
	    HALT_OP => Op::Halt,
	    _ => panic!("unknown opcode"),
	}
    }
}

enum ParamMode {
    Position,
    Immediate,
}

impl ParamMode {
    fn from_value(mode: u32) -> ParamMode {
	match mode {
	    0 => ParamMode::Position,
	    1 => ParamMode::Immediate,
	    _ => panic!("unknown parameter mode"),
	}
    }
}

struct CPU {
    pc: usize,
    mem: Vec<i32>,
    outputs: Vec<i32>,
    inputs: VecDeque<i32>,
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

    fn get_value(&self, val: i32, mode: &ParamMode) -> i32 {
	match mode {
	    ParamMode::Position => self.mem[val as usize],
	    ParamMode::Immediate => val,
	}
    }

    fn get_values(&self, modes: [ParamMode; 3]) -> (i32, i32, usize) {
	let a = self.get_value(self.mem[self.pc + 1], &modes[0]);
	let b = self.get_value(self.mem[self.pc + 2], &modes[1]);
	let c = self.mem[self.pc + 3];
	(a, b, c as usize)
    }

    fn add_input(&mut self, x: i32) {
	self.inputs.push_back(x);
    }

    fn transfer_outputs(&mut self, inputs: &[i32]) {
	for x in inputs.iter().rev() {
	    self.add_input(*x);
	}
    }
    
    fn step(&mut self) -> bool {
	let op = Op::from_value(self.mem[self.pc] as u32);
	match op {
	    Op::Add => {
		let (modes, _) = self.unpack_instr();
		let (a, b, c) = self.get_values(modes);
		self.mem[c] = a + b;
		self.pc += 4;
		false
	    },
	    Op::Mul => {
		let (modes, _) = self.unpack_instr();
		let (a, b, c) = self.get_values(modes);
		self.mem[c] = a * b;
		self.pc += 4;
		false
	    },
	    Op::Input => {
		let pos = self.mem[self.pc + 1] as usize;
		if let Some(x) = self.inputs.pop_front() {
		    self.mem[pos] = x;
		    self.pc += 2;
		    false
		} else {
		    true
		}
	    },
	    Op::Output => {
		let pos = self.mem[self.pc + 1] as usize;
		self.outputs.push(self.mem[pos]);
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
		let (a, b, c) = self.get_values(modes);
		if a < b {
		    self.mem[c] = 1;
		} else {
		    self.mem[c] = 0;
		}
		self.pc += 4;
		false
	    }
	    Op::Equals => {
		let (modes, _) = self.unpack_instr();
		let (a, b, c) = self.get_values(modes);
		if a == b {
		    self.mem[c] = 1;
		} else {
		    self.mem[c] = 0;
		}
		self.pc += 4;
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

// see Knuth 7.2.1.2. (Algorithm L)
fn next_permutation(elems: &mut [i32]) {
    let len = elems.len();
    let mut j = elems.len() - 2;
    while j > 0 && elems[j] >= elems[j + 1] {
	j -= 1;
    }
    if elems[j] < elems[j + 1] {
	let mut l = len - 1;
	while elems[j] >= elems[l] {
	    l -= 1;
	}
	elems.swap(j, l);
	elems[(j+1)..len].reverse();
    } else {
	// done, go again
	elems[..].reverse();
    }

}

fn part1(instrs: &[i32]) -> i32 {
    let mut phases: Vec<i32> = (0..=4).collect();
    let n_phases = phases.len();
    let n_perms: i32 = (1..=n_phases as i32).product();
    let mut max_output = 0;
    for _i in 0..n_perms {
	let mut output = 0;
	for amp in &phases {
	    let mut cpu = CPU {pc: 0,
			       mem: instrs.to_vec(),
			       outputs: vec![],
			       inputs: VecDeque::new()};
	    cpu.add_input(*amp);
	    cpu.add_input(output);
	    cpu.run();
	    output = *cpu.outputs.last().unwrap();
	    max_output = std::cmp::max(max_output, output);
	}
	
	next_permutation(&mut phases);
    }
    max_output
}

fn part2(instrs: &[i32]) -> i32 {
    let mut phases: Vec<i32> = (5..=9).collect();
    let n_phases = phases.len();
    let n_perms = (1..=n_phases as i32).product();
    let mut max_output = 0;
    for i in 0..n_perms {
	let mut amps = [CPU {pc: 0, mem: instrs.to_vec(), outputs: vec![], inputs: VecDeque::new()},
			CPU {pc: 0, mem: instrs.to_vec(), outputs: vec![], inputs: VecDeque::new()},
			CPU {pc: 0, mem: instrs.to_vec(), outputs: vec![], inputs: VecDeque::new()},
			CPU {pc: 0, mem: instrs.to_vec(), outputs: vec![], inputs: VecDeque::new()},
			CPU {pc: 0, mem: instrs.to_vec(), outputs: vec![], inputs: VecDeque::new()}];
	let n_amps = amps.len();

	for (j, phase) in phases.iter().enumerate() {
	    amps[j].add_input(*phase);
	}

	let mut outputs = vec![0];
	let mut last_out = 0;
	loop {
	    let mut curr_amp = 0;
	    let mut done_cnt = 0;
	    for amp in &mut amps {
		curr_amp += 1;
		let (_, op) = amp.unpack_instr();
		if op == Op::Halt {
		    done_cnt += 1;
		    continue;
		}
		amp.transfer_outputs(&outputs[..]);
		amp.run();
		outputs = amp.outputs.clone();
		amp.outputs.clear();
		if curr_amp == 5 {
		    if let Some(out) = outputs.last() {
			last_out = *out;
		    }
		}
	    }
	    if done_cnt == n_amps {
		break;
	    }
	}
	max_output = std::cmp::max(max_output, last_out);

	next_permutation(&mut phases);
    }
    max_output
}

fn main() -> std::io::Result<()> {
    let instructions = fs::read_to_string("d07.in")?;
    let instructions: Vec<i32> = instructions.trim()
	.split(",")
	.map(|x| x.parse().expect("failed to parse number"))
	.collect();
    
    let ans1 = part1(&instructions);
    println!("Part 1: {}", ans1);

    let ans2 = part2(&instructions);
    println!("Part 2: {}", ans2);
    
    Ok(())
}


