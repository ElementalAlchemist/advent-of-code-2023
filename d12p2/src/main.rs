use std::collections::HashMap;
use std::error::Error;
use std::fs;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum SpringStatus {
	Operational,
	Damaged,
	Unknown,
}

impl SpringStatus {
	fn char_rep(&self) -> char {
		match self {
			Self::Operational => '#',
			Self::Damaged => '.',
			Self::Unknown => '?',
		}
	}
}

fn place_springs(
	remaining_springs: &[SpringStatus],
	remaining_arrangements: &[u64],
	cache: &mut HashMap<(Vec<SpringStatus>, Vec<u64>), u64>,
) -> u64 {
	let Some(first_arrangement) = remaining_arrangements.first().copied() else {
		if remaining_springs
			.iter()
			.any(|spring| *spring == SpringStatus::Operational)
		{
			return 0;
		}
		return 1;
	};

	let remaining_arrangements_total: usize =
		(remaining_arrangements.iter().copied().sum::<u64>() as usize) + remaining_arrangements.len() - 1;

	if remaining_arrangements_total > remaining_springs.len() {
		return 0;
	}

	if (first_arrangement as usize) == remaining_springs.len() {
		if remaining_springs.iter().all(|spring| *spring != SpringStatus::Damaged) && remaining_arrangements.len() == 1
		{
			return 1;
		}
		return 0;
	}

	let mut arrangements = 0;
	for start in 0..(remaining_springs.len() - remaining_arrangements_total + 1) {
		if !remaining_springs
			.iter()
			.skip(start)
			.take(first_arrangement as usize)
			.any(|spring| *spring == SpringStatus::Damaged)
			&& remaining_springs.get(start + first_arrangement as usize).copied() != Some(SpringStatus::Operational)
		{
			let next_start_index = start + first_arrangement as usize + 1;
			if next_start_index >= remaining_springs.len() {
				if remaining_arrangements.len() == 1 {
					arrangements += 1;
				}
			} else {
				let cache_key = (
					remaining_springs[next_start_index..].to_vec(),
					remaining_arrangements[1..].to_vec(),
				);
				if let Some(cache_result) = cache.get(&cache_key) {
					arrangements += *cache_result;
				} else {
					let sub_arrangements = place_springs(
						&remaining_springs[next_start_index..],
						&remaining_arrangements[1..],
						cache,
					);
					cache.insert(cache_key, sub_arrangements);
					arrangements += sub_arrangements;
				}
			}
		}

		if *remaining_springs.get(start).unwrap() == SpringStatus::Operational {
			// Moving past a guaranteed working one means we no longer start in the right spot
			break;
		}
	}

	arrangements
}

fn count_sets(spring_set: &[SpringStatus], arrangements: &[u64]) -> u64 {
	let mut cache: HashMap<(Vec<SpringStatus>, Vec<u64>), u64> = HashMap::new();
	let value = place_springs(spring_set, arrangements, &mut cache);
	let spring_set_output: String = spring_set.iter().map(|spring| spring.char_rep()).collect();
	println!("{}: {}", value, spring_set_output);
	value
}

fn main() -> Result<(), Box<dyn Error>> {
	let spring_sets: Vec<(Vec<SpringStatus>, Vec<u64>)> = {
		let input = fs::read_to_string("input.txt")?;

		let mut spring_sets = Vec::new();
		for line in input.lines() {
			let mut line_parts = line.split(' ');
			let spring_set = line_parts.next().unwrap().to_string();
			let arrangements = line_parts.next().unwrap();
			assert!(line_parts.next().is_none());
			let arrangements: Vec<u64> = arrangements.split(',').map(|x| x.parse().unwrap()).collect();

			let mut full_spring_set: String = String::new();
			let mut full_arrangements: Vec<u64> = Vec::new();
			for _ in 0..5 {
				full_spring_set.push_str(&spring_set);
				full_spring_set.push('?');
				for arrangement in arrangements.iter().copied() {
					full_arrangements.push(arrangement);
				}
			}
			full_spring_set.pop();
			let full_spring_set: Vec<SpringStatus> = full_spring_set
				.chars()
				.map(|c| match c {
					'#' => SpringStatus::Operational,
					'.' => SpringStatus::Damaged,
					'?' => SpringStatus::Unknown,
					_ => unreachable!(),
				})
				.collect();
			spring_sets.push((full_spring_set, full_arrangements));
		}

		spring_sets
	};

	let possible_arrangements: u64 = spring_sets
		.iter()
		.map(|(set, arrangements)| count_sets(set, arrangements))
		.sum();

	println!("{}", possible_arrangements);

	Ok(())
}
