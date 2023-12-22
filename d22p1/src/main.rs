use std::collections::{BTreeMap, HashSet};
use std::error::Error;
use std::fs;
use std::ops::RangeInclusive;

#[derive(Clone, Eq, Hash, PartialEq)]
struct Brick {
	x: RangeInclusive<u32>,
	y: RangeInclusive<u32>,
	z: RangeInclusive<u32>,
}

fn main() -> Result<(), Box<dyn Error>> {
	let bricks = {
		let input = fs::read_to_string("input.txt")?;

		let mut bricks: Vec<Brick> = Vec::new();
		for line in input.lines() {
			let mut line_parts = line.split('~');
			let low_corner_data = line_parts.next().unwrap();
			let high_corner_data = line_parts.next().unwrap();
			assert!(line_parts.next().is_none());

			let mut low_corner_parts = low_corner_data.split(',');
			let low_x: u32 = low_corner_parts.next().unwrap().parse()?;
			let low_y: u32 = low_corner_parts.next().unwrap().parse()?;
			let low_z: u32 = low_corner_parts.next().unwrap().parse()?;

			let mut high_corner_parts = high_corner_data.split(',');
			let high_x: u32 = high_corner_parts.next().unwrap().parse()?;
			let high_y: u32 = high_corner_parts.next().unwrap().parse()?;
			let high_z: u32 = high_corner_parts.next().unwrap().parse()?;

			bricks.push(Brick {
				x: low_x..=high_x,
				y: low_y..=high_y,
				z: low_z..=high_z,
			});
		}

		bricks
	};

	let mut low_z: BTreeMap<u32, HashSet<Brick>> = BTreeMap::new();
	let mut high_z: BTreeMap<u32, HashSet<Brick>> = BTreeMap::new();

	for brick in bricks.iter() {
		low_z.entry(*brick.z.start()).or_default().insert(brick.clone());
		high_z.entry(*brick.z.end()).or_default().insert(brick.clone());
	}

	for (_, bricks) in low_z.range(2..) {
		for brick in bricks.iter() {
			let mut reduce_z_by = 0;
			'z_level: for lower_z in 1..*brick.z.start() {
				if let Some(z_bricks) = high_z.get(&(*brick.z.start() - lower_z)) {
					for check_brick in z_bricks.iter() {
						if check_brick.x.end() >= brick.x.start()
							&& brick.x.end() >= check_brick.x.start()
							&& check_brick.y.end() >= brick.y.start()
							&& brick.y.end() >= check_brick.y.start()
						{
							break 'z_level;
						}
					}
				}
				reduce_z_by = lower_z;
			}
			if reduce_z_by > 0 {
				let start_high_bricks = high_z.get_mut(brick.z.end()).unwrap();
				start_high_bricks.remove(brick);
				let x = brick.x.clone();
				let y = brick.y.clone();
				let z = (*brick.z.start() - reduce_z_by)..=(*brick.z.end() - reduce_z_by);
				let new_brick = Brick { x, y, z };
				high_z.entry(*new_brick.z.end()).or_default().insert(new_brick);
			}
		}
	}

	low_z.clear();
	for brick in high_z.values().flatten() {
		low_z.entry(*brick.z.start()).or_default().insert(brick.clone());
	}
	let bricks: Vec<Brick> = high_z.values().flatten().cloned().collect();
	let mut safe_bricks: u32 = 0;
	for brick in bricks.iter() {
		let mut can_disintegrate = true;
		let z_above = *brick.z.end() + 1;
		if let Some(above_bricks) = low_z.get(&z_above) {
			for above_brick in above_bricks {
				let mut brick_is_supported = false;
				for below_brick in high_z.get(brick.z.end()).unwrap().iter() {
					if *below_brick == *brick {
						continue;
					}
					if *below_brick.x.end() >= *above_brick.x.start()
						&& *above_brick.x.end() >= *below_brick.x.start()
						&& *below_brick.y.end() >= *above_brick.y.start()
						&& *above_brick.y.end() >= *below_brick.y.start()
					{
						brick_is_supported = true;
						break;
					}
				}
				if !brick_is_supported {
					can_disintegrate = false;
					break;
				}
			}
		}
		if can_disintegrate {
			safe_bricks += 1;
		}
	}

	println!("{}", safe_bricks);

	Ok(())
}
