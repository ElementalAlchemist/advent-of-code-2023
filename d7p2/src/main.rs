use std::cmp::Ordering;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

const CARD_ORDER: [char; 13] = ['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'];

fn cmp_cards(lhs: char, rhs: char) -> Ordering {
	if lhs == rhs {
		return Ordering::Equal;
	}

	let lhs = CARD_ORDER
		.iter()
		.enumerate()
		.find(|(_, card)| **card == lhs)
		.map(|(index, _)| index)
		.unwrap();
	let rhs = CARD_ORDER
		.iter()
		.enumerate()
		.find(|(_, card)| **card == rhs)
		.map(|(index, _)| index)
		.unwrap();

	rhs.cmp(&lhs)
}

fn hand_type(hand_counts: &HashMap<char, u8>) -> u8 {
	let mut counts_totals: Vec<u8> = hand_counts.values().copied().collect();
	counts_totals.sort_unstable();

	let joker_count = hand_counts.get(&'J').copied();
	if let Some(joker_count) = joker_count {
		match &counts_totals[..] {
			[5] => 7,
			[1, 4] => 7,
			[2, 3] => 7,
			[1, 1, 3] => 6,
			[1, 2, 2] => {
				if joker_count == 2 {
					6
				} else {
					5
				}
			}
			[1, 1, 1, 2] => 4,
			[1, 1, 1, 1, 1] => 2,
			_ => unreachable!(),
		}
	} else {
		match &counts_totals[..] {
			[5] => 7,
			[1, 4] => 6,
			[2, 3] => 5,
			[1, 1, 3] => 4,
			[1, 2, 2] => 3,
			[1, 1, 1, 2] => 2,
			[1, 1, 1, 1, 1] => 1,
			_ => unreachable!(),
		}
	}
}

#[derive(Eq, PartialEq)]
struct Hand {
	cards: [char; 5],
	bid: usize,
}

impl Ord for Hand {
	fn cmp(&self, other: &Self) -> Ordering {
		let mut lhs_counts: HashMap<char, u8> = HashMap::new();
		let mut rhs_counts: HashMap<char, u8> = HashMap::new();

		for card in self.cards.iter() {
			*lhs_counts.entry(*card).or_default() += 1;
		}
		for card in other.cards.iter() {
			*rhs_counts.entry(*card).or_default() += 1;
		}

		// Types are assigned a numeric value from 5 of a Kind (7) down to High Card (1)
		let lhs_type = hand_type(&lhs_counts);
		let rhs_type = hand_type(&rhs_counts);

		lhs_type.cmp(&rhs_type).then_with(|| {
			for (lhs, rhs) in self.cards.iter().zip(&other.cards) {
				match cmp_cards(*lhs, *rhs) {
					Ordering::Less => return Ordering::Less,
					Ordering::Greater => return Ordering::Greater,
					_ => (),
				}
			}
			Ordering::Equal
		})
	}
}

impl PartialOrd for Hand {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let mut hands = {
		let input = fs::read_to_string("input.txt")?;

		let mut hands: Vec<Hand> = Vec::new();
		for line in input.lines() {
			let mut line_parts = line.split(' ');
			let hand = line_parts.next().unwrap();
			let bid = line_parts.next().unwrap();
			assert!(line_parts.next().is_none());

			let bid: usize = bid.parse()?;
			let mut hand_chars = hand.chars();
			let hand = [
				hand_chars.next().unwrap(),
				hand_chars.next().unwrap(),
				hand_chars.next().unwrap(),
				hand_chars.next().unwrap(),
				hand_chars.next().unwrap(),
			];
			hands.push(Hand { cards: hand, bid });
		}

		hands
	};

	hands.sort_unstable();

	let total: usize = hands
		.into_iter()
		.enumerate()
		.map(|(index, hand)| (index + 1) * hand.bid)
		.sum();

	println!("{}", total);

	Ok(())
}
