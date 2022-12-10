use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;


fn calculate_fuel(mass: i32) -> i32 {
    return mass / 3 - 2;
}

fn main() -> std::io::Result<()> {
    let file = File::open("d01.in")?;
    let buf_reader = BufReader::new(file);

    let mut fuel = 0;
    let mut total_fuel = 0;
    for line in buf_reader.lines() {
	match line {
	    Ok(num) => {
		let mass: i32 = num.trim().parse()
		    .expect("Failed to parse number");
		let mut curr_fuel = calculate_fuel(mass);
		fuel += curr_fuel;
		while curr_fuel > 0 {
		    total_fuel += curr_fuel;
		    curr_fuel = calculate_fuel(curr_fuel);
		}
	    },
	    Err(_) => println!("Failed to read line"),
	}
    }

    println!("Part 1: {}", fuel);
    println!("Part 2: {}", total_fuel);
    
    Ok(())
}
