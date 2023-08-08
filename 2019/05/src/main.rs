fn parse_program() -> Vec<i32> {
    let program: Vec<i32> = include_str!("input.txt")
        .split(",")
        .map(|int| int.trim().parse().unwrap())
        .collect();

    program
}

fn part1(program: &mut Vec<i32>) -> i32 {
    let mut i = 0;
    let mut last_output = 0;
    loop {
        let opcode = program[i];

        if [1, 2].contains(&(opcode % 10)) {
            let instruction = opcode % 10;
            let mode_first = opcode / 100 % 10;
            let mode_second = opcode / 1000 % 10;

            let first_operand = if mode_first == 0 {
                program[program[i + 1] as usize]
            } else {
                program[i + 1]
            };
            let second_operand = if mode_second == 0 {
                program[program[i + 2] as usize]
            } else {
                program[i + 2]
            };
            let third_operand = program[i + 3] as usize;

            if instruction == 1 {
                program[third_operand] = first_operand + second_operand;
            } else if instruction == 2 {
                program[third_operand] = first_operand * second_operand;
            }
            i += 4;
        } else {
            if opcode == 3 {
                let first_operand = program[i + 1] as usize;
                program[first_operand] = 1;
            } else if opcode == 4 {
                let mode_first = opcode / 100 % 10;
                if mode_first == 0 {
                    last_output = program[program[i + 1] as usize];
                } else {
                    last_output = program[i + 1];
                }
            } else if opcode == 99 {
                break;
            }
            i += 2;
        }
    }

    last_output
}

fn part2(program: &mut Vec<i32>) -> i32 {
    let mut i = 0;
    let mut last_output = 0;
    loop {
        let opcode = program[i];

        if [1, 2].contains(&(opcode % 10)) {
            let instruction = opcode % 10;
            let mode_first = opcode / 100 % 10;
            let mode_second = opcode / 1000 % 10;

            let first_operand = if mode_first == 0 {
                program[program[i + 1] as usize]
            } else {
                program[i + 1]
            };
            let second_operand = if mode_second == 0 {
                program[program[i + 2] as usize]
            } else {
                program[i + 2]
            };
            let third_operand = program[i + 3] as usize;

            if instruction == 1 {
                program[third_operand] = first_operand + second_operand;
            } else if instruction == 2 {
                program[third_operand] = first_operand * second_operand;
            }
            i += 4;
        } else if [5, 6].contains(&(opcode % 10)) {
            let instruction = opcode % 10;
            let mode_first = opcode / 100 % 10;
            let mode_second = opcode / 1000 % 10;

            let first_operand = if mode_first == 0 {
                program[program[i + 1] as usize]
            } else {
                program[i + 1]
            };
            let second_operand = if mode_second == 0 {
                program[program[i + 2] as usize]
            } else {
                program[i + 2]
            };

            if instruction == 5 {
                if first_operand != 0 {
                    i = second_operand as usize;
                } else {
                    i += 3;
                }
            } else if instruction == 6 {
                if first_operand == 0 {
                    i = second_operand as usize;
                } else {
                    i += 3;
                }
            }
        } else if [7, 8].contains(&(opcode % 10)) {
            let instruction = opcode % 10;
            let mode_first = opcode / 100 % 10;
            let mode_second = opcode / 1000 % 10;

            let first_operand = if mode_first == 0 {
                program[program[i + 1] as usize]
            } else {
                program[i + 1]
            };
            let second_operand = if mode_second == 0 {
                program[program[i + 2] as usize]
            } else {
                program[i + 2]
            };
            let third_operand = program[i + 3] as usize;

            if instruction == 7 {
                if first_operand < second_operand {
                    program[third_operand] = 1;
                } else {
                    program[third_operand] = 0;
                }
            } else if instruction == 8 {
                if first_operand == second_operand {
                    program[third_operand] = 1;
                } else {
                    program[third_operand] = 0;
                }
            }
            i += 4;
        } else {
            if opcode == 3 {
                let first_operand = program[i + 1] as usize;
                program[first_operand] = 5;
            } else if opcode == 4 {
                let mode_first = opcode / 100 % 10;
                if mode_first == 0 {
                    last_output = program[program[i + 1] as usize];
                } else {
                    last_output = program[i + 1];
                }
            } else if opcode == 99 {
                break;
            }
            i += 2;
        }
    }

    last_output
}

fn main() {
    let mut program = parse_program();
    println!("The solution to part 1 is {}.", part1(&mut program));

    let mut program = parse_program();
    println!("The solution to part 2 is {}.", part2(&mut program));
}
