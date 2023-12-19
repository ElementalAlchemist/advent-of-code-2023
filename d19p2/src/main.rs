use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::ops::RangeInclusive;

#[derive(Clone, Copy)]
enum ConditionVariable {
	X,
	M,
	A,
	S,
}

#[derive(Clone, Copy)]
enum ConditionOperator {
	LessThan,
	GreaterThan,
}

#[derive(Clone)]
enum RuleResult {
	Accept,
	Reject,
	Evaluate(String),
}

#[derive(Clone)]
struct RuleCondition {
	variable: ConditionVariable,
	operator: ConditionOperator,
	compare_value: u64,
	result: RuleResult,
}

struct Rule {
	conditions: Vec<RuleCondition>,
	default: RuleResult,
}

struct PartRange {
	x: RangeInclusive<u64>,
	m: RangeInclusive<u64>,
	a: RangeInclusive<u64>,
	s: RangeInclusive<u64>,
}

fn main() -> Result<(), Box<dyn Error>> {
	let rules = {
		let input = fs::read_to_string("input.txt")?;

		let mut rules: HashMap<String, Rule> = HashMap::new();
		for line in input.lines() {
			if line.is_empty() {
				break;
			}

			let mut line_parts = line.split('{');
			let rule_name = line_parts.next().unwrap();
			let rule_data = line_parts.next().unwrap();
			assert!(line_parts.next().is_none());

			let rule_data = rule_data.strip_suffix('}').unwrap();
			let mut rule_conditions_data: Vec<&str> = rule_data.split(',').collect();
			let default_result = rule_conditions_data.pop().unwrap();
			let mut rule_conditions: Vec<RuleCondition> = Vec::new();
			for condition_data in rule_conditions_data {
				let mut condition_parts = condition_data.split(':');
				let condition = condition_parts.next().unwrap();
				let result = condition_parts.next().unwrap();
				assert!(condition_parts.next().is_none());

				let mut condition_chars = condition.chars();
				let variable = condition_chars.next().unwrap();
				let operator = condition_chars.next().unwrap();
				let value: String = condition_chars.collect();

				let variable = match variable {
					'x' => ConditionVariable::X,
					'm' => ConditionVariable::M,
					'a' => ConditionVariable::A,
					's' => ConditionVariable::S,
					_ => panic!("Unexpected variable: {}", variable),
				};
				let operator = match operator {
					'<' => ConditionOperator::LessThan,
					'>' => ConditionOperator::GreaterThan,
					_ => panic!("Unexpected operator: {}", operator),
				};
				let compare_value: u64 = value.parse()?;
				let result = match result {
					"A" => RuleResult::Accept,
					"R" => RuleResult::Reject,
					_ => RuleResult::Evaluate(String::from(result)),
				};

				rule_conditions.push(RuleCondition {
					variable,
					operator,
					compare_value,
					result,
				});
			}

			let default = match default_result {
				"A" => RuleResult::Accept,
				"R" => RuleResult::Reject,
				_ => RuleResult::Evaluate(String::from(default_result)),
			};
			rules.insert(
				String::from(rule_name),
				Rule {
					conditions: rule_conditions,
					default,
				},
			);
		}

		rules
	};

	let initial_part_range = PartRange {
		x: 1..=4000,
		m: 1..=4000,
		a: 1..=4000,
		s: 1..=4000,
	};
	let mut partitions: Vec<(PartRange, &str)> = vec![(initial_part_range, "in")];
	let mut accepted_part_ranges: Vec<PartRange> = Vec::new();

	while !partitions.is_empty() {
		let mut next_partitions = Vec::new();
		'partition: for (mut partition, rule_name) in partitions {
			let part_rule = rules.get(rule_name).unwrap();
			for condition in part_rule.conditions.iter() {
				match condition.variable {
					ConditionVariable::X => {
						let (current_range, new_range) = match condition.operator {
							ConditionOperator::LessThan => (
								condition.compare_value..=*partition.x.end(),
								*partition.x.start()..=(condition.compare_value - 1),
							),
							ConditionOperator::GreaterThan => (
								*partition.x.start()..=condition.compare_value,
								(condition.compare_value + 1)..=*partition.x.end(),
							),
						};
						if !new_range.is_empty() {
							let new_part_range = PartRange {
								x: new_range,
								m: partition.m.clone(),
								a: partition.a.clone(),
								s: partition.s.clone(),
							};
							match &condition.result {
								RuleResult::Accept => accepted_part_ranges.push(new_part_range),
								RuleResult::Reject => (),
								RuleResult::Evaluate(next_rule) => {
									next_partitions.push((new_part_range, next_rule.as_str()))
								}
							}
						}
						if current_range.is_empty() {
							continue 'partition;
						}
						partition.x = current_range;
					}
					ConditionVariable::M => {
						let (current_range, new_range) = match condition.operator {
							ConditionOperator::LessThan => (
								condition.compare_value..=*partition.m.end(),
								*partition.m.start()..=(condition.compare_value - 1),
							),
							ConditionOperator::GreaterThan => (
								*partition.m.start()..=condition.compare_value,
								(condition.compare_value + 1)..=*partition.m.end(),
							),
						};
						if !new_range.is_empty() {
							let new_part_range = PartRange {
								x: partition.x.clone(),
								m: new_range,
								a: partition.a.clone(),
								s: partition.s.clone(),
							};
							match &condition.result {
								RuleResult::Accept => accepted_part_ranges.push(new_part_range),
								RuleResult::Reject => (),
								RuleResult::Evaluate(next_rule) => {
									next_partitions.push((new_part_range, next_rule.as_str()))
								}
							}
						}
						if current_range.is_empty() {
							continue 'partition;
						}
						partition.m = current_range;
					}
					ConditionVariable::A => {
						let (current_range, new_range) = match condition.operator {
							ConditionOperator::LessThan => (
								condition.compare_value..=*partition.a.end(),
								*partition.a.start()..=(condition.compare_value - 1),
							),
							ConditionOperator::GreaterThan => (
								*partition.a.start()..=condition.compare_value,
								(condition.compare_value + 1)..=*partition.a.end(),
							),
						};
						if !new_range.is_empty() {
							let new_part_range = PartRange {
								x: partition.x.clone(),
								m: partition.m.clone(),
								a: new_range,
								s: partition.s.clone(),
							};
							match &condition.result {
								RuleResult::Accept => accepted_part_ranges.push(new_part_range),
								RuleResult::Reject => (),
								RuleResult::Evaluate(next_rule) => {
									next_partitions.push((new_part_range, next_rule.as_str()))
								}
							}
						}
						if current_range.is_empty() {
							continue 'partition;
						}
						partition.a = current_range;
					}
					ConditionVariable::S => {
						let (current_range, new_range) = match condition.operator {
							ConditionOperator::LessThan => (
								condition.compare_value..=*partition.s.end(),
								*partition.s.start()..=(condition.compare_value - 1),
							),
							ConditionOperator::GreaterThan => (
								*partition.s.start()..=condition.compare_value,
								(condition.compare_value + 1)..=*partition.s.end(),
							),
						};
						if !new_range.is_empty() {
							let new_part_range = PartRange {
								x: partition.x.clone(),
								m: partition.m.clone(),
								a: partition.a.clone(),
								s: new_range,
							};
							match &condition.result {
								RuleResult::Accept => accepted_part_ranges.push(new_part_range),
								RuleResult::Reject => (),
								RuleResult::Evaluate(next_rule) => {
									next_partitions.push((new_part_range, next_rule.as_str()))
								}
							}
						}
						if current_range.is_empty() {
							continue 'partition;
						}
						partition.s = current_range;
					}
				}
			}
			match &part_rule.default {
				RuleResult::Accept => accepted_part_ranges.push(partition),
				RuleResult::Reject => (),
				RuleResult::Evaluate(next_rule) => next_partitions.push((partition, next_rule.as_str())),
			}
		}
		partitions = next_partitions;
	}

	let mut accepted_part_count = 0;
	for accept_range in accepted_part_ranges.iter() {
		let valid_x = *accept_range.x.end() - *accept_range.x.start() + 1;
		let valid_m = *accept_range.m.end() - *accept_range.m.start() + 1;
		let valid_a = *accept_range.a.end() - *accept_range.a.start() + 1;
		let valid_s = *accept_range.s.end() - *accept_range.s.start() + 1;

		accepted_part_count += valid_x * valid_m * valid_a * valid_s;
	}

	println!("{}", accepted_part_count);

	Ok(())
}
