use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
	let numbers: Vec<u32> = {
		let input = fs::read_to_string("input.txt")?;
		let mut digits: Vec<u32> = Vec::new();
		for line in input.split('\n') {
			let mut first_digit: Option<char> = None;
			let mut last_digit: Option<char> = None;
			for c in line.chars() {
				if c.is_ascii_digit() {
					if first_digit.is_none() {
						first_digit = Some(c);
					}
					last_digit = Some(c);
				}
			}
			let mut line_digits = String::new();
			line_digits.push(first_digit.unwrap());
			line_digits.push(last_digit.unwrap());
			digits.push(line_digits.parse().unwrap());
		}
		digits
	};

	let value: u32 = numbers.iter().sum();
	println!("{}", value);

	Ok(())
}
