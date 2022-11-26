use std::collections::HashMap;

fn parse_program() -> HashMap<usize, i64> {
	include_str!("input.txt")
		.split(",")
		.enumerate()
		.map(|(k, v)| (k, v.trim().parse().unwrap()))
		.collect()
}

fn get_param(
	program: &mut HashMap<usize, i64>,
	pc: usize,
	base: i64,
	mode: i64,
	index: i64,
) -> i64 {
	program.entry(pc + index as usize).or_insert(0);
	let value = program[&(pc + index as usize)] as usize;

	match mode {
		0 => {
			program.entry(value).or_insert(0);
			program[&value]
		}
		1 => value as i64,
		2 => {
			program.entry((value as i64 + base) as usize).or_insert(0);
			program[&((value as i64 + base) as usize)]
		}
		_ => panic!("invalid mode"),
	}
}

fn get_address(
	program: &mut HashMap<usize, i64>,
	pc: usize,
	base: i64,
	mode: i64,
	index: i64,
) -> usize {
	program.entry(pc + index as usize).or_insert(0);
	let value = program[&(pc + index as usize)] as usize;

	match mode {
		0 => value,
		2 => {
			program.entry((value as i64 + base) as usize).or_insert(0);
			(value as i64 + base) as usize
		}
		_ => panic!("invalid mode"),
	}
}

fn compute(input: i64) -> i64 {
	let mut program = parse_program();
	let mut pc = 0;
	let mut last_output = 0;
	let mut relative_base: i64 = 0;

	loop {
		let opcode = program[&pc];

		let instruction = opcode % 100;
		let mode_first = opcode / 100 % 10;
		let mode_second = opcode / 1000 % 10;
		let mode_third = opcode / 10000 % 10;

		match instruction {
			1 => {
				let first_operand = get_param(&mut program, pc, relative_base, mode_first, 1);
				let second_operand = get_param(&mut program, pc, relative_base, mode_second, 2);
				let third_operand = get_address(&mut program, pc, relative_base, mode_third, 3);

				program.insert(third_operand, first_operand + second_operand);
				pc += 4;
			}
			2 => {
				let first_operand = get_param(&mut program, pc, relative_base, mode_first, 1);
				let second_operand = get_param(&mut program, pc, relative_base, mode_second, 2);
				let third_operand = get_address(&mut program, pc, relative_base, mode_third, 3);

				program.insert(third_operand, first_operand * second_operand);
				pc += 4;
			}
			3 => {
				let first_operand = get_address(&mut program, pc, relative_base, mode_first, 1);

				program.insert(first_operand, input);
				pc += 2;
			}
			4 => {
				let first_operand = get_address(&mut program, pc, relative_base, mode_first, 1);

				last_output = *program.get(&first_operand).unwrap();
				pc += 2;
			}
			5 => {
				let first_operand = get_param(&mut program, pc, relative_base, mode_first, 1);
				let second_operand = get_param(&mut program, pc, relative_base, mode_second, 2);

				if first_operand != 0 {
					pc = second_operand as usize;
				} else {
					pc += 3;
				}
			}
			6 => {
				let first_operand = get_param(&mut program, pc, relative_base, mode_first, 1);
				let second_operand = get_param(&mut program, pc, relative_base, mode_second, 2);

				if first_operand == 0 {
					pc = second_operand as usize;
				} else {
					pc += 3;
				}
			}
			7 => {
				let first_operand = get_param(&mut program, pc, relative_base, mode_first, 1);
				let second_operand = get_param(&mut program, pc, relative_base, mode_second, 2);
				let third_operand = get_address(&mut program, pc, relative_base, mode_third, 3);

				if first_operand < second_operand {
					program.insert(third_operand, 1);
				} else {
					program.insert(third_operand, 0);
				}
				pc += 4;
			}
			8 => {
				let first_operand = get_param(&mut program, pc, relative_base, mode_first, 1);
				let second_operand = get_param(&mut program, pc, relative_base, mode_second, 2);
				let third_operand = get_address(&mut program, pc, relative_base, mode_third, 3);

				if first_operand == second_operand {
					program.insert(third_operand, 1);
				} else {
					program.insert(third_operand, 0);
				}
				pc += 4;
			}
			9 => {
				let first_operand = get_param(&mut program, pc, relative_base, mode_first, 1);

				relative_base += first_operand;
				pc += 2;
			}
			99 => {
				return last_output;
			}
			_ => {
				panic!("invalid opcode")
			}
		}
	}
}

fn part1() -> i64 {
	compute(1)
}

fn part2() -> i64 {
	compute(2)
}

fn main() {
	println!("The solution to part 1 is {}.", part1());
	println!("The solution to part 2 is {}.", part2());
}
