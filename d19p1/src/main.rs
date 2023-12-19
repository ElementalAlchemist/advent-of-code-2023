use std::collections::HashMap;
use std::error::Error;
use std::fs;

#[derive(Clone)]
struct Part {
	x: u32,
	m: u32,
	a: u32,
	s: u32,
}

impl Part {
	fn get_single_variable(&self, variable: ConditionVariable) -> u32 {
		match variable {
			ConditionVariable::X => self.x,
			ConditionVariable::M => self.m,
			ConditionVariable::A => self.a,
			ConditionVariable::S => self.s,
		}
	}

	fn value_sum(&self) -> u32 {
		self.x + self.m + self.a + self.s
	}
}

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
	compare_value: u32,
	result: RuleResult,
}

impl RuleCondition {
	fn part_eval(&self, part: &Part) -> Option<RuleResult> {
		let part_value = part.get_single_variable(self.variable);
		let matches = match self.operator {
			ConditionOperator::LessThan => part_value < self.compare_value,
			ConditionOperator::GreaterThan => part_value > self.compare_value,
		};
		if matches {
			Some(self.result.clone())
		} else {
			None
		}
	}
}

struct Rule {
	conditions: Vec<RuleCondition>,
	default: RuleResult,
}

impl Rule {
	fn result(&self, part: &Part) -> RuleResult {
		for condition in self.conditions.iter() {
			if let Some(result) = condition.part_eval(part) {
				return result;
			}
		}
		self.default.clone()
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let (rules, parts) = {
		let input = fs::read_to_string("input.txt")?;

		let mut input_lines = input.lines();
		let mut rules: HashMap<String, Rule> = HashMap::new();
		for line in input_lines.by_ref() {
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
				let compare_value: u32 = value.parse()?;
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

		let mut parts: Vec<Part> = Vec::new();
		for line in input_lines {
			let line = line.strip_prefix('{').unwrap();
			let line = line.strip_suffix('}').unwrap();
			let mut line_parts = line.split(',');

			let x_part = line_parts.next().unwrap();
			let mut x_parts = x_part.split('=');
			assert_eq!(x_parts.next(), Some("x"));
			let x: u32 = x_parts.next().unwrap().parse()?;
			assert!(x_parts.next().is_none());

			let m_part = line_parts.next().unwrap();
			let mut m_parts = m_part.split('=');
			assert_eq!(m_parts.next(), Some("m"));
			let m: u32 = m_parts.next().unwrap().parse()?;
			assert!(m_parts.next().is_none());

			let a_part = line_parts.next().unwrap();
			let mut a_parts = a_part.split('=');
			assert_eq!(a_parts.next(), Some("a"));
			let a: u32 = a_parts.next().unwrap().parse()?;
			assert!(a_parts.next().is_none());

			let s_part = line_parts.next().unwrap();
			let mut s_parts = s_part.split('=');
			assert_eq!(s_parts.next(), Some("s"));
			let s: u32 = s_parts.next().unwrap().parse()?;
			assert!(s_parts.next().is_none());

			parts.push(Part { x, m, a, s });
		}

		(rules, parts)
	};

	let mut accepted_sum = 0;
	for part in parts.iter() {
		let mut current_rule = rules.get("in").unwrap();
		loop {
			let result = current_rule.result(part);
			match result {
				RuleResult::Accept => {
					accepted_sum += part.value_sum();
					break;
				}
				RuleResult::Reject => break,
				RuleResult::Evaluate(rule) => current_rule = rules.get(&rule).unwrap(),
			}
		}
	}

	println!("{}", accepted_sum);

	Ok(())
}
