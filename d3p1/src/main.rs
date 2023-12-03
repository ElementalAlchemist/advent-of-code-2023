use std::collections::HashSet;
use std::error::Error;
use std::fs;

#[derive(Eq, Hash, PartialEq)]
struct Coordinate {
	x: u32,
	y: u32,
}

struct Coordinates {
	x: (u32, u32),
	y: u32,
}

struct Number {
	value: u32,
	position: Coordinates,
}

fn main() -> Result<(), Box<dyn Error>> {
	let (numbers, symbols) = {
		let input = fs::read_to_string("input.txt")?;

		let mut symbols: HashSet<Coordinate> = HashSet::new();
		let mut numbers: Vec<Number> = Vec::new();
		for (y, line) in input.lines().enumerate() {
			let y = y as u32;

			let mut current_number: u32 = 0;
			let mut number_start: Option<u32> = None;
			for (x, c) in line.chars().enumerate() {
				let x = x as u32;

				if c.is_ascii_digit() {
					if number_start.is_none() {
						number_start = Some(x);
					}
					current_number = current_number * 10 + (c as u32 - 48);
				} else if let Some(start) = number_start {
					let number_coords = Coordinates { x: (start, x), y };
					let number = Number {
						value: current_number,
						position: number_coords,
					};
					numbers.push(number);
					current_number = 0;
					number_start = None;
				}

				if c != '.' && !c.is_ascii_digit() {
					let coord = Coordinate { x, y };
					symbols.insert(coord);
				}
			}
			if let Some(start) = number_start {
				let x = (start, line.len() as u32);
				let number_coords = Coordinates { x, y };
				let number = Number {
					value: current_number,
					position: number_coords,
				};
				numbers.push(number);
			}
		}

		(numbers, symbols)
	};

	let mut part_number_sum: u32 = 0;

	for number in numbers {
		let (low_x, high_x) = number.position.x;
		let low_x = low_x.saturating_sub(1);

		let low_y = number.position.y.saturating_sub(1);
		let high_y = number.position.y + 1;

		'coord: for x in low_x..=high_x {
			for y in low_y..=high_y {
				let coord = Coordinate { x, y };
				if symbols.contains(&coord) {
					part_number_sum += number.value;
					println!("Included number {}", number.value);
					break 'coord;
				}
			}
		}
	}

	println!("{}", part_number_sum);

	Ok(())
}
