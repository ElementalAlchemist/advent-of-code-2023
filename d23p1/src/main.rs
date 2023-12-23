use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;

#[derive(Clone, Eq, Hash, PartialEq)]
struct Coordinate {
	x: usize,
	y: usize,
}

impl Coordinate {
	fn all_adjacent(&self) -> Vec<Self> {
		let mut adjacents: Vec<Self> = Vec::new();

		if self.x > 0 {
			adjacents.push(Self {
				x: self.x - 1,
				y: self.y,
			});
		}
		if self.y > 0 {
			adjacents.push(Self {
				x: self.x,
				y: self.y - 1,
			});
		}
		adjacents.push(Self {
			x: self.x + 1,
			y: self.y,
		});
		adjacents.push(Self {
			x: self.x,
			y: self.y + 1,
		});

		adjacents
	}
}

#[derive(Clone)]
enum TileType {
	Free,
	MustTravelNext(Coordinate),
}

#[derive(Clone)]
struct PathProgress {
	location: Coordinate,
	visited: HashSet<Coordinate>,
}

fn main() -> Result<(), Box<dyn Error>> {
	let tiles = {
		let input = fs::read_to_string("input.txt")?;

		let mut tiles: HashMap<Coordinate, TileType> = HashMap::new();
		for (y, line) in input.lines().enumerate() {
			for (x, c) in line.chars().enumerate() {
				let coordinate = Coordinate { x, y };
				match c {
					'#' => (),
					'.' => {
						tiles.insert(coordinate, TileType::Free);
					}
					'>' => {
						let next_coordinate = Coordinate { x: x + 1, y };
						tiles.insert(coordinate, TileType::MustTravelNext(next_coordinate));
					}
					'v' => {
						let next_coordinate = Coordinate { x, y: y + 1 };
						tiles.insert(coordinate, TileType::MustTravelNext(next_coordinate));
					}
					'<' => {
						let next_coordinate = Coordinate { x: x - 1, y };
						tiles.insert(coordinate, TileType::MustTravelNext(next_coordinate));
					}
					'^' => {
						let next_coordinate = Coordinate { x, y: y - 1 };
						tiles.insert(coordinate, TileType::MustTravelNext(next_coordinate));
					}
					_ => panic!("Unexpected character: {}", c),
				}
			}
		}

		tiles
	};

	let start_coord_x = tiles
		.keys()
		.filter(|coord| coord.y == 0)
		.map(|coord| coord.x)
		.min()
		.unwrap();
	let start_coord = Coordinate { x: start_coord_x, y: 0 };
	let mut start_progress = PathProgress {
		location: start_coord.clone(),
		visited: HashSet::new(),
	};
	start_progress.visited.insert(start_coord);

	let mut distance_traveled = -1;
	let mut progress = vec![start_progress];

	while !progress.is_empty() {
		distance_traveled += 1;
		let mut next_progress = Vec::new();
		for path in progress.iter() {
			let tile_type = tiles.get(&path.location).unwrap();
			match tile_type {
				TileType::Free => {
					for next_coordinate in path.location.all_adjacent() {
						if path.visited.contains(&next_coordinate) {
							continue;
						}
						if let Some(next_tile) = tiles.get(&next_coordinate) {
							if let TileType::MustTravelNext(travel_coordinate) = next_tile {
								if *travel_coordinate == path.location {
									continue;
								}
							}
							let mut visited = path.visited.clone();
							visited.insert(next_coordinate.clone());
							next_progress.push(PathProgress {
								location: next_coordinate,
								visited,
							});
						}
					}
				}
				TileType::MustTravelNext(next_coordinate) => {
					let mut visited = path.visited.clone();
					visited.insert(next_coordinate.clone());
					next_progress.push(PathProgress {
						location: next_coordinate.clone(),
						visited,
					});
				}
			}
		}

		progress = next_progress;
	}

	println!("{}", distance_traveled);

	Ok(())
}
