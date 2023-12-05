use std::error::Error;
use std::fs;
use std::ops::Range;

#[derive(Eq, PartialEq)]
enum ParseLocation {
	Seeds,
	SeedSoil,
	SoilFertilizer,
	FertilizerWater,
	WaterLight,
	LightTemp,
	TempHumidity,
	HumidityLocation,
}

fn map_num(map: &[(Range<i64>, i64)], input: i64) -> i64 {
	for (start_range, modify_by) in map.iter() {
		if start_range.contains(&input) {
			return input + modify_by;
		}
	}
	input
}

fn main() -> Result<(), Box<dyn Error>> {
	let input = fs::read_to_string("input.txt")?;

	let mut seeds: Vec<Range<i64>> = Vec::new();
	let mut seed_soil_map: Vec<(Range<i64>, i64)> = Vec::new();
	let mut soil_fertilizer_map: Vec<(Range<i64>, i64)> = Vec::new();
	let mut fertilizer_water_map: Vec<(Range<i64>, i64)> = Vec::new();
	let mut water_light_map: Vec<(Range<i64>, i64)> = Vec::new();
	let mut light_temperature_map: Vec<(Range<i64>, i64)> = Vec::new();
	let mut temperature_humidity_map: Vec<(Range<i64>, i64)> = Vec::new();
	let mut humidity_location_map: Vec<(Range<i64>, i64)> = Vec::new();

	let mut parse_location = ParseLocation::Seeds;
	for line in input.lines().filter(|s| !s.is_empty()) {
		match line {
			"seed-to-soil map:" => parse_location = ParseLocation::SeedSoil,
			"soil-to-fertilizer map:" => parse_location = ParseLocation::SoilFertilizer,
			"fertilizer-to-water map:" => parse_location = ParseLocation::FertilizerWater,
			"water-to-light map:" => parse_location = ParseLocation::WaterLight,
			"light-to-temperature map:" => parse_location = ParseLocation::LightTemp,
			"temperature-to-humidity map:" => parse_location = ParseLocation::TempHumidity,
			"humidity-to-location map:" => parse_location = ParseLocation::HumidityLocation,
			_ => {
				if parse_location == ParseLocation::Seeds {
					line.strip_prefix("seeds: ")
						.unwrap()
						.split(' ')
						.fold(None, |prev, seed_step| {
							let seed_step: i64 = seed_step.parse().unwrap();
							match prev {
								Some(seed) => {
									seeds.push(seed..(seed + seed_step));
									None
								}
								None => Some(seed_step),
							}
						});
					continue;
				}
				let mut number_iter = line.split(' ').map(|n| n.parse().unwrap());
				let destination: i64 = number_iter.next().unwrap();
				let start: i64 = number_iter.next().unwrap();
				let range: i64 = number_iter.next().unwrap();
				assert!(number_iter.next().is_none());

				let modify_by = start - destination;

				let map_ref = match parse_location {
					ParseLocation::Seeds => unreachable!(),
					ParseLocation::SeedSoil => &mut seed_soil_map,
					ParseLocation::SoilFertilizer => &mut soil_fertilizer_map,
					ParseLocation::FertilizerWater => &mut fertilizer_water_map,
					ParseLocation::WaterLight => &mut water_light_map,
					ParseLocation::LightTemp => &mut light_temperature_map,
					ParseLocation::TempHumidity => &mut temperature_humidity_map,
					ParseLocation::HumidityLocation => &mut humidity_location_map,
				};
				map_ref.push((destination..(destination + range), modify_by));
			}
		}
	}

	let mut current_location = 0;
	'location: loop {
		let humidity = map_num(&humidity_location_map, current_location);
		let temperature = map_num(&temperature_humidity_map, humidity);
		let light = map_num(&light_temperature_map, temperature);
		let water = map_num(&water_light_map, light);
		let fertilizer = map_num(&fertilizer_water_map, water);
		let soil = map_num(&soil_fertilizer_map, fertilizer);
		let seed = map_num(&seed_soil_map, soil);

		for seed_range in seeds.iter() {
			if seed_range.contains(&seed) {
				break 'location;
			}
		}

		current_location += 1;
	}

	println!("{}", current_location);

	Ok(())
}
