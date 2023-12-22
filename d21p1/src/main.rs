use std::collections::HashSet;
use std::error::Error;
use std::fs;

const STEP_COUNT: u32 = 64;

#[derive(Clone, Eq, Hash, PartialEq)]
struct Coordinate {
	x: usize,
	y: usize,
}

impl Coordinate {
	fn all_adjacent(&self) -> Vec<Self> {
		let mut adjacent = Vec::new();

		if self.x > 0 {
			let west = Coordinate {
				x: self.x - 1,
				y: self.y,
			};
			adjacent.push(west);
		}

		if self.y > 0 {
			let north = Coordinate {
				x: self.x,
				y: self.y - 1,
			};
			adjacent.push(north);
		}

		let east = Coordinate {
			x: self.x + 1,
			y: self.y,
		};
		adjacent.push(east);

		let south = Coordinate {
			x: self.x,
			y: self.y + 1,
		};
		adjacent.push(south);

		adjacent
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let (start, steppable) = {
		let input = fs::read_to_string("input.txt")?;

		let mut start: Option<Coordinate> = None;
		let mut steppable: HashSet<Coordinate> = HashSet::new();
		for (y, line) in input.lines().enumerate() {
			for (x, c) in line.chars().enumerate() {
				if c == 'S' {
					assert!(start.is_none());
					start = Some(Coordinate { x, y });
					steppable.insert(Coordinate { x, y });
				} else if c == '.' {
					steppable.insert(Coordinate { x, y });
				}
			}
		}

		(start.unwrap(), steppable)
	};

	let mut current_locations: HashSet<Coordinate> = HashSet::new();
	current_locations.insert(start);
	for _ in 0..STEP_COUNT {
		let mut next_locations: HashSet<Coordinate> = HashSet::new();
		for location in current_locations {
			let adjacents = location.all_adjacent();
			for adjacent in adjacents {
				if steppable.contains(&adjacent) {
					next_locations.insert(adjacent);
				}
			}
		}

		current_locations = next_locations;
	}

	println!("{}", current_locations.len());

	Ok(())
}
