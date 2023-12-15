use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
	let steps: Vec<String> = {
		let input = fs::read_to_string("input.txt")?;

		let mut steps = Vec::new();
		for step in input.split(',') {
			steps.push(step.chars().filter(|c| *c != '\n').collect());
		}

		steps
	};

	let mut hash_sum: u32 = 0;

	for step in steps.iter() {
		let mut hash: u32 = 0;
		for c in step.chars() {
			let c_value = c as u32;
			hash += c_value;
			hash *= 17;
			hash %= 256;
		}
		hash_sum += hash;
	}

	println!("{}", hash_sum);

	Ok(())
}
