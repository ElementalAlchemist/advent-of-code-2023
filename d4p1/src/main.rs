use std::error::Error;
use std::fs;

struct Card {
	winning_numbers: Vec<u32>,
	play_numbers: Vec<u32>,
}

impl Card {
	fn score(&self) -> u32 {
		let mut score = 0;
		for number in self.play_numbers.iter() {
			if self.winning_numbers.iter().any(|num| *num == *number) {
				if score == 0 {
					score = 1;
				} else {
					score *= 2;
				}
			}
		}
		score
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let cards = {
		let input = fs::read_to_string("input.txt")?;

		let mut cards: Vec<Card> = Vec::new();
		for line in input.lines() {
			let numbers = line.split(": ").nth(1).unwrap().to_string();
			let mut parts = numbers.split(" | ");
			let winning_numbers = parts.next().unwrap();
			let play_numbers = parts.next().unwrap();
			assert!(parts.next().is_none());

			let winning_numbers: Vec<u32> = winning_numbers
				.split(' ')
				.filter(|s| !s.is_empty())
				.map(|s| s.parse().unwrap())
				.collect();
			let play_numbers: Vec<u32> = play_numbers
				.split(' ')
				.filter(|s| !s.is_empty())
				.map(|s| s.parse().unwrap())
				.collect();

			cards.push(Card {
				winning_numbers,
				play_numbers,
			});
		}
		cards
	};

	let mut total_score = 0;
	for card in cards.iter() {
		total_score += card.score();
	}

	println!("{}", total_score);

	Ok(())
}
