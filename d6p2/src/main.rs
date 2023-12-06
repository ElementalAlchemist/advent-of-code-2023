use std::error::Error;
use std::fs;

struct EventRecord {
	time: u64,
	distance: u64,
}

fn main() -> Result<(), Box<dyn Error>> {
	let record = {
		let input = fs::read_to_string("input.txt")?;

		let mut lines = input.lines().filter(|s| !s.is_empty());
		let mut times = lines.next().unwrap();
		let mut distances = lines.next().unwrap();

		times = times.strip_prefix("Time:").unwrap();
		distances = distances.strip_prefix("Distance:").unwrap();

		times = times.trim();
		distances = distances.trim();

		let times_iter = times.split(' ').filter(|s| !s.is_empty());
		let mut distances_iter = distances.split(' ').filter(|s| !s.is_empty());

		let mut big_time = String::new();
		let mut big_distance = String::new();

		for time in times_iter {
			let distance = distances_iter.next().unwrap();

			big_time.push_str(time);
			big_distance.push_str(distance);
		}

		let time: u64 = big_time.parse().unwrap();
		let distance: u64 = big_distance.parse().unwrap();

		EventRecord { time, distance }
	};

	let mut ways_to_beat = 1;

	let mut button_hold_time = record.time / 2;
	let mut ways_to_beat_record = 0;
	loop {
		let distance = (record.time - button_hold_time) * button_hold_time;
		if distance > record.distance {
			ways_to_beat_record += 1;
		} else {
			break;
		}
		button_hold_time -= 1;
	}
	ways_to_beat_record *= 2;
	if record.time % 2 == 0 {
		ways_to_beat_record -= 1;
	}
	ways_to_beat *= ways_to_beat_record;

	println!("{}", ways_to_beat);

	Ok(())
}
