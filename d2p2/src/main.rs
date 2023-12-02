use std::error::Error;
use std::fs;

enum Color {
	Red,
	Green,
	Blue,
}

struct ColorCount {
	color: Color,
	count: u32,
}

struct Draw {
	cube_colors: Vec<ColorCount>,
}

struct Game {
	id: u32,
	draws: Vec<Draw>,
}

fn main() -> Result<(), Box<dyn Error>> {
	let games: Vec<Game> = {
		let data = fs::read_to_string("input.txt")?;

		let mut games: Vec<Game> = Vec::new();
		for line in data.split('\n').filter(|s| !s.is_empty()) {
			let mut line_parts = line.split(": ");
			let game = line_parts.next().unwrap();
			let mut game_parts = game.split(' ');
			assert_eq!(game_parts.next().unwrap(), "Game");
			let game_id: u32 = game_parts.next().unwrap().parse().unwrap();
			assert!(game_parts.next().is_none());

			let draws = line_parts.next().unwrap();
			assert!(line_parts.next().is_none());

			let mut game_draws: Vec<Draw> = Vec::new();
			for draw in draws.split("; ") {
				let mut cube_colors: Vec<ColorCount> = Vec::new();
				for kind in draw.split(", ") {
					let mut kind_parts = kind.split(' ');
					let count: u32 = kind_parts.next().unwrap().parse().unwrap();
					let color = kind_parts.next().unwrap();
					assert!(kind_parts.next().is_none());
					let color = match color {
						"red" => Color::Red,
						"green" => Color::Green,
						"blue" => Color::Blue,
						_ => unreachable!(),
					};
					cube_colors.push(ColorCount { color, count });
				}
				game_draws.push(Draw { cube_colors });
			}

			games.push(Game {
				id: game_id,
				draws: game_draws,
			});
		}

		games
	};

	let mut sum_game_power: u32 = 0;

	for game in games.iter() {
		let mut max_red = 0;
		let mut max_green = 0;
		let mut max_blue = 0;
		for draw in game.draws.iter() {
			for cube_color in draw.cube_colors.iter() {
				match cube_color.color {
					Color::Red => max_red = max_red.max(cube_color.count),
					Color::Green => max_green = max_green.max(cube_color.count),
					Color::Blue => max_blue = max_blue.max(cube_color.count),
				}
			}
		}
		sum_game_power += max_red * max_green * max_blue;
	}

	println!("{}", sum_game_power);

	Ok(())
}
