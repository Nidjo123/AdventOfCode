use std::error::Error;

const RADIX: u32 = 10;
const WIDTH: usize = 25;
const HEIGHT: usize = 6;
const LAYER_SIZE: usize = WIDTH * HEIGHT;

fn get_counts(layer: &[u32]) -> (u32, u32, u32) {
    let mut counts = [0, 0, 0];
    for d in 0..=2 {
	counts[d] = layer.iter()
	    .filter(|x| **x == d as u32)
	    .count() as u32;
    }
    assert_eq!(counts.iter().sum::<u32>() , LAYER_SIZE as u32);
    (counts[0], counts[1], counts[2])
}

fn part1(digits: &[u32]) -> u32 {
    let mut layer_counts = Vec::new();
    for layer in digits.chunks(LAYER_SIZE) {
	let counts = get_counts(layer);
	layer_counts.push(counts);
    }
    let (_, cnt1, cnt2) = layer_counts.iter()
	.min_by_key(|v| v.0)
	.unwrap();
    cnt1 * cnt2
}

fn part2(digits: &[u32]) -> [u32; LAYER_SIZE] {
    let mut image = [2u32; LAYER_SIZE];
    for layer in digits.chunks(LAYER_SIZE) {
	let transformed_layer: Vec<u32> = layer.iter()
	    .zip(&image[..])
	    .map(|(new, old)| if *old >= 2 {
		*new
	    } else {
		*old
	    })
	    .collect();
	for (i, v) in transformed_layer.iter().enumerate() {
	    image[i] = *v;
	}
    }
    image
}

fn print_image(image: &[u32; LAYER_SIZE]) {
    for y in 0..HEIGHT {
	for x in 0..WIDTH {
	    print!("{}", image[y * WIDTH + x]);
	}
	println!();
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let line = std::fs::read_to_string("d08.in")?;
    let digits: Vec<u32> = line.trim()
	.chars()
	.map(|c| c.to_digit(RADIX).unwrap())
	.collect();

    let ans1 = part1(&digits[..]);
    println!("Part 1: {}", ans1);

    let image = part2(&digits[..]);
    print_image(&image);
    
    Ok(())
}
