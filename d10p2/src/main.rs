use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Coordinate {
	x: u32,
	y: u32,
}

impl Ord for Coordinate {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.x.cmp(&other.x).then_with(|| self.y.cmp(&other.y))
	}
}

impl PartialOrd for Coordinate {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let (start_tile, mut pipe_tiles) = {
		let input = fs::read_to_string("input.txt")?;

		let mut start_tile: Option<Coordinate> = None;
		let mut pipe_tiles: HashMap<Coordinate, Vec<Coordinate>> = HashMap::new();

		for (y, line) in input.lines().enumerate() {
			for (x, c) in line.chars().enumerate() {
				let x = x as u32;
				let y = y as u32;

				match c {
					'|' => {
						let mut connects_to = Vec::new();
						if y > 0 {
							connects_to.push(Coordinate { x, y: y - 1 });
						}
						connects_to.push(Coordinate { x, y: y + 1 });
						pipe_tiles.insert(Coordinate { x, y }, connects_to);
					}
					'-' => {
						let mut connects_to = Vec::new();
						if x > 0 {
							connects_to.push(Coordinate { x: x - 1, y });
						}
						connects_to.push(Coordinate { x: x + 1, y });
						pipe_tiles.insert(Coordinate { x, y }, connects_to);
					}
					'L' => {
						let mut connects_to = Vec::new();
						if y > 0 {
							connects_to.push(Coordinate { x, y: y - 1 });
						}
						connects_to.push(Coordinate { x: x + 1, y });
						pipe_tiles.insert(Coordinate { x, y }, connects_to);
					}
					'J' => {
						let mut connects_to = Vec::new();
						if y > 0 {
							connects_to.push(Coordinate { x, y: y - 1 });
						}
						if x > 0 {
							connects_to.push(Coordinate { x: x - 1, y });
						}
						pipe_tiles.insert(Coordinate { x, y }, connects_to);
					}
					'7' => {
						let mut connects_to = Vec::new();
						if x > 0 {
							connects_to.push(Coordinate { x: x - 1, y });
						}
						connects_to.push(Coordinate { x, y: y + 1 });
						pipe_tiles.insert(Coordinate { x, y }, connects_to);
					}
					'F' => {
						let connects_to = vec![Coordinate { x: x + 1, y }, Coordinate { x, y: y + 1 }];
						pipe_tiles.insert(Coordinate { x, y }, connects_to);
					}
					'.' => (),
					'S' => start_tile = Some(Coordinate { x, y }),
					_ => panic!("Unexpected character: {}", c),
				}
			}
		}

		let start_tile = start_tile.unwrap();
		let mut start_tile_connects_to: Vec<Coordinate> = Vec::new();
		for (pipe, connections) in pipe_tiles.iter() {
			if connections.contains(&start_tile) {
				start_tile_connects_to.push(*pipe);
			}
		}
		pipe_tiles.insert(start_tile, start_tile_connects_to);

		(start_tile, pipe_tiles)
	};

	let mut main_pipe_tiles: HashSet<Coordinate> = HashSet::new();
	main_pipe_tiles.insert(start_tile);

	let mut current_tiles = vec![start_tile];

	loop {
		let mut next_tiles = Vec::new();
		for tile in current_tiles.iter() {
			let connected_tiles = pipe_tiles.get(tile).unwrap();
			for connected_tile in connected_tiles.iter() {
				if !main_pipe_tiles.contains(connected_tile) {
					next_tiles.push(*connected_tile);
				}
			}
		}

		if next_tiles.is_empty() {
			break;
		}

		for tile in next_tiles.iter() {
			main_pipe_tiles.insert(*tile);
		}
		current_tiles = next_tiles;
	}

	pipe_tiles.retain(|coord, _| main_pipe_tiles.contains(coord));

	let mut max_x = 0;
	let mut max_y = 0;
	for pipe_coord in pipe_tiles.keys() {
		max_x = max_x.max(pipe_coord.x + 1);
		max_y = max_y.max(pipe_coord.y + 1);
	}

	// Start by assuming all are inside, then eliminate those not inside
	let mut inside: HashSet<Coordinate> = HashSet::new();
	for x in 0..=max_x {
		for y in 0..=max_y {
			let coord = Coordinate { x, y };
			if !pipe_tiles.contains_key(&coord) {
				inside.insert(Coordinate { x, y });
			}
		}
	}

	// We treat ourselves as just to the top and left of the current tile for pass-by checks
	let mut outside_check_coords = vec![Coordinate { x: 0, y: 0 }];
	let mut visited: HashSet<Coordinate> = HashSet::new();

	while !outside_check_coords.is_empty() {
		let mut next_coords: Vec<Coordinate> = Vec::new();
		for coord in outside_check_coords.iter() {
			let mut try_next_coords = Vec::new();
			if coord.x > 0 {
				let try_coord = Coordinate {
					x: coord.x - 1,
					y: coord.y,
				};
				if !visited.contains(&try_coord) {
					try_next_coords.push(try_coord);
				}
			}
			if coord.x < max_x {
				let try_coord = Coordinate {
					x: coord.x + 1,
					y: coord.y,
				};
				if !visited.contains(&try_coord) {
					try_next_coords.push(try_coord);
				}
			}
			if coord.y > 0 {
				let try_coord = Coordinate {
					x: coord.x,
					y: coord.y - 1,
				};
				if !visited.contains(&try_coord) {
					try_next_coords.push(try_coord);
				}
			}
			if coord.y < max_y {
				let try_coord = Coordinate {
					x: coord.x,
					y: coord.y + 1,
				};
				if !visited.contains(&try_coord) {
					try_next_coords.push(try_coord);
				}
			}
			for next_coord in try_next_coords.iter() {
				let mut relevant_coords = [*coord, *next_coord];
				relevant_coords.sort_unstable();
				let first_coord = relevant_coords[0];
				let second_coord = relevant_coords[1];

				let mut can_go = true;
				if let Some(pipe_connects) = pipe_tiles.get(&first_coord) {
					// We just want to check for blockage
					// For example, if going from (1, 1) to (1, 2), we need to ensure there's no pipe connecting (0, 1)
					// and (1, 1)
					// Or if going from (1, 1), to (2, 1), we need to ensure there's no pipe connecting (1, 1) and
					// (1, 0)
					if second_coord.x > 0 && second_coord.y > 0 {
						let bad_connect_coord = Coordinate {
							x: second_coord.x - 1,
							y: second_coord.y - 1,
						};
						if pipe_connects.contains(&bad_connect_coord) {
							can_go = false;
						}
					}
				}
				if can_go {
					next_coords.push(*next_coord);
					visited.insert(*next_coord);
					inside.remove(next_coord);
				}
			}
		}

		outside_check_coords = next_coords;
	}

	println!("{}", inside.len());

	Ok(())
}
