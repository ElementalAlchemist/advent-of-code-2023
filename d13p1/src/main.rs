use std::collections::HashMap;
use std::error::Error;
use std::fs;

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
			for low_x in (0..x).rev() {
				let high_x = x + (x - low_x) - 1;
				if high_x > self.max_x {
					break;
				}
				if self.coords_by_x.get(&low_x) != self.coords_by_x.get(&high_x) {
					continue 'x;
				}
			}
			return Some(x);
		}
		None
	}

	/// Finds a reflection over a line on a single y coordinate. Returns the number of rows above.
	fn find_y_reflection(&self) -> Option<usize> {
		'y: for y in 1..=self.max_y {
			for low_y in (0..y).rev() {
				let high_y = y + (y - low_y) - 1;
				if high_y > self.max_y {
					break;
				}
				if self.coords_by_y.get(&low_y) != self.coords_by_y.get(&high_y) {
					continue 'y;
				}
			}
			return Some(y);
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
