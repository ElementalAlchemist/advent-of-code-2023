use std::cmp::Ordering;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

const CYCLE_COUNT: usize = 1000000000;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum RockType {
	Round,
	Cube,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Coordinate {
	x: usize,
	y: usize,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Rock {
	rock_type: RockType,
	coordinate: Coordinate,
}

fn sort_by_x(lhs: &Rock, rhs: &Rock) -> Ordering {
	lhs.coordinate
		.x
		.cmp(&rhs.coordinate.x)
		.then_with(|| lhs.coordinate.y.cmp(&rhs.coordinate.y))
}

fn sort_by_y(lhs: &Rock, rhs: &Rock) -> Ordering {
	lhs.coordinate
		.y
		.cmp(&rhs.coordinate.y)
		.then_with(|| lhs.coordinate.x.cmp(&rhs.coordinate.x))
}

fn main() -> Result<(), Box<dyn Error>> {
	let (mut rocks_by_x, mut rocks_by_y, height, width) = {
		let input = fs::read_to_string("input.txt")?;

		let mut rocks: Vec<Rock> = Vec::new();
		let mut height: usize = 0;
		let mut width: usize = 0;
		for (y, line) in input.lines().enumerate() {
			height = height.max(y + 1);
			for (x, c) in line.chars().enumerate() {
				width = width.max(x + 1);
				let rock_type = match c {
					'O' => RockType::Round,
					'#' => RockType::Cube,
					_ => continue,
				};
				rocks.push(Rock {
					rock_type,
					coordinate: Coordinate { x, y },
				});
			}
		}

		let mut rocks_by_x = rocks.clone();
		let mut rocks_by_y = rocks;

		rocks_by_x.sort_unstable_by(sort_by_x);
		rocks_by_y.sort_unstable_by(sort_by_y);

		(rocks_by_x, rocks_by_y, height, width)
	};
	
	let rock_count = rocks_by_x.len();

	let mut seen_before: HashMap<Vec<Rock>, usize> = HashMap::new();
	let mut cycle_info: Option<(usize, usize)> = None;
	for cycle in 1..=CYCLE_COUNT {
		// North
		let mut new_rocks: Vec<Rock> = Vec::new();
		for rock in rocks_by_x.iter() {
			if rock.rock_type == RockType::Cube {
				new_rocks.push(rock.clone());
				continue;
			}
			let mut new_rock = rock.clone();
			if let Some(previous_rock) = new_rocks.last() {
				if previous_rock.coordinate.x == new_rock.coordinate.x {
					new_rock.coordinate.y = previous_rock.coordinate.y + 1;
				} else {
					new_rock.coordinate.y = 0;
				}
			} else {
				new_rock.coordinate.y = 0;
			}
			new_rocks.push(new_rock);
		}

		// West
		rocks_by_y = std::mem::take(&mut new_rocks);
		rocks_by_y.sort_unstable_by(sort_by_y);
		for rock in rocks_by_y.iter() {
			if rock.rock_type == RockType::Cube {
				new_rocks.push(rock.clone());
				continue;
			}
			let mut new_rock = rock.clone();
			if let Some(previous_rock) = new_rocks.last() {
				if previous_rock.coordinate.y == new_rock.coordinate.y {
					new_rock.coordinate.x = previous_rock.coordinate.x + 1;
				} else {
					new_rock.coordinate.x = 0;
				}
			} else {
				new_rock.coordinate.x = 0;
			}
			new_rocks.push(new_rock);
		}

		// South
		rocks_by_x = std::mem::take(&mut new_rocks);
		rocks_by_x.sort_unstable_by(sort_by_x);
		for rock in rocks_by_x.iter().rev() {
			if rock.rock_type == RockType::Cube {
				new_rocks.push(rock.clone());
				continue;
			}
			let mut new_rock = rock.clone();
			if let Some(previous_rock) = new_rocks.last() {
				if previous_rock.coordinate.x == new_rock.coordinate.x {
					new_rock.coordinate.y = previous_rock.coordinate.y - 1;
				} else {
					new_rock.coordinate.y = height - 1;
				}
			} else {
				new_rock.coordinate.y = height - 1;
			}
			new_rocks.push(new_rock);
		}

		// East
		rocks_by_y = std::mem::take(&mut new_rocks);
		rocks_by_y.sort_unstable_by(sort_by_y);
		for rock in rocks_by_y.iter().rev() {
			if rock.rock_type == RockType::Cube {
				new_rocks.push(rock.clone());
				continue;
			}
			let mut new_rock = rock.clone();
			if let Some(previous_rock) = new_rocks.last() {
				if previous_rock.coordinate.y == new_rock.coordinate.y {
					new_rock.coordinate.x = previous_rock.coordinate.x - 1;
				} else {
					new_rock.coordinate.x = width - 1;
				}
			} else {
				new_rock.coordinate.x = width - 1;
			}
			new_rocks.push(new_rock);
		}

		rocks_by_x = new_rocks.clone();
		rocks_by_y = new_rocks;

		rocks_by_x.sort_unstable_by(sort_by_x);
		rocks_by_y.sort_unstable_by(sort_by_y);

		match seen_before.get(&rocks_by_y) {
			Some(original_cycle) => {
				cycle_info = Some((*original_cycle, cycle));
				break;
			}
			None => seen_before.insert(rocks_by_y.clone(), cycle),
		};

		assert_eq!(rock_count, rocks_by_x.len());
	}

	if let Some((cycle_start, cycle_repeat)) = cycle_info {
		let cycle_length = cycle_repeat - cycle_start;
		let cycle_count_aligned = CYCLE_COUNT - cycle_start;
		let cycle_past_start = cycle_count_aligned % cycle_length;
		let final_cycle_equivalence_number = cycle_start + cycle_past_start;
		for (seen_rocks_by_y, cycle_number) in seen_before.iter() {
			if *cycle_number == final_cycle_equivalence_number {
				rocks_by_y = seen_rocks_by_y.clone();
				break;
			}
		}
	}

	let mut load = 0;
	for rock in rocks_by_y.iter().filter(|rock| rock.rock_type == RockType::Round) {
		load += height - rock.coordinate.y;
	}

	println!("{}", load);

	Ok(())
}
