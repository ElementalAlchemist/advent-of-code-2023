use std::error::Error;
use std::fs;

fn handle_digit(first_digit: &mut Option<char>, last_digit: &mut Option<char>, digit: char) {
	if first_digit.is_none() {
		*first_digit = Some(digit);
	}
	*last_digit = Some(digit);
}

fn main() -> Result<(), Box<dyn Error>> {
	let numbers: Vec<u32> = {
		let input = fs::read_to_string("input.txt")?;
		let mut digits: Vec<u32> = Vec::new();
		for line in input.split('\n') {
			let mut first_digit: Option<char> = None;
			let mut last_digit: Option<char> = None;
			let mut line_so_far: String = String::new();
			for c in line.chars() {
				if c.is_ascii_digit() {
					handle_digit(&mut first_digit, &mut last_digit, c);
					line_so_far.clear();
				} else {
					line_so_far.push(c);
					let line_len = line_so_far.len();
					if line_len >= 3 && &line_so_far[(line_len - 3)..line_len] == "one" {
						handle_digit(&mut first_digit, &mut last_digit, '1');
					} else if line_len >= 3 && &line_so_far[(line_len - 3)..line_len] == "two" {
						handle_digit(&mut first_digit, &mut last_digit, '2');
					} else if line_len >= 5 && &line_so_far[(line_len - 5)..line_len] == "three" {
						handle_digit(&mut first_digit, &mut last_digit, '3');
					} else if line_len >= 4 && &line_so_far[(line_len - 4)..line_len] == "four" {
						handle_digit(&mut first_digit, &mut last_digit, '4');
					} else if line_len >= 4 && &line_so_far[(line_len - 4)..line_len] == "five" {
						handle_digit(&mut first_digit, &mut last_digit, '5');
					} else if line_len >= 3 && &line_so_far[(line_len - 3)..line_len] == "six" {
						handle_digit(&mut first_digit, &mut last_digit, '6');
					} else if line_len >= 5 && &line_so_far[(line_len - 5)..line_len] == "seven" {
						handle_digit(&mut first_digit, &mut last_digit, '7');
					} else if line_len >= 5 && &line_so_far[(line_len - 5)..line_len] == "eight" {
						handle_digit(&mut first_digit, &mut last_digit, '8');
					} else if line_len >= 4 && &line_so_far[(line_len - 4)..line_len] == "nine" {
						handle_digit(&mut first_digit, &mut last_digit, '9');
					}
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
