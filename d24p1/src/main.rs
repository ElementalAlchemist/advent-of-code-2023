use std::error::Error;
use std::fs;

const TEST_MIN_COORD: f64 = 200000000000000.0;
const TEST_MAX_COORD: f64 = 400000000000000.0;

#[derive(Clone)]
struct Coordinate {
	x: f64,
	y: f64,
	z: f64,
}

#[derive(Clone)]
struct Hailstone {
	position: Coordinate,
	velocity: Coordinate,
}

#[derive(Clone)]
struct HailstonePath {
	start_position: Coordinate,
	move_pos_x: bool,
	slope: f64,
	origin_offset: f64,
}

impl HailstonePath {
	fn intersection_point(&self, other: &Self) -> Option<Coordinate> {
		if self.slope == other.slope {
			return None;
		}
		let x = (other.origin_offset - self.origin_offset) / (self.slope - other.slope);
		let y = x * self.slope + self.origin_offset;

		if self.move_pos_x && x < self.start_position.x {
			return None;
		}
		if !self.move_pos_x && x > self.start_position.x {
			return None;
		}
		if other.move_pos_x && x < other.start_position.x {
			return None;
		}
		if !other.move_pos_x && x > other.start_position.x {
			return None;
		}

		Some(Coordinate {
			x,
			y,
			z: self.start_position.z,
		})
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let hailstones = {
		let input = fs::read_to_string("input.txt")?;

		let mut hailstones: Vec<Hailstone> = Vec::new();
		for line in input.lines() {
			let mut line_parts = line.split(" @ ");
			let coordinate = line_parts.next().unwrap();
			let velocity = line_parts.next().unwrap();
			assert!(line_parts.next().is_none());

			let mut coordinate_parts = coordinate.split(", ");
			let x: f64 = coordinate_parts.next().unwrap().parse()?;
			let y: f64 = coordinate_parts.next().unwrap().parse()?;
			let z: f64 = coordinate_parts.next().unwrap().parse()?;
			assert!(coordinate_parts.next().is_none());
			let position = Coordinate { x, y, z };

			let mut velocity_parts = velocity.split(", ");
			let x: f64 = velocity_parts.next().unwrap().parse()?;
			let y: f64 = velocity_parts.next().unwrap().parse()?;
			let z: f64 = velocity_parts.next().unwrap().parse()?;
			assert!(velocity_parts.next().is_none());
			let velocity = Coordinate { x, y, z };

			hailstones.push(Hailstone { position, velocity });
		}

		hailstones
	};

	let mut hailstone_paths: Vec<HailstonePath> = Vec::new();
	for hailstone in hailstones {
		let start_position = hailstone.position;
		let move_pos_x = hailstone.velocity.x > 0.0;
		let slope = hailstone.velocity.y / hailstone.velocity.x;
		let origin_offset = start_position.y - start_position.x * slope;
		hailstone_paths.push(HailstonePath {
			start_position,
			move_pos_x,
			slope,
			origin_offset,
		});
	}

	let mut intersections: u64 = 0;
	for (first_index, first_hailstone) in hailstone_paths.iter().enumerate() {
		for second_hailstone in hailstone_paths.iter().skip(first_index + 1) {
			if let Some(intersection) = first_hailstone.intersection_point(second_hailstone) {
				if intersection.x >= TEST_MIN_COORD
					&& intersection.x <= TEST_MAX_COORD
					&& intersection.y >= TEST_MIN_COORD
					&& intersection.y <= TEST_MAX_COORD
				{
					intersections += 1;
				}
			}
		}
	}

	println!("{}", intersections);

	Ok(())
}
