use std::collections::{BTreeSet, HashMap, HashSet};
use std::error::Error;
use std::fs;

const STEP_COUNT: i32 = 26501365;

#[derive(Clone, Eq, Hash, PartialEq)]
struct Coordinate {
	map_x: i32,
	map_y: i32,
	x: usize,
	y: usize,
}

impl Coordinate {
	fn all_adjacent(&self, max_x: usize, max_y: usize) -> Vec<Self> {
		let mut adjacent = Vec::new();

		let west = if self.x == 0 {
			Coordinate {
				map_x: self.map_x - 1,
				map_y: self.map_y,
				x: max_x,
				y: self.y,
			}
		} else {
			Coordinate {
				map_x: self.map_x,
				map_y: self.map_y,
				x: self.x - 1,
				y: self.y,
			}
		};
		adjacent.push(west);

		let north = if self.y == 0 {
			Coordinate {
				map_x: self.map_x,
				map_y: self.map_y - 1,
				x: self.x,
				y: max_y,
			}
		} else {
			Coordinate {
				map_x: self.map_x,
				map_y: self.map_y,
				x: self.x,
				y: self.y - 1,
			}
		};
		adjacent.push(north);

		let east = if self.x < max_x {
			Coordinate {
				map_x: self.map_x,
				map_y: self.map_y,
				x: self.x + 1,
				y: self.y,
			}
		} else {
			Coordinate {
				map_x: self.map_x + 1,
				map_y: self.map_y,
				x: 0,
				y: self.y,
			}
		};
		adjacent.push(east);

		let south = if self.y < max_y {
			Coordinate {
				map_x: self.map_x,
				map_y: self.map_y,
				x: self.x,
				y: self.y + 1,
			}
		} else {
			Coordinate {
				map_x: self.map_x,
				map_y: self.map_y + 1,
				x: self.x,
				y: 0,
			}
		};
		adjacent.push(south);

		adjacent
	}

	fn as_step(&self) -> StepCoordinate {
		StepCoordinate { x: self.x, y: self.y }
	}

	fn as_map_only(&self) -> MapCoordinate {
		MapCoordinate {
			x: self.map_x,
			y: self.map_y,
		}
	}
}

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct StepCoordinate {
	x: usize,
	y: usize,
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct MapCoordinate {
	x: i32,
	y: i32,
}

fn single_direction_count(
	complete_plots: i32,
	remaining_steps: i32,
	partial: &Vec<(i32, usize)>,
	mut even_coord: bool,
	steps_complete_diff: i32,
	even_plot_count: usize,
	odd_plot_count: usize,
) -> usize {
	let complete_plots = complete_plots as usize;
	let half_complete_plots = complete_plots / 2;
	if STEP_COUNT % 2 == 1 {
		even_coord = !even_coord;
	}
	let extra_plot_count = if even_coord { odd_plot_count } else { even_plot_count };
	let total_plots = half_complete_plots * even_plot_count
		+ half_complete_plots * odd_plot_count
		+ (complete_plots % 2) * extra_plot_count;

	let mut partial_index = partial.len();
	partial_index -= (steps_complete_diff - remaining_steps) as usize;
	let steps_complete_diff = steps_complete_diff as usize;
	let mut partial_total = if partial_index == partial.len() {
		0
	} else {
		partial[partial_index].1
	};
	while partial_index >= steps_complete_diff {
		partial_index -= steps_complete_diff;
		partial_total += partial[partial_index].1;
	}

	total_plots + partial_total
}

fn corner_count(
	complete_plots_direction: i32,
	remaining: i32,
	partial: &Vec<(i32, usize)>,
	steps_complete_diff: i32,
	even_plot_count: usize,
	odd_plot_count: usize,
) -> usize {
	let complete_plots_direction = complete_plots_direction as usize;
	let mut total_plots = 0;
	let mut use_even = STEP_COUNT % 2 == 1;
	for plot_count in 1..=complete_plots_direction {
		total_plots += if use_even { even_plot_count } else { odd_plot_count } * plot_count;
		use_even = !use_even;
	}

	let mut partial_index = partial.len();
	partial_index -= (steps_complete_diff - remaining) as usize;
	let steps_complete_diff = steps_complete_diff as usize;
	let mut partial_total_column = if partial_index == partial.len() {
		0
	} else {
		partial[partial_index].1
	};
	let mut partial_calc_index = partial_index;
	while partial_calc_index >= steps_complete_diff {
		partial_calc_index -= steps_complete_diff;
		partial_total_column += partial[partial_calc_index].1;
	}

	let mut partial_total = partial_total_column * (complete_plots_direction + 1);

	let mut apply_subpartial_count = 1;
	while partial_index >= steps_complete_diff {
		partial_index -= steps_complete_diff;
		partial_total += partial[partial_index].1 * apply_subpartial_count;
		apply_subpartial_count += 1;
	}

	total_plots + partial_total
}

fn main() -> Result<(), Box<dyn Error>> {
	let (start, steppable, max_x, max_y) = {
		let input = fs::read_to_string("input.txt")?;

		let mut start: Option<Coordinate> = None;
		let mut steppable: HashSet<StepCoordinate> = HashSet::new();
		let mut max_x = 0;
		let mut max_y = 0;
		for (y, line) in input.lines().enumerate() {
			max_y = max_y.max(y);
			for (x, c) in line.chars().enumerate() {
				max_x = max_x.max(x);
				if c == 'S' {
					assert!(start.is_none());
					start = Some(Coordinate {
						map_x: 0,
						map_y: 0,
						x,
						y,
					});
					steppable.insert(StepCoordinate { x, y });
				} else if c == '.' {
					steppable.insert(StepCoordinate { x, y });
				}
			}
		}

		(start.unwrap(), steppable, max_x, max_y)
	};

	let mut seen_plots: HashSet<BTreeSet<StepCoordinate>> = HashSet::new();
	let mut completed_counts: Option<(usize, usize)> = None;
	let mut partially_completed_count: Option<usize> = None;
	let mut current_locations: HashSet<Coordinate> = HashSet::new();
	current_locations.insert(start.clone());
	for current_step in 1..=STEP_COUNT {
		let mut next_locations: HashSet<Coordinate> = HashSet::new();
		for location in current_locations {
			let adjacents = location.all_adjacent(max_x, max_y);
			for adjacent in adjacents {
				if steppable.contains(&adjacent.as_step()) && adjacent.map_x == 0 && adjacent.map_y == 0 {
					next_locations.insert(adjacent);
				}
			}
		}

		current_locations = next_locations;

		if let Some(count) = partially_completed_count.take() {
			let completed_info = if current_step % 2 == 0 {
				(current_locations.len(), count)
			} else {
				(count, current_locations.len())
			};
			completed_counts = Some(completed_info);
			break;
		}
		let current_locations_check: BTreeSet<StepCoordinate> =
			current_locations.iter().map(|coord| coord.as_step()).collect();
		if seen_plots.contains(&current_locations_check) {
			partially_completed_count = Some(current_locations.len());
		} else {
			seen_plots.insert(current_locations_check);
		}
	}

	drop(seen_plots);
	drop(current_locations);
	let completed_counts = completed_counts.unwrap();

	let mut completed_plots: HashMap<MapCoordinate, i32> = HashMap::new();
	let mut current_locations: HashSet<Coordinate> = HashSet::new();
	let mut outer_progress: HashMap<MapCoordinate, Vec<(i32, usize)>> = HashMap::new();
	current_locations.insert(start);
	for current_step in 1..=STEP_COUNT {
		let mut next_locations: HashSet<Coordinate> = HashSet::new();
		for location in current_locations {
			let adjacents = location.all_adjacent(max_x, max_y);
			for adjacent in adjacents {
				if steppable.contains(&adjacent.as_step())
				&& !completed_plots.contains_key(&adjacent.as_map_only())
				&& adjacent.map_x.abs() <= 3
				&& adjacent.map_y.abs() <= 3
				{
					next_locations.insert(adjacent);
				}
			}
		}

		current_locations = next_locations;

		let mut per_map_steps: HashMap<MapCoordinate, BTreeSet<StepCoordinate>> = HashMap::new();
		for location in current_locations.iter() {
			let map_set = per_map_steps.entry(location.as_map_only()).or_default();
			map_set.insert(location.as_step());
		}

		for (map, plot) in per_map_steps {
			let completed_count = if (map.x + map.y + current_step) % 2 == 0 {
				completed_counts.0
			} else {
				completed_counts.1
			};
			let plot_len = plot.len();
			if plot_len == completed_count {
				completed_plots.insert(map.clone(), current_step);
			} else {
				outer_progress.entry(map).or_default().push((current_step, plot_len));
			}
		}

		current_locations.retain(|coord| !completed_plots.contains_key(&coord.as_map_only()));
		if current_locations.is_empty() {
			break;
		}
	}

	let pos_x_close = MapCoordinate { x: 2, y: 0 };
	let pos_x_far = MapCoordinate { x: 3, y: 0 };

	let steps_complete_diff = *completed_plots.get(&pos_x_far).unwrap() - *completed_plots.get(&pos_x_close).unwrap();

	let mut central_complete = 0;
	let (even_plot_count, odd_plot_count) = completed_counts;
	for x in -2..=2 {
		for y in -2..=2 {
			central_complete += if (x + y + STEP_COUNT) % 2 == 0 {
				even_plot_count
			} else {
				odd_plot_count
			};
		}
	}
	let central_complete = central_complete;

	let up_1_complete_offset = *completed_plots.get(&MapCoordinate { x: -1, y: -2 }).unwrap();
	let up_2_complete_offset = *completed_plots.get(&MapCoordinate { x: 0, y: -2 }).unwrap();
	let up_3_complete_offset = *completed_plots.get(&MapCoordinate { x: 1, y: -2 }).unwrap();
	let up_right_complete_offset = *completed_plots.get(&MapCoordinate { x: 2, y: -2 }).unwrap();

	let right_1_complete_offset = *completed_plots.get(&MapCoordinate { x: 2, y: -1 }).unwrap();
	let right_2_complete_offset = *completed_plots.get(&MapCoordinate { x: 2, y: 0 }).unwrap();
	let right_3_complete_offset = *completed_plots.get(&MapCoordinate { x: 2, y: 1 }).unwrap();
	let right_down_complete_offset = *completed_plots.get(&MapCoordinate { x: 2, y: 2 }).unwrap();

	let down_1_complete_offset = *completed_plots.get(&MapCoordinate { x: 1, y: 2 }).unwrap();
	let down_2_complete_offset = *completed_plots.get(&MapCoordinate { x: 0, y: 2 }).unwrap();
	let down_3_complete_offset = *completed_plots.get(&MapCoordinate { x: -1, y: 2 }).unwrap();
	let down_left_complete_offset = *completed_plots.get(&MapCoordinate { x: -2, y: 2 }).unwrap();

	let left_1_complete_offset = *completed_plots.get(&MapCoordinate { x: -2, y: 1 }).unwrap();
	let left_2_complete_offset = *completed_plots.get(&MapCoordinate { x: -2, y: 0 }).unwrap();
	let left_3_complete_offset = *completed_plots.get(&MapCoordinate { x: -2, y: -1 }).unwrap();
	let left_up_complete_offset = *completed_plots.get(&MapCoordinate { x: -2, y: -2 }).unwrap();

	let up_1_complete = (STEP_COUNT - up_1_complete_offset) / steps_complete_diff;
	let up_1_remaining = (STEP_COUNT - up_1_complete_offset) % steps_complete_diff;
	let up_2_complete = (STEP_COUNT - up_2_complete_offset) / steps_complete_diff;
	let up_2_remaining = (STEP_COUNT - up_2_complete_offset) % steps_complete_diff;
	let up_3_complete = (STEP_COUNT - up_3_complete_offset) / steps_complete_diff;
	let up_3_remaining = (STEP_COUNT - up_3_complete_offset) % steps_complete_diff;

	let right_1_complete = (STEP_COUNT - right_1_complete_offset) / steps_complete_diff;
	let right_1_remaining = (STEP_COUNT - right_1_complete_offset) % steps_complete_diff;
	let right_2_complete = (STEP_COUNT - right_2_complete_offset) / steps_complete_diff;
	let right_2_remaining = (STEP_COUNT - right_2_complete_offset) % steps_complete_diff;
	let right_3_complete = (STEP_COUNT - right_3_complete_offset) / steps_complete_diff;
	let right_3_remaining = (STEP_COUNT - right_3_complete_offset) % steps_complete_diff;

	let down_1_complete = (STEP_COUNT - down_1_complete_offset) / steps_complete_diff;
	let down_1_remaining = (STEP_COUNT - down_1_complete_offset) % steps_complete_diff;
	let down_2_complete = (STEP_COUNT - down_2_complete_offset) / steps_complete_diff;
	let down_2_remaining = (STEP_COUNT - down_2_complete_offset) % steps_complete_diff;
	let down_3_complete = (STEP_COUNT - down_3_complete_offset) / steps_complete_diff;
	let down_3_remaining = (STEP_COUNT - down_3_complete_offset) % steps_complete_diff;

	let left_1_complete = (STEP_COUNT - left_1_complete_offset) / steps_complete_diff;
	let left_1_remaining = (STEP_COUNT - left_1_complete_offset) % steps_complete_diff;
	let left_2_complete = (STEP_COUNT - left_2_complete_offset) / steps_complete_diff;
	let left_2_remaining = (STEP_COUNT - left_2_complete_offset) % steps_complete_diff;
	let left_3_complete = (STEP_COUNT - left_3_complete_offset) / steps_complete_diff;
	let left_3_remaining = (STEP_COUNT - left_3_complete_offset) % steps_complete_diff;

	let left_up_complete = (STEP_COUNT - left_up_complete_offset) / steps_complete_diff;
	let left_up_remaining = (STEP_COUNT - left_up_complete_offset) % steps_complete_diff;
	let up_right_complete = (STEP_COUNT - up_right_complete_offset) / steps_complete_diff;
	let up_right_remaining = (STEP_COUNT - up_right_complete_offset) % steps_complete_diff;
	let right_down_complete = (STEP_COUNT - right_down_complete_offset) / steps_complete_diff;
	let right_down_remaining = (STEP_COUNT - right_down_complete_offset) % steps_complete_diff;
	let down_left_complete = (STEP_COUNT - down_left_complete_offset) / steps_complete_diff;
	let down_left_remaining = (STEP_COUNT - down_left_complete_offset) % steps_complete_diff;

	let up_partial = outer_progress.get(&MapCoordinate { x: 0, y: -3 }).unwrap();
	let up_right_partial = outer_progress.get(&MapCoordinate { x: 3, y: -3 }).unwrap();
	let right_partial = outer_progress.get(&MapCoordinate { x: 3, y: 0 }).unwrap();
	let right_down_partial = outer_progress.get(&MapCoordinate { x: 3, y: 3 }).unwrap();
	let down_partial = outer_progress.get(&MapCoordinate { x: 0, y: 3 }).unwrap();
	let down_left_partial = outer_progress.get(&MapCoordinate { x: -3, y: 3 }).unwrap();
	let left_partial = outer_progress.get(&MapCoordinate { x: -3, y: 0 }).unwrap();
	let left_up_partial = outer_progress.get(&MapCoordinate { x: -3, y: -3 }).unwrap();

	let mut total_plots = central_complete;
	
	let up_central_plots = single_direction_count(
		up_1_complete,
		up_1_remaining,
		left_up_partial,
		false,
		steps_complete_diff,
		even_plot_count,
		odd_plot_count,
	) + single_direction_count(
		up_2_complete,
		up_2_remaining,
		up_partial,
		true,
		steps_complete_diff,
		even_plot_count,
		odd_plot_count,
	) + single_direction_count(
		up_3_complete,
		up_3_remaining,
		up_right_partial,
		false,
		steps_complete_diff,
		even_plot_count,
		odd_plot_count,
	);
	total_plots += up_central_plots;
	
	let right_central_plots = single_direction_count(
		right_1_complete,
		right_1_remaining,
		up_right_partial,
		false,
		steps_complete_diff,
		even_plot_count,
		odd_plot_count,
	) + single_direction_count(
		right_2_complete,
		right_2_remaining,
		right_partial,
		true,
		steps_complete_diff,
		even_plot_count,
		odd_plot_count,
	) + single_direction_count(
		right_3_complete,
		right_3_remaining,
		right_down_partial,
		false,
		steps_complete_diff,
		even_plot_count,
		odd_plot_count,
	);
	total_plots += right_central_plots;
	
	let down_central_plots = single_direction_count(
		down_1_complete,
		down_1_remaining,
		right_down_partial,
		false,
		steps_complete_diff,
		even_plot_count,
		odd_plot_count,
	) + single_direction_count(
		down_2_complete,
		down_2_remaining,
		down_partial,
		true,
		steps_complete_diff,
		even_plot_count,
		odd_plot_count,
	) + single_direction_count(
		down_3_complete,
		down_3_remaining,
		down_left_partial,
		false,
		steps_complete_diff,
		even_plot_count,
		odd_plot_count,
	);
	total_plots += down_central_plots;
	
	let left_central_plots = single_direction_count(
		left_1_complete,
		left_1_remaining,
		down_left_partial,
		false,
		steps_complete_diff,
		even_plot_count,
		odd_plot_count,
	) + single_direction_count(
		left_2_complete,
		left_2_remaining,
		left_partial,
		true,
		steps_complete_diff,
		even_plot_count,
		odd_plot_count,
	) + single_direction_count(
		left_3_complete,
		left_3_remaining,
		left_up_partial,
		false,
		steps_complete_diff,
		even_plot_count,
		odd_plot_count,
	);
	total_plots += left_central_plots;
	
	let up_right_plots = corner_count(
		up_right_complete,
		up_right_remaining,
		up_right_partial,
		steps_complete_diff,
		even_plot_count,
		odd_plot_count,
	);
	let up_right_plots = up_right_plots
		+ single_direction_count(
			up_right_complete,
			up_right_remaining,
			up_right_partial,
			true,
			steps_complete_diff,
			even_plot_count,
			odd_plot_count,
		);
	total_plots += up_right_plots;
	
	let right_down_plots = corner_count(
		right_down_complete,
		right_down_remaining,
		right_down_partial,
		steps_complete_diff,
		even_plot_count,
		odd_plot_count,
	) + single_direction_count(
		right_down_complete,
		right_down_remaining,
		right_down_partial,
		true,
		steps_complete_diff,
		even_plot_count,
		odd_plot_count,
	);
	total_plots += right_down_plots;
	
	let down_left_plots = corner_count(
		down_left_complete,
		down_left_remaining,
		down_left_partial,
		steps_complete_diff,
		even_plot_count,
		odd_plot_count,
	) + single_direction_count(
		down_left_complete,
		down_left_remaining,
		down_left_partial,
		true,
		steps_complete_diff,
		even_plot_count,
		odd_plot_count,
	);
	total_plots += down_left_plots;
	
	let left_up_plots = corner_count(
		left_up_complete,
		left_up_remaining,
		left_up_partial,
		steps_complete_diff,
		even_plot_count,
		odd_plot_count,
	) + single_direction_count(
		left_up_complete,
		left_up_remaining,
		left_up_partial,
		true,
		steps_complete_diff,
		even_plot_count,
		odd_plot_count,
	);
	total_plots += left_up_plots;
	
	println!("{}", total_plots);

	Ok(())
}
