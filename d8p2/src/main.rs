use num::integer::lcm;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

#[derive(Clone, Copy)]
enum Direction {
	Left,
	Right,
}

#[derive(Debug)]
struct StartNodeData {
	to_end: Vec<u64>,
	cycle_offset: u64,
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

	let current_nodes: Vec<&String> = map.keys().filter(|k| k.ends_with('A')).collect();
	let mut node_data: Vec<StartNodeData> = Vec::new();
	for node in current_nodes {
		let mut directions_iter = directions.iter().cycle();
		let mut steps_taken: u64 = 0;

		let mut first_z_node = None;
		let mut end_z_positions: Vec<u64> = Vec::new();

		let mut current_node = node;

		loop {
			steps_taken += 1;
			let next_direction = *directions_iter.next().unwrap();
			let (left_node, right_node) = map.get(current_node).unwrap();
			current_node = match next_direction {
				Direction::Left => left_node,
				Direction::Right => right_node,
			};
			if current_node.ends_with('Z') {
				match first_z_node {
					Some((node, _)) => {
						end_z_positions.push(steps_taken);
						if node == current_node {
							break;
						}
					}
					None => first_z_node = Some((current_node, steps_taken)),
				}
			}
		}

		let (_, cycle_offset) = first_z_node.unwrap();

		node_data.push(StartNodeData {
			to_end: end_z_positions,
			cycle_offset,
		});
	}

	let mut fewest_steps = 1;
	for node in node_data.iter() {
		println!("{:?}", node);
		let mut fewest_steps_for_node = 1;
		for position in node.to_end.iter() {
			fewest_steps_for_node = lcm(fewest_steps_for_node, *position - node.cycle_offset);
		}
		fewest_steps = lcm(fewest_steps, fewest_steps_for_node);
	}

	println!("{}", fewest_steps);

	Ok(())
}
