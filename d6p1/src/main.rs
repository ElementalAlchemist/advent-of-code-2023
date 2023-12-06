use std::error::Error;
use std::fs;

struct EventRecord {
	time: u32,
	distance: u32,
}

fn main() -> Result<(), Box<dyn Error>> {
	let records = {
		let input = fs::read_to_string("input.txt")?;

		let mut lines = input.lines().filter(|s| !s.is_empty());
		let mut times = lines.next().unwrap();
		let mut distances = lines.next().unwrap();

		times = times.strip_prefix("Time:").unwrap();
		distances = distances.strip_prefix("Distance:").unwrap();

		times = times.trim();
		distances = distances.trim();

		let mut records: Vec<EventRecord> = Vec::new();
		let times_iter = times.split(' ').filter(|s| !s.is_empty());
		let mut distances_iter = distances.split(' ').filter(|s| !s.is_empty());

		for time in times_iter {
			let distance = distances_iter.next().unwrap();
			let time: u32 = time.parse().unwrap();
			let distance: u32 = distance.parse().unwrap();

			records.push(EventRecord { time, distance });
		}

		records
	};

	let mut ways_to_beat = 1;

	for record in records.iter() {
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
	}

	println!("{}", ways_to_beat);

	Ok(())
}
