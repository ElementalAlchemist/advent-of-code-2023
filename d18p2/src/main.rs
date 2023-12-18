use std::error::Error;
use std::fs;

#[derive(Clone, Eq, Hash, PartialEq)]
struct Coordinate {
	x: i64,
	y: i64,
}

#[derive(Clone, Copy)]
enum Direction {
	North,
	East,
	South,
	West,
}

#[derive(Clone)]
struct Trench {
	direction: Direction,
	length: i64,
}

fn main() -> Result<(), Box<dyn Error>> {
	let trenches = {
		let input = fs::read_to_string("input.txt")?;

		let mut trenches: Vec<Trench> = Vec::new();
		for line in input.lines() {
			let mut line_parts = line.split(' ');
			assert!(line_parts.next().is_some());
			assert!(line_parts.next().is_some());
			let color = line_parts.next().unwrap();
			let Some(color) = color.strip_prefix("(#") else {
				panic!("Missing parenthesis")
			};
			let Some(color) = color.strip_suffix(')') else {
				panic!("Missing parenthesis")
			};
			let mut color_chars = color.chars();
			let mut length: Vec<char> = Vec::new();
			for _ in 0..5 {
				length.push(color_chars.next().unwrap());
			}
			let length: String = length.iter().collect();
			let length = i64::from_str_radix(&length, 16)?;
			let direction = match color_chars.next().unwrap() {
				'0' => Direction::East,
				'1' => Direction::South,
				'2' => Direction::West,
				'3' => Direction::North,
				_ => panic!("Unexpected direction"),
			};
			assert!(color_chars.next().is_none());

			trenches.push(Trench { direction, length });
		}

		trenches
	};

	let mut current_coordinate = Coordinate { x: 0, y: 0 };
	let mut dig_corners = vec![current_coordinate.clone()];

	let mut outer_trench = 0;
	for trench in trenches.iter() {
		match trench.direction {
			Direction::North => current_coordinate.y -= trench.length,
			Direction::East => current_coordinate.x += trench.length,
			Direction::South => current_coordinate.y += trench.length,
			Direction::West => current_coordinate.x -= trench.length,
		}
		dig_corners.push(current_coordinate.clone());
		outer_trench += trench.length;
	}

	let mut total = 0;
	let mut corner_iter = dig_corners.iter();
	let first_corner = corner_iter.next().unwrap();
	let mut previous_corner = first_corner;
	for corner in corner_iter {
		total += (previous_corner.x * corner.y) - (previous_corner.y * corner.x);
		previous_corner = corner;
	}
	total += (previous_corner.x * first_corner.y) - (previous_corner.y * first_corner.x);

	let dig_area = (total.abs() + outer_trench) / 2 + 1;
	println!("{}", dig_area);

	Ok(())
}
// 365707789713841058158275
// 952408144115
