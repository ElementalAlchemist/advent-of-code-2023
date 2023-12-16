use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;

#[derive(Clone, Copy)]
enum DirectorType {
	MirrorSlash,
	MirrorBackslash,
	SplitterHorizontal,
	SplitterVertical,
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum Direction {
	Up,
	Right,
	Down,
	Left,
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
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
					Some(Self {
						x: self.x,
						y: self.y - 1,
					})
				}
			}
			Direction::Right => {
				if self.x < max_x {
					Some(Self {
						x: self.x + 1,
						y: self.y,
					})
				} else {
					None
				}
			}
			Direction::Down => {
				if self.y < max_y {
					Some(Self {
						x: self.x,
						y: self.y + 1,
					})
				} else {
					None
				}
			}
			Direction::Left => {
				if self.x == 0 {
					None
				} else {
					Some(Self {
						x: self.x - 1,
						y: self.y,
					})
				}
			}
		}
	}
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct BeamEnd {
	direction: Direction,
	coordinate: Coordinate,
}

fn get_energized_count(
	start_coordinate: Coordinate,
	start_direction: Direction,
	directors: &HashMap<Coordinate, DirectorType>,
	max_x: usize,
	max_y: usize,
) -> usize {
	let mut energized: HashSet<BeamEnd> = HashSet::new();

	let mut current_beams = vec![BeamEnd {
		direction: start_direction,
		coordinate: start_coordinate,
	}];
	while !current_beams.is_empty() {
		let mut new_beams = Vec::new();
		for beam in current_beams.iter() {
			if energized.contains(beam) {
				continue;
			}
			energized.insert(beam.clone());
			match directors.get(&beam.coordinate) {
				Some(DirectorType::MirrorSlash) => {
					let direction = match beam.direction {
						Direction::Up => Direction::Right,
						Direction::Right => Direction::Up,
						Direction::Down => Direction::Left,
						Direction::Left => Direction::Down,
					};
					let coordinate = beam.coordinate.move_in_direction(direction, max_x, max_y);
					if let Some(coordinate) = coordinate {
						new_beams.push(BeamEnd { direction, coordinate });
					}
				}
				Some(DirectorType::MirrorBackslash) => {
					let direction = match beam.direction {
						Direction::Up => Direction::Left,
						Direction::Right => Direction::Down,
						Direction::Down => Direction::Right,
						Direction::Left => Direction::Up,
					};
					let coordinate = beam.coordinate.move_in_direction(direction, max_x, max_y);
					if let Some(coordinate) = coordinate {
						new_beams.push(BeamEnd { direction, coordinate });
					}
				}
				Some(DirectorType::SplitterHorizontal) => match beam.direction {
					Direction::Up | Direction::Down => {
						let left_coordinate = beam.coordinate.move_in_direction(Direction::Left, max_x, max_y);
						let right_coordinate = beam.coordinate.move_in_direction(Direction::Right, max_x, max_y);

						if let Some(coordinate) = left_coordinate {
							new_beams.push(BeamEnd {
								direction: Direction::Left,
								coordinate,
							});
						}
						if let Some(coordinate) = right_coordinate {
							new_beams.push(BeamEnd {
								direction: Direction::Right,
								coordinate,
							});
						}
					}
					_ => {
						let direction = beam.direction;
						let coordinate = beam.coordinate.move_in_direction(direction, max_x, max_y);
						if let Some(coordinate) = coordinate {
							new_beams.push(BeamEnd { direction, coordinate });
						}
					}
				},
				Some(DirectorType::SplitterVertical) => match beam.direction {
					Direction::Left | Direction::Right => {
						let up_coordinate = beam.coordinate.move_in_direction(Direction::Up, max_x, max_y);
						let down_coordinate = beam.coordinate.move_in_direction(Direction::Down, max_x, max_y);

						if let Some(coordinate) = up_coordinate {
							new_beams.push(BeamEnd {
								direction: Direction::Up,
								coordinate,
							});
						}
						if let Some(coordinate) = down_coordinate {
							new_beams.push(BeamEnd {
								direction: Direction::Down,
								coordinate,
							});
						}
					}
					_ => {
						let direction = beam.direction;
						let coordinate = beam.coordinate.move_in_direction(direction, max_x, max_y);
						if let Some(coordinate) = coordinate {
							new_beams.push(BeamEnd { direction, coordinate });
						}
					}
				},
				None => {
					let direction = beam.direction;
					let coordinate = beam.coordinate.move_in_direction(direction, max_x, max_y);
					if let Some(coordinate) = coordinate {
						new_beams.push(BeamEnd { direction, coordinate });
					}
				}
			}
		}

		current_beams = new_beams;
	}

	let energized: HashSet<Coordinate> = energized.iter().map(|beam| beam.coordinate).collect();

	energized.len()
}

fn main() -> Result<(), Box<dyn Error>> {
	let (directors, max_x, max_y) = {
		let input = fs::read_to_string("input.txt")?;

		let mut directors: HashMap<Coordinate, DirectorType> = HashMap::new();
		let mut max_x = 0;
		let mut max_y = 0;
		for (y, line) in input.lines().enumerate() {
			max_y = max_y.max(y);
			for (x, c) in line.chars().enumerate() {
				max_x = max_x.max(x);
				let director_type = match c {
					'/' => DirectorType::MirrorSlash,
					'\\' => DirectorType::MirrorBackslash,
					'-' => DirectorType::SplitterHorizontal,
					'|' => DirectorType::SplitterVertical,
					_ => continue,
				};
				let coordinate = Coordinate { x, y };

				directors.insert(coordinate, director_type);
			}
		}

		(directors, max_x, max_y)
	};

	let mut energized = 0;

	for x in 0..=max_x {
		for y_start in 0..=1 {
			let y = y_start * max_y;
			let start_coordinate = Coordinate { x, y };
			let start_direction = if y == 0 { Direction::Down } else { Direction::Up };
			energized = energized.max(get_energized_count(
				start_coordinate,
				start_direction,
				&directors,
				max_x,
				max_y,
			));
		}
	}

	for y in 0..=max_y {
		for x_start in 0..=1 {
			let x = x_start * max_x;
			let start_coordinate = Coordinate { x, y };
			let start_direction = if x == 0 { Direction::Right } else { Direction::Left };
			energized = energized.max(get_energized_count(
				start_coordinate,
				start_direction,
				&directors,
				max_x,
				max_y,
			));
		}
	}

	println!("{}", energized);

	Ok(())
}
