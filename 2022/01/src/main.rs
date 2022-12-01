fn parse() -> Vec<u32> {
	let mut calories = Vec::new();
	let mut current_calories = 0;

	for line in include_str!("input.txt").lines() {
		if line.is_empty() {
			calories.push(current_calories);
			current_calories = 0;
		} else {
			current_calories += line.parse::<u32>().unwrap();
		}
	}
	calories.push(current_calories);

	calories
}

fn part1() -> u32 {
	let calories = parse();
	calories.into_iter().max().unwrap()
}

fn part2() -> u32 {
	let calories = parse();
	let mut top_3 = Vec::new();

	for _ in 0..3 {
		let maximum = calories
			.iter()
			.filter(|x| !top_3.contains(x))
			.max()
			.unwrap();
		top_3.push(maximum);
	}

	top_3.into_iter().sum()
}

fn main() {
	println!("The solution to part 1 is {}.", part1());
	println!("The solution to part 2 is {}.", part2());
}
