use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Clone, Eq)]
struct Asteroid {
    x: i32,
    y: i32,
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

type Vec2 = (f64, f64);

fn length(v: Vec2) -> f64 {
    (v.0 * v.0 + v.1 * v.1).sqrt()
}

fn normalize(v: Vec2) -> Vec2 {
    let len = length(v);
    (v.0 / len, v.1 / len)
}

// https://math.stackexchange.com/questions/878785/how-to-find-an-angle-in-range0-360-between-2-vectors
fn angle(v: Vec2) -> f64 {
    let (a, b) = normalize(v);
    let (c, d) = (0f64, 1f64);

    let dot = a * b + b * d;
    let det = a * d - b * c;

    det.atan2(dot) + std::f64::consts::PI
}

impl Asteroid {
    fn can_detect(&self, other: &Asteroid, asteroids: &[Asteroid]) -> bool {
        let dy = other.y - self.y;
        let dx = other.x - self.x;
        let g = gcd(dx.abs(), dy.abs());
        let dy = dy / g;
        let dx = dx / g;

        let mut x = self.x + dx;
        let mut y = self.y + dy;
        while x != other.x || y != other.y {
            for a in asteroids {
                if a.x == x && a.y == y {
                    return false
                }
            }
            x += dx;
            y += dy;
        }
        true
    }

    fn count_detections(&self, asteroids: &[Asteroid]) -> usize {
        asteroids.iter()
            .filter(|a| a != &self && self.can_detect(a, asteroids))
            .count()
    }
    
    fn destroy_all(&self, asteroids: &[Asteroid]) -> Vec<(i32, i32)> {
        let mut destroyed_asteroids: Vec<(i32, i32)> = Vec::new();
	let mut asteroids: Vec<Asteroid> = asteroids.iter().cloned().collect();

        while asteroids.len() != 1 {
            let mut batch: Vec<Asteroid> = asteroids.iter()
		.cloned()
                .filter(|a| a != self && self.can_detect(a, &asteroids))
                .collect();

	    batch.sort_by(|a, b| {
		let ang_a = angle(((a.x - self.x) as f64, (a.y - self.y) as f64));
		let ang_b = angle(((b.x - self.x) as f64, (b.y - self.y) as f64));
		if ang_a < ang_b {
		    Ordering::Greater
		} else if ang_a > ang_b {
		    Ordering::Less
		} else {
		    Ordering::Equal
		}
	    });
	    
	    let locations: Vec<(i32, i32)> = batch.iter()
		.map(|a| (a.x, a.y))
		.collect();
	    destroyed_asteroids.extend(&locations);

	    asteroids.retain(|a| !batch.contains(&a));
        }
        destroyed_asteroids
    }
}

fn find_best_asteroid(asteroids: &[Asteroid]) -> &Asteroid {
    asteroids.iter()
        .max_by_key(|a| a.count_detections(asteroids) as i32)
        .unwrap()
}

fn main() -> std::io::Result<()> {
    let file = File::open("d10.in")?;
    let buf_reader = BufReader::new(file);

    let mut asteroids = Vec::new();

    for (y, line) in buf_reader.lines().enumerate() {
        let line = line?;
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                let asteroid = Asteroid {x: x as i32, y: y as i32};
                asteroids.push(asteroid);
            }
        }
    }

    let asteroid = find_best_asteroid(&asteroids);
    let detections = asteroid.count_detections(&asteroids);
    println!("Part 1: {}", detections);
    
    let destroyed_asteroids = asteroid.destroy_all(&asteroids);
    let (x, y) = destroyed_asteroids[199];
    let ans2 = x * 100 + y;
    println!("Part 2: {}", ans2);

    Ok(())
}
