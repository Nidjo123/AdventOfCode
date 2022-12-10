use std::fs;

const ADD_OP: u32 = 1;
const MUL_OP: u32 = 2;
const END_OP: u32 = 99;

const WANTED_OUTPUT: u32 = 19690720;

fn apply_op(op: u32, a: u32, b: u32) -> u32 {
    match op {
	ADD_OP => a + b,
	MUL_OP => a * b,
	_ => panic!("unknown op"),
    }
}

fn process(mem: &mut Vec<u32>) {
    let mut pc: usize = 0;
    while mem[pc] != END_OP {
	let opcode = mem[pc];
	let a = mem[mem[pc + 1] as usize];
	let b = mem[mem[pc + 2] as usize];
	let res_loc = mem[pc + 3];
	mem[res_loc as usize] = apply_op(opcode, a, b);
	pc += 4;
    }
}

fn get_output(noun: u32, verb: u32, init_mem: &Vec<u32>) -> u32 {
    let mut mem = init_mem.clone();
    mem[1] = noun;
    mem[2] = verb;
    process(&mut mem);
    mem[0]
}

fn find_inputs_for(out: u32, init_mem: &Vec<u32>) -> (u32, u32) {
    let mut input: (u32, u32) = (0, 0);
    'outer: for i in 0..100 {
	for j in 0..100 {
	    if get_output(i, j, init_mem) == out {
		input = (i, j);
		break 'outer;
	    }
	}
    }
    input
}

fn main() -> std::io::Result<()> {
    let instructions = fs::read_to_string("d02.in")?;
    let mut instructions: Vec<u32> = instructions.trim()
	.split(",")
	.map(|x| x.parse().expect("failed to parse number"))
	.collect();

    let ans1 = get_output(12, 2, &instructions);
    println!("Part 1: {}", ans1);

    let (noun, verb) = find_inputs_for(WANTED_OUTPUT, &instructions);
    let ans2 = 100 * noun + verb;
    println!("Part 2: {}", ans2);
    
    Ok(())
}

