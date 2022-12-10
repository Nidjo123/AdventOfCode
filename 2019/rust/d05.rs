use std::fs;

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
    input: i32,
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
		self.mem[pos] = self.input;
		self.pc += 2;
		false
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

fn main() -> std::io::Result<()> {
    let instructions = fs::read_to_string("d05.in")?;
    let instructions: Vec<i32> = instructions.trim()
	.split(",")
	.map(|x| x.parse().expect("failed to parse number"))
	.collect();

    let mut cpu = CPU {pc: 0, mem: instructions.clone(), outputs: vec![], input: 1};
    cpu.run();
    println!("Part 1: {}", cpu.outputs.last().unwrap());

    let mut cpu = CPU {pc: 0, mem: instructions.clone(), outputs: vec![], input: 5};
    cpu.run();
    println!("Part 1: {}", cpu.outputs.last().unwrap());
    
    Ok(())
}

