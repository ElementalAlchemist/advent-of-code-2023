use std::collections::HashSet;
use std::error::Error;
use std::fs;

#[derive(Clone, Eq, Hash, PartialEq)]
struct Coordinate {
	x: i32,
	y: i32,
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
	length: u32,
	color: String,
}

fn main() -> Result<(), Box<dyn Error>> {
	let trenches = {
		let input = fs::read_to_string("input.txt")?;

		let mut trenches: Vec<Trench> = Vec::new();
		for line in input.lines() {
			let mut line_parts = line.split(' ');
			let direction = match line_parts.next().unwrap() {
				"U" => Direction::North,
				"R" => Direction::East,
				"D" => Direction::South,
				"L" => Direction::West,
				_ => panic!("Unexpected direction"),
			};
			let length = line_parts.next().unwrap().parse()?;
			let color = line_parts.next().unwrap();
			let Some(color) = color.strip_prefix('(') else {
				panic!("Missing parenthesis")
			};
			let Some(color) = color.strip_suffix(')') else {
				panic!("Missing parenthesis")
			};
			let color = color.to_string();

			trenches.push(Trench {
				direction,
				length,
				color,
			});
		}

		trenches
	};

	let mut min_x = 0;
	let mut min_y = 0;
	let mut max_x = 0;
	let mut max_y = 0;
	let mut current_coordinate = Coordinate { x: 0, y: 0 };
	let mut dug_coordinates: HashSet<Coordinate> = HashSet::new();

	for trench in trenches.iter() {
		for _ in 0..trench.length {
			match trench.direction {
				Direction::North => current_coordinate.y -= 1,
				Direction::East => current_coordinate.x += 1,
				Direction::South => current_coordinate.y += 1,
				Direction::West => current_coordinate.x -= 1,
			}
			dug_coordinates.insert(current_coordinate.clone());

			min_x = min_x.min(current_coordinate.x);
			min_y = min_y.min(current_coordinate.y);
			max_x = max_x.max(current_coordinate.x);
			max_y = max_y.max(current_coordinate.y);
		}
	}

	let mut exterior_coordinates: HashSet<Coordinate> = HashSet::new();

	min_x -= 1;
	min_y -= 1;
	max_x += 1;
	max_y += 1;

	let mut current_coordinates = vec![Coordinate { x: min_x, y: min_y }];
	while !current_coordinates.is_empty() {
		let mut next_coordinates = Vec::new();
		for coordinate in current_coordinates.iter() {
			let next_north = Coordinate {
				x: coordinate.x,
				y: coordinate.y - 1,
			};
			if next_north.y >= min_y
				&& !exterior_coordinates.contains(&next_north)
				&& !dug_coordinates.contains(&next_north)
			{
				exterior_coordinates.insert(next_north.clone());
				next_coordinates.push(next_north);
			}
			let next_east = Coordinate {
				x: coordinate.x + 1,
				y: coordinate.y,
			};
			if next_east.x <= max_x
				&& !exterior_coordinates.contains(&next_east)
				&& !dug_coordinates.contains(&next_east)
			{
				exterior_coordinates.insert(next_east.clone());
				next_coordinates.push(next_east);
			}
			let next_south = Coordinate {
				x: coordinate.x,
				y: coordinate.y + 1,
			};
			if next_south.y <= max_y
				&& !exterior_coordinates.contains(&next_south)
				&& !dug_coordinates.contains(&next_south)
			{
				exterior_coordinates.insert(next_south.clone());
				next_coordinates.push(next_south);
			}
			let next_west = Coordinate {
				x: coordinate.x - 1,
				y: coordinate.y,
			};
			if next_west.x >= min_x
				&& !exterior_coordinates.contains(&next_west)
				&& !dug_coordinates.contains(&next_west)
			{
				exterior_coordinates.insert(next_west.clone());
				next_coordinates.push(next_west);
			}
		}

		current_coordinates = next_coordinates;
	}

	let full_size = (max_x - min_x + 1) * (max_y - min_y + 1);
	let dig_size = full_size - exterior_coordinates.len() as i32;

	println!("{}", dig_size);

	Ok(())
}
