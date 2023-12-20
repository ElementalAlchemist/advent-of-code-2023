use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::fs;

#[derive(Clone, Copy, Default)]
struct FlipFlopData {
	state: bool,
}

impl FlipFlopData {
	fn get_response_pulse(&mut self, pulse: bool) -> Option<bool> {
		if pulse {
			None
		} else {
			self.state = !self.state;
			Some(self.state)
		}
	}
}

#[derive(Clone, Default)]
struct ConjunctionData {
	last_input_values: HashMap<String, bool>,
}

impl ConjunctionData {
	fn get_response_pulse(&mut self, input: &str, pulse: bool) -> Option<bool> {
		self.last_input_values.insert(input.to_string(), pulse);
		if self.last_input_values.values().all(|s| *s) {
			Some(false)
		} else {
			Some(true)
		}
	}
}

#[derive(Clone)]
enum ModuleTypeData {
	FlipFlop(FlipFlopData),
	Conjunction(ConjunctionData),
	Broadcaster,
}

impl ModuleTypeData {
	fn get_response_pulse(&mut self, input: &str, pulse: bool) -> Option<bool> {
		match self {
			Self::FlipFlop(data) => data.get_response_pulse(pulse),
			Self::Conjunction(data) => data.get_response_pulse(input, pulse),
			Self::Broadcaster => Some(pulse),
		}
	}
}

#[derive(Clone)]
struct ModuleData {
	type_data: ModuleTypeData,
	outputs: Vec<String>,
}

struct PendingPulse {
	input: String,
	value: bool,
	destination: String,
}

fn main() -> Result<(), Box<dyn Error>> {
	let mut modules = {
		let input = fs::read_to_string("input.txt")?;

		let mut modules: HashMap<String, ModuleData> = HashMap::new();
		for line in input.lines() {
			let mut line_parts = line.split(" -> ");
			let name = line_parts.next().unwrap();
			let outputs = line_parts.next().unwrap();
			let outputs: Vec<String> = outputs.split(", ").map(|s| s.to_string()).collect();
			assert!(line_parts.next().is_none());

			if name == "broadcaster" {
				modules.insert(
					name.to_string(),
					ModuleData {
						type_data: ModuleTypeData::Broadcaster,
						outputs,
					},
				);
			} else {
				let mut name_chars = name.chars();
				let module_type = name_chars.next().unwrap();
				let name: String = name_chars.collect();

				let type_data = match module_type {
					'%' => ModuleTypeData::FlipFlop(FlipFlopData::default()),
					'&' => ModuleTypeData::Conjunction(ConjunctionData::default()),
					_ => panic!("Unknown module type: {}", module_type),
				};
				modules.insert(name, ModuleData { type_data, outputs });
			}
		}

		let mut input_map: HashMap<String, Vec<String>> = HashMap::new();
		for (name, module) in modules.iter() {
			for output in module.outputs.iter() {
				input_map.entry(output.clone()).or_default().push(name.clone());
			}
		}
		for (name, module) in modules.iter_mut() {
			if let ModuleTypeData::Conjunction(conjunction_data) = &mut module.type_data {
				if let Some(inputs) = input_map.get(name.as_str()) {
					for input in inputs.iter() {
						conjunction_data.last_input_values.insert(input.to_string(), false);
					}
				}
			}
		}

		modules
	};

	let mut output_high_pulses = 0;
	let mut output_low_pulses = 0;
	for _ in 0..1000 {
		let mut pulse_queue: VecDeque<PendingPulse> = VecDeque::new();
		output_low_pulses += 1;
		pulse_queue.push_back(PendingPulse {
			input: String::new(),
			value: false,
			destination: String::from("broadcaster"),
		});

		while let Some(pulse) = pulse_queue.pop_front() {
			let Some(module) = modules.get_mut(&pulse.destination) else { continue; };
			let next_pulse = module.type_data.get_response_pulse(&pulse.input, pulse.value);
			if let Some(next_pulse) = next_pulse {
				for output in module.outputs.iter() {
					if next_pulse {
						output_high_pulses += 1;
					} else {
						output_low_pulses += 1;
					}
					pulse_queue.push_back(PendingPulse {
						input: pulse.destination.clone(),
						value: next_pulse,
						destination: output.clone(),
					});
				}
			}
		}
	}

	let product = output_high_pulses * output_low_pulses;
	println!("{}", product);

	Ok(())
}
