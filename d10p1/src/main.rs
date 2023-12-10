use std::collections::HashMap;
use std::error::Error;
use std::fs;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Coordinate {
	x: u32,
	y: u32,
}

fn main() -> Result<(), Box<dyn Error>> {
	let (start_tile, pipe_tiles) = {
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
						let mut connects_to = Vec::new();
						connects_to.push(Coordinate { x: x + 1, y });
						connects_to.push(Coordinate { x, y: y + 1 });
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

	let mut pipe_distances: HashMap<Coordinate, u32> = HashMap::new();
	pipe_distances.insert(start_tile, 0);

	let mut current_distance = 0;
	let mut current_tiles = vec![start_tile];

	loop {
		let mut next_tiles = Vec::new();
		for tile in current_tiles.iter() {
			let connected_tiles = pipe_tiles.get(tile).unwrap();
			for connected_tile in connected_tiles.iter() {
				if !pipe_distances.contains_key(connected_tile) {
					next_tiles.push(*connected_tile);
				}
			}
		}

		if next_tiles.is_empty() {
			break;
		}

		current_distance += 1;
		for tile in next_tiles.iter() {
			pipe_distances.insert(*tile, current_distance);
		}
		current_tiles = next_tiles;
	}

	println!("{}", current_distance);

	Ok(())
}
