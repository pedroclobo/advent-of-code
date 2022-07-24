use std::collections::HashMap;

struct Instruction {
	direction: char,
	path_size: i32,
}

impl Instruction {
	fn new(direction: char, path_size: i32) -> Self {
		Instruction {
			direction,
			path_size,
		}
	}
}

#[derive(Copy, Clone, Hash, Eq)]
struct Point {
	x: i32,
	y: i32,
}

impl Point {
	fn new(x: i32, y: i32) -> Self {
		Point { x, y }
	}

	fn move_by(&mut self, direction: char, path_size: i32) {
		if direction == 'L' {
			self.x -= path_size;
		} else if direction == 'R' {
			self.x += path_size;
		} else if direction == 'U' {
			self.y -= path_size;
		} else if direction == 'D' {
			self.y += path_size;
		}
	}

	fn manhattan_distance(&self) -> i32 {
		self.x.abs() + self.y.abs()
	}
}

impl PartialEq for Point {
	fn eq(&self, other: &Self) -> bool {
		(self.x, self.y) == (other.x, other.y)
	}

	fn ne(&self, other: &Self) -> bool {
		(self.x, self.y) != (other.x, other.y)
	}
}

fn part1(instructions_first: &Vec<Instruction>, instructions_second: &Vec<Instruction>) -> i32 {
	let mut paths: HashMap<Point, u32> = HashMap::new();
	let mut intersections: Vec<Point> = Vec::new();

	let mut current_point = Point::new(0, 0);
	for instruction in instructions_first {
		for _ in 1..=instruction.path_size {
			current_point.move_by(instruction.direction, 1);
			let count = paths.entry(current_point).or_insert(0);
			*count += 1;
		}
	}

	current_point = Point::new(0, 0);
	for instruction in instructions_second {
		for _ in 1..=instruction.path_size {
			current_point.move_by(instruction.direction, 1);
			let count = paths.entry(current_point).or_insert(0);
			if *count == 1 {
				intersections.push(current_point);
			}
		}
	}

	let nearest_intersection = intersections
		.into_iter()
		.min_by(|x, y| x.manhattan_distance().cmp(&y.manhattan_distance()))
		.unwrap();

	nearest_intersection.manhattan_distance()
}

fn part2(instructions_first: &Vec<Instruction>, instructions_second: &Vec<Instruction>) -> u32 {
	let mut path_first: HashMap<Point, u32> = HashMap::new();
	let mut path_second: HashMap<Point, u32> = HashMap::new();
	let mut paths: HashMap<Point, u32> = HashMap::new();
	let mut intersections: Vec<Point> = Vec::new();

	// compute paths
	let mut current_point = Point::new(0, 0);
	let mut steps = 1;
	for instruction in instructions_first {
		for _ in 1..=instruction.path_size {
			current_point.move_by(instruction.direction, 1);
			let count = paths.entry(current_point).or_insert(0);
			*count += 1;
			path_first.entry(current_point).or_insert(steps);
			steps += 1;
		}
	}

	current_point = Point::new(0, 0);
	steps = 1;
	for instruction in instructions_second {
		for _ in 1..=instruction.path_size {
			current_point.move_by(instruction.direction, 1);
			let count = paths.entry(current_point).or_insert(0);
			if *count == 1 {
				intersections.push(current_point);
			}
			path_second.entry(current_point).or_insert(steps);
			steps += 1;
		}
	}

	let mut min_steps = u32::MAX;
	for point in intersections {
		let steps_first = path_first.get(&point).unwrap();
		let steps_second = path_second.get(&point).unwrap();
		if steps_first + steps_second < min_steps {
			min_steps = steps_first + steps_second;
		}
	}

	min_steps
}

fn main() {
	let mut input = include_str!("input.txt").split('\n');

	let instructions_first: Vec<Instruction> = input
		.next()
		.unwrap()
		.split(",")
		.map(|inst| {
			Instruction::new(
				inst.chars().next().unwrap(),
				inst[1..].parse::<i32>().unwrap(),
			)
		})
		.collect();

	let instructions_second: Vec<Instruction> = input
		.next()
		.unwrap()
		.split(",")
		.map(|inst| {
			Instruction::new(
				inst.chars().next().unwrap(),
				inst[1..].parse::<i32>().unwrap(),
			)
		})
		.collect();

	println!(
		"The solution to part 1 is {}.",
		part1(&instructions_first, &instructions_second),
	);

	println!(
		"The solution to part 2 is {}.",
		part2(&instructions_first, &instructions_second),
	);
}
