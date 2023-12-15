use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

fn label_hash(label: &str) -> usize {
	let mut hash = 0;
	for c in label.chars() {
		let c_value = c as usize;
		hash += c_value;
		hash *= 17;
		hash %= 256;
	}
	hash
}

struct Lens {
	label: String,
	focal_length: u32,
}

#[derive(Default)]
struct LensBox {
	ordered_lenses: Vec<Lens>,
	lens_map: HashMap<String, usize>,
}

impl LensBox {
	fn add_lens(&mut self, lens: Lens) {
		match self.lens_map.entry(lens.label.clone()) {
			Entry::Occupied(entry) => {
				let existing_lens_index = entry.get();
				self.ordered_lenses[*existing_lens_index].focal_length = lens.focal_length;
			}
			Entry::Vacant(entry) => {
				let new_index = self.ordered_lenses.len();
				self.ordered_lenses.push(lens);
				entry.insert(new_index);
			}
		}
	}

	fn remove_lens(&mut self, label: &str) {
		let index = self.lens_map.remove(label);
		if let Some(index) = index {
			self.ordered_lenses.remove(index);
			for lens_index in self.lens_map.values_mut() {
				if *lens_index > index {
					*lens_index -= 1;
				}
			}
		}
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let steps: Vec<String> = {
		let input = fs::read_to_string("input.txt")?;

		let mut steps = Vec::new();
		for step in input.split(',') {
			steps.push(step.chars().filter(|c| *c != '\n').collect());
		}

		steps
	};

	let mut boxes: Vec<LensBox> = Vec::new();

	for step in steps.iter() {
		if let Some(label) = step.strip_suffix('-') {
			let box_index = label_hash(label);
			if box_index < boxes.len() {
				boxes[box_index].remove_lens(label);
			}
		} else {
			let mut parts = step.split('=');
			let label = parts.next().unwrap();
			let focus_length: u32 = parts.next().unwrap().parse()?;
			assert!(parts.next().is_none());

			let box_index = label_hash(label);
			while box_index >= boxes.len() {
				boxes.push(LensBox::default());
			}

			let lens = Lens {
				label: label.to_string(),
				focal_length: focus_length,
			};
			boxes[box_index].add_lens(lens);
		}
	}

	let focusing_power: usize = boxes
		.iter()
		.enumerate()
		.map(|(box_index, lenses)| {
			let box_number = box_index + 1;
			lenses
				.ordered_lenses
				.iter()
				.enumerate()
				.map(|(lens_index, lens_data)| {
					let lens_number = lens_index + 1;
					let focal_length = lens_data.focal_length as usize;
					box_number * lens_number * focal_length
				})
				.sum::<usize>()
		})
		.sum();

	println!("{}", focusing_power);

	Ok(())
}
