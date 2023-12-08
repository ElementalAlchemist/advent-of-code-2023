use std::collections::HashMap;
use std::error::Error;
use std::fs;

#[derive(Clone, Copy)]
enum Direction {
	Left,
	Right,
}

fn main() -> Result<(), Box<dyn Error>> {
	let (directions, map) = {
		let input = fs::read_to_string("input.txt")?;

		let mut lines = input.lines();
		let directions = lines.next().unwrap();
		assert_eq!(lines.next().unwrap(), "");
		let directions: Vec<Direction> = directions
			.chars()
			.map(|c| match c {
				'L' => Direction::Left,
				'R' => Direction::Right,
				_ => panic!("Unexpected character '{}'", c),
			})
			.collect();

		let mut map: HashMap<String, (String, String)> = HashMap::new();
		for line in lines {
			let mut line_parts = line.split(" = ");
			let start_node = line_parts.next().unwrap();
			let dest_nodes = line_parts.next().unwrap();
			assert!(line_parts.next().is_none());
			let dest_nodes = dest_nodes.strip_prefix('(').unwrap();
			let dest_nodes = dest_nodes.strip_suffix(')').unwrap();
			let mut dest_nodes_iter = dest_nodes.split(", ");
			let dest_nodes = (
				dest_nodes_iter.next().unwrap().to_string(),
				dest_nodes_iter.next().unwrap().to_string(),
			);
			assert!(dest_nodes_iter.next().is_none());
			map.insert(start_node.to_string(), dest_nodes);
		}

		(directions, map)
	};

	let mut current_node = "AAA";
	let mut steps_taken = 0;
	let mut directions_iter = directions.iter().cycle();
	while current_node != "ZZZ" {
		let next_direction = *directions_iter.next().unwrap();
		let (left_node, right_node) = map.get(current_node).unwrap();
		let next_node = match next_direction {
			Direction::Left => left_node,
			Direction::Right => right_node,
		};
		current_node = next_node;
		steps_taken += 1;
	}

	println!("{}", steps_taken);

	Ok(())
}
