use regex::Regex;

fn parse() -> Vec<((u32, u32), (u32, u32))> {
	let mut assigments = Vec::new();
	let re = Regex::new(r"\d+").unwrap();

	for line in include_str!("input.txt").lines() {
		let caps: Vec<u32> = re
			.find_iter(line)
			.filter_map(|digits| digits.as_str().parse().ok())
			.collect();

		assigments.push(((caps[0], caps[1]), (caps[2], caps[3])));
	}

	assigments
}

fn part1(assignments: &Vec<((u32, u32), (u32, u32))>) -> u32 {
	let mut count = 0;
	for ((s1, e1), (s2, e2)) in assignments {
		if (s1 <= s2 && e2 <= e1) || (s2 <= s1 && e1 <= e2) {
			count += 1;
		}
	}

	count
}

fn part2(assignments: &Vec<((u32, u32), (u32, u32))>) -> u32 {
	let mut count = 0;
	for ((s1, e1), (s2, e2)) in assignments {
		if !(e1 < s2 || e2 < s1) {
			count += 1;
		}
	}

	count
}

fn main() {
	let assignments = parse();

	println!("The solution to part 1 is {}.", part1(&assignments));
	println!("The solution to part 2 is {}.", part2(&assignments));
}
