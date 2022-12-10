use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::{HashMap, HashSet};

fn count(depth: u32, current: String, map: &HashMap<String, HashSet<String>>) -> u32 {
    depth + match map.get(&current) {
	Some(bodies) => {
	    bodies.iter()
		.map(|x| count(depth + 1, x.to_string(), map))
		.sum()	
	},
	None => 0,
    }
}

fn count_orbits(map: &HashMap<String, HashSet<String>>) -> u32 {
    let start = String::from("COM");
    count(0, start, map)
}

fn path_to(wanted: &String, path: &mut Vec<String>,
	    map: &HashMap<String, HashSet<String>>) -> Option<Vec<String>> {
    let current = path.last().unwrap();
    match map.get(current) {
	Some(bodies) => {
	    for body in bodies {
		let name = body.to_string();
		if body == wanted {
		    path.push(name);
		    return Some(path.clone())
		} else {
		    path.push(name);
		    let res = path_to(wanted, path, map);
		    if res.is_some() {
			return res
		    }
		    path.pop();
		}
	    }
	    None
	},
	None => None,
    }
}

fn get_path(name: &String, map: &HashMap<String, HashSet<String>>) -> Option<Vec<String>> {
    let start = String::from("COM");
    let mut path = vec![start];
    path_to(name, &mut path, map)
}

fn main() -> std::io::Result<()> {
    let file = File::open("d06.in")?;
    let reader = BufReader::new(file);

    let mut map = HashMap::new();
    
    for line in reader.lines() {
	let line = line.unwrap();
	let bodies: Vec<&str> = line.split(")").collect();
	let a = bodies[0].to_string();
	let b = bodies[1].to_string();
	let orbits = map.entry(a)
	    .or_insert(HashSet::new());
	orbits.insert(b);
    }

    let res1 = count_orbits(&map);
    println!("Part 1: {}", res1);

    let you = String::from("YOU");
    let san = String::from("SAN");
    let path_you = get_path(&you, &map).unwrap();
    let path_san = get_path(&san, &map).unwrap();

    let both = path_you.iter()
	.zip(&path_san)
	.take_while(|(x, y)| x == y)
	.count();

    let deg_you = path_you.len();
    let deg_san = path_san.len();
    let res2 = deg_you + deg_san - 2 * both - 2;

    println!("Part 2: {}", res2);

    Ok(())
}
