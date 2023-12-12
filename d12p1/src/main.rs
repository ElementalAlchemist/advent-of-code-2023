use std::error::Error;
use std::fs;

fn place_springs(remaining_springs: &str, remaining_arrangements: &[u32]) -> u32 {
	let Some(first_arrangement) = remaining_arrangements.first().copied() else {
		if remaining_springs.chars().any(|c| c == '#') {
			return 0;
		}
		return 1;
	};

	if (first_arrangement as usize) > remaining_springs.len() {
		return 0;
	}

	if (first_arrangement as usize) == remaining_springs.len() {
		if remaining_springs.chars().all(|c| c != '.') && remaining_arrangements.len() == 1 {
			return 1;
		}
		return 0;
	}

	let mut arrangements = 0;
	for start in 0..(remaining_springs.len() - first_arrangement as usize + 1) {
		if !remaining_springs
			.chars()
			.skip(start)
			.take(first_arrangement as usize)
			.any(|c| c == '.')
			&& remaining_springs.chars().nth(start + first_arrangement as usize) != Some('#')
		{
			let new_remaining_springs: String = remaining_springs
				.chars()
				.skip(start + first_arrangement as usize + 1)
				.collect();
			arrangements += place_springs(&new_remaining_springs, &remaining_arrangements[1..]);
		}

		if remaining_springs.chars().nth(start).unwrap() == '#' {
			// Moving past a guaranteed working one means we no longer start in the right spot
			break;
		}
	}

	arrangements
}

fn count_sets(spring_set: &str, arrangements: &[u32]) -> u32 {
	place_springs(spring_set, arrangements)
}

fn main() -> Result<(), Box<dyn Error>> {
	let spring_sets: Vec<(String, Vec<u32>)> = {
		let input = fs::read_to_string("input.txt")?;

		let mut spring_sets = Vec::new();
		for line in input.lines() {
			let mut line_parts = line.split(' ');
			let spring_set = line_parts.next().unwrap().to_string();
			let arrangements = line_parts.next().unwrap();
			assert!(line_parts.next().is_none());
			let arrangements: Vec<u32> = arrangements.split(',').map(|x| x.parse().unwrap()).collect();

			spring_sets.push((spring_set, arrangements));
		}

		spring_sets
	};

	let possible_arrangements: u32 = spring_sets
		.iter()
		.map(|(set, arrangements)| count_sets(set, arrangements))
		.sum();

	println!("{}", possible_arrangements);

	Ok(())
}
