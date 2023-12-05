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

fn map_num(map: &[(Range<u64>, u64)], input: u64) -> u64 {
	for (start_range, dest_start) in map.iter() {
		if start_range.contains(&input) {
			let difference = input - start_range.start;
			return dest_start + difference;
		}
	}
	input
}

fn main() -> Result<(), Box<dyn Error>> {
	let input = fs::read_to_string("input.txt")?;

	let mut seeds: Vec<Range<u64>> = Vec::new();
	let mut seed_soil_map: Vec<(Range<u64>, u64)> = Vec::new();
	let mut soil_fertilizer_map: Vec<(Range<u64>, u64)> = Vec::new();
	let mut fertilizer_water_map: Vec<(Range<u64>, u64)> = Vec::new();
	let mut water_light_map: Vec<(Range<u64>, u64)> = Vec::new();
	let mut light_temperature_map: Vec<(Range<u64>, u64)> = Vec::new();
	let mut temperature_humidity_map: Vec<(Range<u64>, u64)> = Vec::new();
	let mut humidity_location_map: Vec<(Range<u64>, u64)> = Vec::new();

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
							let seed_step: u64 = seed_step.parse().unwrap();
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
				let destination: u64 = number_iter.next().unwrap();
				let start: u64 = number_iter.next().unwrap();
				let range: u64 = number_iter.next().unwrap();
				assert!(number_iter.next().is_none());

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
				map_ref.push((start..(start + range), destination));
			}
		}
	}

	let mut closest_location = u64::MAX;
	for seed_range in seeds {
		for seed in seed_range {
			let soil = map_num(&seed_soil_map, seed);
			let fertilizer = map_num(&soil_fertilizer_map, soil);
			let water = map_num(&fertilizer_water_map, fertilizer);
			let light = map_num(&water_light_map, water);
			let temperature = map_num(&light_temperature_map, light);
			let humidity = map_num(&temperature_humidity_map, temperature);
			let location = map_num(&humidity_location_map, humidity);
			closest_location = closest_location.min(location);
		}
	}

	println!("{}", closest_location);

	Ok(())
}
