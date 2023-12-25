use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
	let connected_components = {
		let input = fs::read_to_string("input.txt")?;

		let mut connected_components: HashMap<String, HashSet<String>> = HashMap::new();
		for line in input.lines() {
			let mut line_parts = line.split(": ");
			let first_wire = line_parts.next().unwrap();
			let connecting_wires = line_parts.next().unwrap();
			assert!(line_parts.next().is_none());

			for connecting_wire in connecting_wires.split(' ') {
				connected_components
					.entry(first_wire.to_string())
					.or_default()
					.insert(connecting_wire.to_string());
			}
		}
		connected_components
	};

	let mut bidirectional_connected_components: HashMap<String, HashSet<String>> = HashMap::new();
	for (component, connecting_components) in connected_components.iter() {
		for connecting_wire in connecting_components.iter() {
			bidirectional_connected_components
				.entry(component.clone())
				.or_default()
				.insert(connecting_wire.clone());
			bidirectional_connected_components
				.entry(connecting_wire.clone())
				.or_default()
				.insert(component.clone());
		}
	}

	let component_pairs: Vec<(String, String)> = connected_components
		.iter()
		.flat_map(|(component, connections)| {
			connections
				.iter()
				.map(|connection| (component.clone(), connection.clone()))
		})
		.collect();
	let mut uncut_edges: HashMap<(String, String), usize> = HashMap::new();
	let mut failed_cuts: HashSet<[String; 6]> = HashSet::new();
	let mut rng = thread_rng();
	loop {
		let mut merged_components: HashMap<String, String> = HashMap::new();
		let mut pair_order = component_pairs.clone();
		pair_order.shuffle(&mut rng);

		let mut merged_graph = bidirectional_connected_components.clone();

		while merged_graph.len() > 2 {
			let (component, other_component) = pair_order.pop().unwrap();

			let component = merged_components.get(&component).cloned().unwrap_or(component);
			let other_component = merged_components
				.get(&other_component)
				.cloned()
				.unwrap_or(other_component);

			if component == other_component {
				continue;
			}

			let merged_component_name = format!("{}{}", component, other_component);
			for merged_component in merged_components.values_mut() {
				if *merged_component == component || *merged_component == other_component {
					*merged_component = merged_component_name.clone();
				}
			}
			let mut merged_connections: HashSet<String> = HashSet::new();

			let mut connections = merged_graph.remove(&component).unwrap();
			connections.remove(&other_component);
			let mut other_connections = merged_graph.remove(&other_component).unwrap();
			other_connections.remove(&component);
			for connection in connections {
				let connection_connections = merged_graph.get_mut(&connection).unwrap();
				connection_connections.remove(&component);
				connection_connections.insert(merged_component_name.clone());
				merged_connections.insert(connection);
			}
			for connection in other_connections {
				let connection_connections = merged_graph.get_mut(&connection).unwrap();
				connection_connections.remove(&other_component);
				connection_connections.insert(merged_component_name.clone());
				merged_connections.insert(connection);
			}

			merged_components.insert(component, merged_component_name.clone());
			merged_components.insert(other_component, merged_component_name.clone());
			merged_graph.insert(merged_component_name, merged_connections);
		}

		for (remaining_component_1, remaining_component_2) in pair_order.iter() {
			if merged_components.get(remaining_component_1) != merged_components.get(remaining_component_2) {
				*uncut_edges
					.entry((remaining_component_1.clone(), remaining_component_2.clone()))
					.or_default() += 1;
			}
		}

		let mut pair_removed_buckets: Vec<Vec<(String, String)>> = Vec::new();
		for ((edge_vertex_1, edge_vertex_2), times_used) in uncut_edges.iter() {
			let edge = (edge_vertex_1.clone(), edge_vertex_2.clone());
			while pair_removed_buckets.len() <= *times_used {
				pair_removed_buckets.push(Vec::new());
			}
			pair_removed_buckets[*times_used].push(edge);
		}
		if pair_removed_buckets.len() < 3 {
			continue;
		}

		let mut most_uncut_pairs = pair_removed_buckets.iter().flatten().rev();

		let (component_1_1, component_1_2) = most_uncut_pairs.next().unwrap().clone();
		let (component_2_1, component_2_2) = most_uncut_pairs.next().unwrap().clone();
		let (component_3_1, component_3_2) = most_uncut_pairs.next().unwrap().clone();

		let component_group = [
			component_1_1.clone(),
			component_1_2.clone(),
			component_2_1.clone(),
			component_2_2.clone(),
			component_3_1.clone(),
			component_3_2.clone(),
		];
		if failed_cuts.contains(&component_group) {
			continue;
		}

		println!(
			"({}, {}) ({}, {}) ({}, {})",
			component_1_1, component_1_2, component_2_1, component_2_2, component_3_1, component_3_2
		);

		let mut bidirectional_connected_components = bidirectional_connected_components.clone();

		bidirectional_connected_components
			.get_mut(&component_1_1)
			.unwrap()
			.remove(&component_1_2);
		bidirectional_connected_components
			.get_mut(&component_1_2)
			.unwrap()
			.remove(&component_1_1);
		bidirectional_connected_components
			.get_mut(&component_2_1)
			.unwrap()
			.remove(&component_2_2);
		bidirectional_connected_components
			.get_mut(&component_2_2)
			.unwrap()
			.remove(&component_2_1);
		bidirectional_connected_components
			.get_mut(&component_3_1)
			.unwrap()
			.remove(&component_3_2);
		bidirectional_connected_components
			.get_mut(&component_3_2)
			.unwrap()
			.remove(&component_3_1);

		let mut first_group: HashSet<String> = HashSet::new();
		first_group.insert(component_1_1.clone());
		let mut current_positions = vec![component_1_1];
		while !current_positions.is_empty() {
			let mut next_positions = Vec::new();
			for position in current_positions.iter() {
				for next_position in bidirectional_connected_components.get(position).unwrap().iter() {
					if !first_group.contains(next_position) {
						first_group.insert(next_position.clone());
						next_positions.push(next_position.clone());
					}
				}
			}
			current_positions = next_positions;
		}

		let mut second_group: HashSet<String> = HashSet::new();
		for component in [
			component_1_2,
			component_2_1,
			component_2_2,
			component_3_1,
			component_3_2,
		] {
			if first_group.contains(&component) || second_group.contains(&component) {
				continue;
			}

			current_positions = vec![component.clone()];
			if !second_group.is_empty() {
				println!("More than two groups found!");
				break;
			}
			second_group.insert(component);

			while !current_positions.is_empty() {
				let mut next_positions = Vec::new();
				for position in current_positions.iter() {
					for next_position in bidirectional_connected_components.get(position).unwrap().iter() {
						if !second_group.contains(next_position) {
							next_positions.push(next_position.clone());
							second_group.insert(next_position.clone());
						}
					}
				}
				current_positions = next_positions;
			}
		}

		if !first_group.is_empty() && !second_group.is_empty() {
			let first_group_size = first_group.len();
			let second_group_size = second_group.len();
			let answer = first_group_size * second_group_size;
			println!("{} = {} * {}", answer, first_group_size, second_group_size);
			break;
		}

		failed_cuts.insert(component_group);
	}

	Ok(())
}
