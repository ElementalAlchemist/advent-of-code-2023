use std::collections::HashSet;
use std::error::Error;
use std::fs;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Coordinate {
	x: u32,
	y: u32,
}

fn main() -> Result<(), Box<dyn Error>> {
	let galaxies = {
		let input = fs::read_to_string("input.txt")?;

		let mut galaxy_coordinates: Vec<Coordinate> = Vec::new();
		for (y, line) in input.lines().enumerate() {
			for (x, c) in line.chars().enumerate() {
				if c == '#' {
					galaxy_coordinates.push(Coordinate { x: x as u32, y: y as u32 });
				}
			}
		}

		let mut max_x = 0;
		let mut max_y = 0;
		for coord in galaxy_coordinates.iter() {
			max_x = max_x.max(coord.x);
			max_y = max_y.max(coord.y);
		}
		let mut empty_rows: HashSet<u32> = (0..=max_y).collect();
		let mut empty_columns: HashSet<u32> = (0..=max_x).collect();
		for coord in galaxy_coordinates.iter() {
			empty_rows.remove(&coord.y);
			empty_columns.remove(&coord.x);
		}

		for coord in galaxy_coordinates.iter_mut() {
			let add_x = empty_columns.iter().filter(|col| **col < coord.x).count() as u32;
			let add_y = empty_rows.iter().filter(|col| **col < coord.y).count() as u32;

			coord.x += add_x;
			coord.y += add_y;
		}

		galaxy_coordinates
	};

	let mut distance = 0;
	for (first_index, first) in galaxies.iter().enumerate() {
		for second in galaxies.iter().skip(first_index + 1) {
			distance += first.x.abs_diff(second.x) + first.y.abs_diff(second.y);
		}
	}

	println!("{}", distance);

	Ok(())
}
