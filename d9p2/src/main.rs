use std::error::Error;
use std::fs;

fn prepend_sequence(sequence: &[i32]) -> i32 {
	let mut differences: Vec<i32> = Vec::new();

	for value_pair in sequence.windows(2) {
		differences.push(value_pair[1] - value_pair[0]);
	}

	if differences.iter().all(|v| *v == 0) {
		*sequence.first().unwrap()
	} else {
		*sequence.first().unwrap() - prepend_sequence(&differences)
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let sequences = {
		let input = fs::read_to_string("input.txt")?;

		let mut sequences: Vec<Vec<i32>> = Vec::new();
		for line in input.lines() {
			let mut sequence: Vec<i32> = Vec::new();
			for num in line.split(' ') {
				sequence.push(num.parse()?);
			}
			sequences.push(sequence);
		}

		sequences
	};

	let mut prev_values_sum = 0;
	for sequence in sequences.iter() {
		prev_values_sum += prepend_sequence(sequence);
	}

	println!("{}", prev_values_sum);

	Ok(())
}
