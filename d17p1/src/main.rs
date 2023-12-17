use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashSet};
use std::error::Error;
use std::fs;

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Direction {
	Up,
	Right,
	Down,
	Left,
}

impl Direction {
	fn opposite(&self) -> Direction {
		match self {
			Self::Up => Self::Down,
			Self::Right => Self::Left,
			Self::Down => Self::Up,
			Self::Left => Self::Right,
		}
	}
}

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Coordinate {
	x: usize,
	y: usize,
}

impl Coordinate {
	fn move_in_direction(&self, direction: Direction, max_x: usize, max_y: usize) -> Option<Self> {
		match direction {
			Direction::Up => {
				if self.y == 0 {
					None
				} else {
					Some(Coordinate {
						x: self.x,
						y: self.y - 1,
					})
				}
			}
			Direction::Right => {
				if self.x == max_x {
					None
				} else {
					Some(Coordinate {
						x: self.x + 1,
						y: self.y,
					})
				}
			}
			Direction::Down => {
				if self.y == max_y {
					None
				} else {
					Some(Coordinate {
						x: self.x,
						y: self.y + 1,
					})
				}
			}
			Direction::Left => {
				if self.x == 0 {
					None
				} else {
					Some(Coordinate {
						x: self.x - 1,
						y: self.y,
					})
				}
			}
		}
	}
}

#[derive(Clone, Eq, PartialEq)]
struct CartProgress {
	heat_loss: usize,
	current_location: Coordinate,
	current_direction: Direction,
	direction_distance: usize,
}

impl CartProgress {
	fn to_visited_equivalent(&self) -> CartVisited {
		CartVisited {
			location: self.current_location,
			direction: self.current_direction,
			direction_distance: self.direction_distance,
		}
	}
}

impl Default for CartProgress {
	fn default() -> Self {
		let heat_loss = 0;
		let current_location = Coordinate { x: 0, y: 0 };
		let current_direction = Direction::Right;
		let direction_distance = 0;
		Self {
			heat_loss,
			current_location,
			current_direction,
			direction_distance,
		}
	}
}

impl Ord for CartProgress {
	fn cmp(&self, other: &Self) -> Ordering {
		self.heat_loss
			.cmp(&other.heat_loss)
			.then_with(|| self.current_location.cmp(&other.current_location))
			.then_with(|| self.current_direction.cmp(&other.current_direction))
			.then_with(|| self.direction_distance.cmp(&other.direction_distance))
	}
}

impl PartialOrd for CartProgress {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct CartVisited {
	location: Coordinate,
	direction: Direction,
	direction_distance: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
	let (block_heat_loss, max_x, max_y) = {
		let input = fs::read_to_string("input.txt")?;

		let mut block_heat_loss: Vec<Vec<usize>> = Vec::new();
		let mut max_x = 0;
		let mut max_y = 0;

		for (y, line) in input.lines().enumerate() {
			max_y = max_y.max(y);
			let mut line_heat_loss: Vec<usize> = Vec::new();

			for (x, c) in line.chars().enumerate() {
				max_x = max_x.max(x);
				let loss = c.to_digit(10).unwrap() as usize;
				line_heat_loss.push(loss);
			}

			block_heat_loss.push(line_heat_loss);
		}

		(block_heat_loss, max_x, max_y)
	};

	let destination = Coordinate { x: max_x, y: max_y };

	let mut path_locations: BinaryHeap<Reverse<CartProgress>> = BinaryHeap::new();
	path_locations.push(Reverse(CartProgress::default()));

	let all_directions = [Direction::Up, Direction::Right, Direction::Down, Direction::Left];
	let mut visited: HashSet<CartVisited> = HashSet::new();

	'cart: while let Some(Reverse(cart_progress)) = path_locations.pop() {
		let visited_equivalent = cart_progress.to_visited_equivalent();
		if visited.contains(&visited_equivalent) {
			continue;
		}
		visited.insert(visited_equivalent);

		for direction in all_directions.iter() {
			if *direction == cart_progress.current_direction.opposite() {
				continue;
			}
			if *direction == cart_progress.current_direction && cart_progress.direction_distance >= 3 {
				continue;
			}
			let next_coordinate = cart_progress
				.current_location
				.move_in_direction(*direction, max_x, max_y);
			let Some(next_coordinate) = next_coordinate else {
				continue;
			};

			let new_heat_loss = cart_progress.heat_loss + block_heat_loss[next_coordinate.y][next_coordinate.x];
			let new_direction_distance = if *direction == cart_progress.current_direction {
				cart_progress.direction_distance + 1
			} else {
				1
			};

			if next_coordinate == destination {
				println!("{}", new_heat_loss);
				break 'cart;
			}

			path_locations.push(Reverse(CartProgress {
				heat_loss: new_heat_loss,
				current_location: next_coordinate,
				current_direction: *direction,
				direction_distance: new_direction_distance,
			}));
		}
	}

	Ok(())
}
