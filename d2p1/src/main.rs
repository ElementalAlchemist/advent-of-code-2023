use std::error::Error;
use std::fs;

const MAX_RED_CUBES: u32 = 12;
const MAX_GREEN_CUBES: u32 = 13;
const MAX_BLUE_CUBES: u32 = 14;

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

	let mut possible_game_id_sum: u32 = 0;

	'game: for game in games.iter() {
		for draw in game.draws.iter() {
			for cube_color in draw.cube_colors.iter() {
				let max = match cube_color.color {
					Color::Red => MAX_RED_CUBES,
					Color::Green => MAX_GREEN_CUBES,
					Color::Blue => MAX_BLUE_CUBES,
				};
				if cube_color.count > max {
					continue 'game;
				}
			}
		}
		possible_game_id_sum += game.id;
	}

	println!("{}", possible_game_id_sum);

	Ok(())
}
