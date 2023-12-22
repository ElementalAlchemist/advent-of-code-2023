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

fn get_fall_count(
	mut all_bricks: Vec<Brick>,
	mut low_z: BTreeMap<u32, HashSet<Brick>>,
	mut high_z: BTreeMap<u32, HashSet<Brick>>,
	remove_index: usize,
) -> u32 {
	let mut fallen_bricks = 0;
	let brick_to_remove = all_bricks.swap_remove(remove_index);
	low_z
		.get_mut(brick_to_remove.z.start())
		.unwrap()
		.remove(&brick_to_remove);
	high_z
		.get_mut(brick_to_remove.z.end())
		.unwrap()
		.remove(&brick_to_remove);

	loop {
		let mut remove_indices: Vec<usize> = Vec::new();
		for (brick_index, brick) in all_bricks.iter().enumerate() {
			if *brick.z.start() == 1 {
				continue;
			}
			let lower_z = *brick.z.start() - 1;
			if let Some(lower_bricks) = high_z.get(&lower_z) {
				let mut is_supported = false;
				for lower_brick in lower_bricks.iter() {
					if *lower_brick.x.end() >= *brick.x.start()
						&& *brick.x.end() >= *lower_brick.x.start()
						&& *lower_brick.y.end() >= *brick.y.start()
						&& *brick.y.end() >= *lower_brick.y.start()
					{
						is_supported = true;
						break;
					}
				}
				if !is_supported {
					remove_indices.push(brick_index);
				}
			} else {
				remove_indices.push(brick_index);
			}
		}

		remove_indices.sort_unstable();
		for index in remove_indices.iter().rev() {
			let fallen_brick = all_bricks.remove(*index);
			low_z.get_mut(fallen_brick.z.start()).unwrap().remove(&fallen_brick);
			high_z.get_mut(fallen_brick.z.end()).unwrap().remove(&fallen_brick);
			fallen_bricks += 1;
		}
		if remove_indices.is_empty() {
			break;
		}
	}

	fallen_bricks
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
	let mut bricks: Vec<Brick> = high_z.values().flatten().cloned().collect();
	bricks.sort_by(|a, b| {
		(*a.z.start())
			.cmp(b.z.start())
			.then_with(|| (*a.z.end()).cmp(b.z.end()))
	});

	let mut fallen_brick_count = 0;
	for (brick_index, _) in bricks.iter().enumerate() {
		fallen_brick_count += get_fall_count(bricks.clone(), low_z.clone(), high_z.clone(), brick_index);
	}
	println!("{}", fallen_brick_count);

	Ok(())
}
