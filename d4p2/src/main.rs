use std::error::Error;
use std::fs;

struct Card {
	winning_numbers: Vec<u32>,
	play_numbers: Vec<u32>,
}

impl Card {
	fn matches(&self) -> u32 {
		let mut matches = 0;
		for number in self.play_numbers.iter() {
			if self.winning_numbers.iter().any(|num| *num == *number) {
				matches += 1;
			}
		}
		matches
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

	let mut process_counts: Vec<u32> = cards.iter().map(|_| 1).collect();
	let mut total_cards = 0;

	for (card_index, card) in cards.iter().enumerate() {
		let num_to_process = process_counts[card_index];
		total_cards += num_to_process;
		let matches = card.matches();
		for count in process_counts.iter_mut().skip(card_index + 1).take(matches as usize) {
			*count += num_to_process;
		}
	}

	println!("{}", total_cards);

	Ok(())
}
