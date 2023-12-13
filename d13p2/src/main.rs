use std::collections::HashMap;
use std::error::Error;
use std::fs;

#[derive(Clone, Copy, Eq, PartialEq)]
enum DifferenceType {
	Same,
	Single,
	Multiple,
}

fn find_differences(lhs: &[usize], rhs: &[usize]) -> DifferenceType {
	if *lhs == *rhs {
		return DifferenceType::Same;
	}

	let mut lhs_iter = lhs.iter();
	let mut rhs_iter = rhs.iter();
	loop {
		let lhs_val = lhs_iter.next();
		let rhs_val = rhs_iter.next();
		let Some(lhs_val) = lhs_val else {
			match rhs_val {
				Some(_) => {
					if rhs_iter.next().is_some() {
						return DifferenceType::Multiple;
					}
					return DifferenceType::Single;
				}
				None => {
					return DifferenceType::Same;
				}
			}
		};
		let Some(rhs_val) = rhs_val else {
			if lhs_iter.next().is_some() {
				return DifferenceType::Multiple;
			}
			return DifferenceType::Single;
		};

		if *lhs_val != *rhs_val {
			let mut new_lhs: Vec<usize> = lhs_iter.copied().collect();
			let mut new_rhs: Vec<usize> = rhs_iter.copied().collect();
			new_lhs.insert(0, *lhs_val);
			new_rhs.insert(0, *rhs_val);

			let advance_lhs_differences = find_differences(&new_lhs[1..], &new_rhs);
			let advance_rhs_differences = find_differences(&new_lhs, &new_rhs[1..]);

			if advance_lhs_differences == DifferenceType::Same || advance_rhs_differences == DifferenceType::Same {
				break DifferenceType::Single;
			}
			break DifferenceType::Multiple;
		}
	}
}

struct Grid {
	coords_by_x: HashMap<usize, Vec<usize>>,
	coords_by_y: HashMap<usize, Vec<usize>>,
	max_x: usize,
	max_y: usize,
}

impl Grid {
	fn new() -> Self {
		let coords_by_x = HashMap::new();
		let coords_by_y = HashMap::new();
		let max_x = 0;
		let max_y = 0;
		Self {
			coords_by_x,
			coords_by_y,
			max_x,
			max_y,
		}
	}

	fn add_coordinate(&mut self, x: usize, y: usize) {
		let x_y_coords = self.coords_by_x.entry(x).or_default();
		x_y_coords.push(y);
		x_y_coords.sort_unstable();

		let y_x_coords = self.coords_by_y.entry(y).or_default();
		y_x_coords.push(x);
		y_x_coords.sort_unstable();

		self.max_x = self.max_x.max(x);
		self.max_y = self.max_y.max(y);
	}

	/// Finds a reflection over a line on a single x coordinate. Returns the number of columns to the left.
	fn find_x_reflection(&self) -> Option<usize> {
		'x: for x in 1..=self.max_x {
			let mut found_single_difference = false;
			for low_x in (0..x).rev() {
				let high_x = x + (x - low_x) - 1;
				if high_x > self.max_x {
					break;
				}
				let low_x_coords = self.coords_by_x.get(&low_x).cloned().unwrap_or_default();
				let high_x_coords = self.coords_by_x.get(&high_x).cloned().unwrap_or_default();
				match find_differences(&low_x_coords, &high_x_coords) {
					DifferenceType::Same => (),
					DifferenceType::Single => {
						if found_single_difference {
							continue 'x;
						}
						found_single_difference = true;
					}
					DifferenceType::Multiple => continue 'x,
				}
			}
			if found_single_difference {
				return Some(x);
			}
		}
		None
	}

	/// Finds a reflection over a line on a single y coordinate. Returns the number of rows above.
	fn find_y_reflection(&self) -> Option<usize> {
		'y: for y in 1..=self.max_y {
			let mut found_single_difference = false;
			for low_y in (0..y).rev() {
				let high_y = y + (y - low_y) - 1;
				if high_y > self.max_y {
					break;
				}
				let low_y_coords = self.coords_by_y.get(&low_y).cloned().unwrap_or_default();
				let high_y_coords = self.coords_by_y.get(&high_y).cloned().unwrap_or_default();
				match find_differences(&low_y_coords, &high_y_coords) {
					DifferenceType::Same => (),
					DifferenceType::Single => {
						if found_single_difference {
							continue 'y;
						}
						found_single_difference = true;
					}
					DifferenceType::Multiple => continue 'y,
				}
			}
			if found_single_difference {
				return Some(y);
			}
		}
		None
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let grids = {
		let input = fs::read_to_string("input.txt")?;

		let mut grids: Vec<Grid> = Vec::new();
		let mut grid_start: usize = 0;
		let mut current_grid = Grid::new();
		for (y, line) in input.lines().enumerate() {
			if line.is_empty() {
				grids.push(current_grid);
				current_grid = Grid::new();
				grid_start = y + 1;
				continue;
			}

			for (x, c) in line.chars().enumerate() {
				if c == '#' {
					current_grid.add_coordinate(x, y - grid_start);
				}
			}
		}
		if !current_grid.coords_by_x.is_empty() {
			grids.push(current_grid);
		}

		grids
	};

	let mut single_lines = 0;
	for grid in grids.iter() {
		let x_point = grid.find_x_reflection();
		if let Some(x) = x_point {
			single_lines += x;
			continue;
		}

		let y_point = grid.find_y_reflection();
		if let Some(y) = y_point {
			single_lines += y * 100;
		}
	}

	println!("{}", single_lines);

	Ok(())
}
