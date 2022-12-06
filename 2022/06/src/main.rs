use std::collections::VecDeque;

fn parse() -> String {
	include_str!("input.txt").to_string()
}

fn find_first_n_distinct_chars(datastream: &String, n: usize) -> usize {
	let mut iterator = datastream.chars().enumerate();
	let mut last_four: VecDeque<char> = VecDeque::new();
	for _ in 0..n {
		last_four.push_front(
			iterator
				.next()
				.expect("message doesn't have enought chars")
				.1,
		);
	}

	let all_different_elements = |vec: &VecDeque<char>| -> bool {
		let mut seen = Vec::new();

		for el in vec {
			if seen.contains(el) {
				return false;
			} else {
				seen.push(*el);
			}
		}

		true
	};

	while let Some((i, c)) = iterator.next() {
		if all_different_elements(&last_four) {
			return i;
		}
		last_four.pop_back();
		last_four.push_front(c);
	}

	0
}

fn part1(datastream: &String) -> usize {
	find_first_n_distinct_chars(datastream, 4)
}

fn part2(datastream: &String) -> usize {
	find_first_n_distinct_chars(datastream, 14)
}

fn main() {
	let datastream = parse();
	println!("The solution to part 1 is {}.", part1(&datastream));
	println!("The solution to part 2 is {}.", part2(&datastream));
}
