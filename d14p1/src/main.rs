use std::error::Error;
use std::fs;

#[derive(Default)]
struct Column {
	round_rocks: Vec<usize>,
	cube_rocks: Vec<usize>,
}

fn main() -> Result<(), Box<dyn Error>> {
	let (columns, height) = {
		let input = fs::read_to_string("input.txt")?;

		let mut columns: Vec<Column> = Vec::new();
		let mut height: usize = 0;
		for (y, line) in input.lines().enumerate() {
			height += 1;
			for (x, c) in line.chars().enumerate() {
				while columns.len() <= x {
					columns.push(Column::default());
				}
				match c {
					'O' => columns[x].round_rocks.push(y),
					'#' => columns[x].cube_rocks.push(y),
					_ => (),
				}
			}
		}

		(columns, height)
	};

	let mut load = 0;
	for column in columns.iter() {
		let mut new_round_rocks = Vec::new();
		for rock in column.round_rocks.iter() {
			let mut position = *rock;
			while position > 0
				&& !new_round_rocks.contains(&(position - 1))
				&& !column.cube_rocks.contains(&(position - 1))
			{
				position -= 1;
			}
			new_round_rocks.push(position);
		}

		for rock in new_round_rocks.iter() {
			load += height - rock;
		}
	}

	println!("{}", load);

	Ok(())
}
