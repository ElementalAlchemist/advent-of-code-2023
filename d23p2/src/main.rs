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

struct PathProgress {
	location: Coordinate,
	visited: HashSet<Coordinate>,
}

struct IntersectionProgress {
	location: Coordinate,
	visited: HashSet<Coordinate>,
	distance: u32,
}

fn main() -> Result<(), Box<dyn Error>> {
	let (tiles, start, destination) = {
		let input = fs::read_to_string("input.txt")?;

		let mut tiles: HashSet<Coordinate> = HashSet::new();
		let mut first_tile: Option<Coordinate> = None;
		let mut last_tile: Option<Coordinate> = None;
		for (y, line) in input.lines().enumerate() {
			for (x, c) in line.chars().enumerate() {
				let coordinate = Coordinate { x, y };
				match c {
					'#' => (),
					'.' | '>' | 'v' | '<' | '^' => {
						tiles.insert(coordinate.clone());
						if first_tile.is_none() {
							first_tile = Some(coordinate.clone());
						}
						last_tile = Some(coordinate.clone());
					}
					_ => panic!("Unexpected character: {}", c),
				}
			}
		}

		(tiles, first_tile.unwrap(), last_tile.unwrap())
	};

	let mut intersections: HashSet<Coordinate> = HashSet::new();
	for tile in tiles.iter() {
		let adjacent_tiles = tile.all_adjacent();
		if adjacent_tiles.iter().filter(|coord| tiles.contains(*coord)).count() > 2 {
			intersections.insert(tile.clone());
		}
	}

	let mut adjacent_intersections: HashMap<Coordinate, HashMap<Coordinate, u32>> = HashMap::new();
	for source_intersection in intersections.iter() {
		let mut distance_traveled = 0;
		let mut start_progress = PathProgress {
			location: source_intersection.clone(),
			visited: HashSet::new(),
		};
		start_progress.visited.insert(source_intersection.clone());

		let mut progress = vec![start_progress];
		while !progress.is_empty() {
			distance_traveled += 1;
			let mut next_progress = Vec::new();
			for path in progress.iter() {
				for next_coordinate in path.location.all_adjacent() {
					if !path.visited.contains(&next_coordinate) && tiles.contains(&next_coordinate) {
						if intersections.contains(&next_coordinate) {
							adjacent_intersections
								.entry(source_intersection.clone())
								.or_default()
								.insert(next_coordinate, distance_traveled);
						} else {
							let mut visited = path.visited.clone();
							visited.insert(next_coordinate.clone());
							next_progress.push(PathProgress {
								location: next_coordinate,
								visited,
							});
						}
					}
				}
			}

			progress = next_progress;
		}
	}

	let mut start_distance = 0;
	let mut start_path_coordinate = start.clone();
	let mut start_visited: HashSet<Coordinate> = HashSet::new();
	start_visited.insert(start.clone());
	let start_intersection = 'start_distance: loop {
		start_distance += 1;
		for next_coordinate in start_path_coordinate.all_adjacent() {
			if intersections.contains(&next_coordinate) {
				break 'start_distance next_coordinate;
			}
			if tiles.contains(&next_coordinate) && !start_visited.contains(&next_coordinate) {
				start_path_coordinate = next_coordinate.clone();
				start_visited.insert(next_coordinate);
				break;
			}
		}
	};
	let start_distance = start_distance;
	drop(start_visited);

	let mut end_distance = 0;
	let mut end_path_coordinate = destination.clone();
	let mut end_visited: HashSet<Coordinate> = HashSet::new();
	end_visited.insert(destination.clone());
	let end_intersection = 'end_distance: loop {
		end_distance += 1;
		for next_coordinate in end_path_coordinate.all_adjacent() {
			if intersections.contains(&next_coordinate) {
				break 'end_distance next_coordinate;
			}
			if tiles.contains(&next_coordinate) && !end_visited.contains(&next_coordinate) {
				end_path_coordinate = next_coordinate.clone();
				end_visited.insert(next_coordinate);
				break;
			}
		}
	};
	let end_distance = end_distance;
	drop(end_visited);

	adjacent_intersections.remove(&end_intersection); // prevent accidental advancement

	let mut start_progress = IntersectionProgress {
		location: start_intersection.clone(),
		visited: HashSet::new(),
		distance: start_distance + end_distance,
	};
	start_progress.visited.insert(start_intersection);

	let mut progress = vec![start_progress];
	let mut max_distance = 0;
	while !progress.is_empty() {
		let mut next_progress = Vec::new();
		for path in progress.iter() {
			for (next_coordinate, distance) in adjacent_intersections.get(&path.location).unwrap().iter() {
				if *next_coordinate == end_intersection {
					max_distance = max_distance.max(*distance + path.distance);
				} else if !path.visited.contains(next_coordinate) {
					let mut visited = path.visited.clone();
					visited.insert(next_coordinate.clone());
					let distance = *distance + path.distance;
					next_progress.push(IntersectionProgress {
						location: next_coordinate.clone(),
						visited,
						distance,
					});
				}
			}
		}

		progress = next_progress;
	}

	println!("{}", max_distance);

	Ok(())
}
