use std::collections::HashMap;
use std::error::Error;
use std::fs;

#[derive(Eq, Hash, PartialEq)]
struct Coordinate {
	x: u32,
	y: u32,
}

#[derive(Clone, Copy)]
struct Coordinates {
	x: (u32, u32),
	y: u32,
}

#[derive(Clone, Copy)]
struct Number {
	value: u32,
	position: Coordinates,
}

fn main() -> Result<(), Box<dyn Error>> {
	let (numbers, gears) = {
		let input = fs::read_to_string("input.txt")?;

		let mut gears: Vec<Coordinate> = Vec::new();
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

				if c == '*' {
					let coord = Coordinate { x, y };
					gears.push(coord);
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

		(numbers, gears)
	};

	let mut number_coords: HashMap<Coordinate, Number> = HashMap::new();
	for number in numbers.iter() {
		let (low_x, high_x) = number.position.x;
		for x in low_x..high_x {
			let coord = Coordinate {
				x,
				y: number.position.y,
			};
			number_coords.insert(coord, *number);
		}
	}

	let mut gear_ratio_sum = 0;

	for gear in gears.iter() {
		let low_x = gear.x.saturating_sub(1);
		let high_x = gear.x + 1;
		let low_y = gear.y.saturating_sub(1);
		let high_y = gear.y + 1;

		let mut first_number: Option<u32> = None;
		let mut second_number: Option<u32> = None;
		'search: for x in low_x..=high_x {
			for y in low_y..=high_y {
				let coord = Coordinate { x, y };
				let found_number = number_coords.get(&coord);
				let Some(found_number) = found_number else {
					continue;
				};
				if first_number == Some(found_number.value) || second_number == Some(found_number.value) {
					continue;
				}
				if first_number.is_some() && second_number.is_some() {
					// This is a third number
					first_number = None;
					second_number = None;
					break 'search;
				}

				if first_number.is_some() {
					second_number = Some(found_number.value);
				} else {
					first_number = Some(found_number.value);
				}
			}
		}
		if let (Some(first), Some(second)) = (first_number, second_number) {
			let gear_ratio = first * second;
			gear_ratio_sum += gear_ratio;
		}
	}

	println!("{}", gear_ratio_sum);

	Ok(())
}
