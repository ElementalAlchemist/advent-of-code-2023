use std::error::Error;
use std::fs;
use z3::ast::{Ast, Int};
use z3::{Config, Context, SatResult, Solver};

#[derive(Clone)]
struct Coordinate {
	x: i64,
	y: i64,
	z: i64,
}

#[derive(Clone)]
struct Hailstone {
	position: Coordinate,
	velocity: Coordinate,
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
			let x: i64 = coordinate_parts.next().unwrap().parse()?;
			let y: i64 = coordinate_parts.next().unwrap().parse()?;
			let z: i64 = coordinate_parts.next().unwrap().parse()?;
			assert!(coordinate_parts.next().is_none());
			let position = Coordinate { x, y, z };

			let mut velocity_parts = velocity.split(", ");
			let x: i64 = velocity_parts.next().unwrap().parse()?;
			let y: i64 = velocity_parts.next().unwrap().parse()?;
			let z: i64 = velocity_parts.next().unwrap().parse()?;
			assert!(velocity_parts.next().is_none());
			let velocity = Coordinate { x, y, z };

			hailstones.push(Hailstone { position, velocity });
		}

		hailstones
	};

	let mut hailstone_iter = hailstones.iter();
	let first_hailstone = hailstone_iter.next().unwrap();
	let second_hailstone = hailstone_iter.next().unwrap();
	let third_hailstone = hailstone_iter.next().unwrap();

	let z3_config = Config::new();
	let z3_context = Context::new(&z3_config);

	let x1 = Int::from_i64(&z3_context, first_hailstone.position.x);
	let x1v = Int::from_i64(&z3_context, first_hailstone.velocity.x);

	let y1 = Int::from_i64(&z3_context, first_hailstone.position.y);
	let y1v = Int::from_i64(&z3_context, first_hailstone.velocity.y);

	let z1 = Int::from_i64(&z3_context, first_hailstone.position.z);
	let z1v = Int::from_i64(&z3_context, first_hailstone.velocity.z);

	let x2 = Int::from_i64(&z3_context, second_hailstone.position.x);
	let x2v = Int::from_i64(&z3_context, second_hailstone.velocity.x);

	let y2 = Int::from_i64(&z3_context, second_hailstone.position.y);
	let y2v = Int::from_i64(&z3_context, second_hailstone.velocity.y);

	let z2 = Int::from_i64(&z3_context, second_hailstone.position.z);
	let z2v = Int::from_i64(&z3_context, second_hailstone.velocity.z);

	let x3 = Int::from_i64(&z3_context, third_hailstone.position.x);
	let x3v = Int::from_i64(&z3_context, third_hailstone.velocity.x);

	let y3 = Int::from_i64(&z3_context, third_hailstone.position.y);
	let y3v = Int::from_i64(&z3_context, third_hailstone.velocity.y);

	let z3 = Int::from_i64(&z3_context, third_hailstone.position.z);
	let z3v = Int::from_i64(&z3_context, third_hailstone.velocity.z);

	let x0 = Int::new_const(&z3_context, "x0");
	let y0 = Int::new_const(&z3_context, "y0");
	let z0 = Int::new_const(&z3_context, "z0");

	let x0v = Int::new_const(&z3_context, "x0v");
	let y0v = Int::new_const(&z3_context, "y0v");
	let z0v = Int::new_const(&z3_context, "z0v");

	let elapse_0_1 = Int::new_const(&z3_context, "elapse_0_1");
	let elapse_0_2 = Int::new_const(&z3_context, "elapse_0_2");
	let elapse_0_3 = Int::new_const(&z3_context, "elapse_0_3");

	let x0_dest1 = &x0 + &x0v * &elapse_0_1;
	let x0_dest2 = &x0 + &x0v * &elapse_0_2;
	let x0_dest3 = &x0 + &x0v * &elapse_0_3;
	let y0_dest1 = &y0 + &y0v * &elapse_0_1;
	let y0_dest2 = &y0 + &y0v * &elapse_0_2;
	let y0_dest3 = &y0 + &y0v * &elapse_0_3;
	let z0_dest1 = &z0 + &z0v * &elapse_0_1;
	let z0_dest2 = &z0 + &z0v * &elapse_0_2;
	let z0_dest3 = &z0 + &z0v * &elapse_0_3;

	let x1_dest = &x1 + &x1v * &elapse_0_1;
	let y1_dest = &y1 + &y1v * &elapse_0_1;
	let z1_dest = &z1 + &z1v * &elapse_0_1;
	let x2_dest = &x2 + &x2v * &elapse_0_2;
	let y2_dest = &y2 + &y2v * &elapse_0_2;
	let z2_dest = &z2 + &z2v * &elapse_0_2;
	let x3_dest = &x3 + &x3v * &elapse_0_3;
	let y3_dest = &y3 + &y3v * &elapse_0_3;
	let z3_dest = &z3 + &z3v * &elapse_0_3;

	let solver = Solver::new(&z3_context);
	solver.assert(&x0_dest1._eq(&x1_dest));
	solver.assert(&y0_dest1._eq(&y1_dest));
	solver.assert(&z0_dest1._eq(&z1_dest));
	solver.assert(&x0_dest2._eq(&x2_dest));
	solver.assert(&y0_dest2._eq(&y2_dest));
	solver.assert(&z0_dest2._eq(&z2_dest));
	solver.assert(&x0_dest3._eq(&x3_dest));
	solver.assert(&y0_dest3._eq(&y3_dest));
	solver.assert(&z0_dest3._eq(&z3_dest));

	assert_eq!(solver.check(), SatResult::Sat);
	let model = solver.get_model().unwrap();

	let x0 = model.get_const_interp(&x0).unwrap().as_i64().unwrap();
	let y0 = model.get_const_interp(&y0).unwrap().as_i64().unwrap();
	let z0 = model.get_const_interp(&z0).unwrap().as_i64().unwrap();
	let x0v = model.get_const_interp(&x0v).unwrap().as_i64().unwrap();
	let y0v = model.get_const_interp(&y0v).unwrap().as_i64().unwrap();
	let z0v = model.get_const_interp(&z0v).unwrap().as_i64().unwrap();

	let answer = x0 + y0 + z0;
	println!("{} ({}, {}, {}) ({}, {}, {})", answer, x0, y0, z0, x0v, y0v, z0v);

	Ok(())
}
