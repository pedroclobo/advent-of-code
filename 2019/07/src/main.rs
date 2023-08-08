use itertools::Itertools;

fn parse_program() -> Vec<i32> {
    let program: Vec<i32> = include_str!("input.txt")
        .split(",")
        .map(|int| int.trim().parse().unwrap())
        .collect();

    program
}

fn compute(phase_setting: i32, input_signal: i32) -> i32 {
    let mut program = parse_program();
    let mut i = 0;
    let mut last_output = 0;

    let mut has_phase = false;

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
                if !has_phase {
                    program[first_operand] = phase_setting;
                    has_phase = true;
                } else {
                    program[first_operand] = input_signal;
                }
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

fn compute_and_wait(program: &mut Vec<i32>, input: &i32, output: &mut i32, pc: &mut usize) -> bool {
    let mut waiting_input = false;

    loop {
        let opcode = program[*pc];

        if [1, 2].contains(&(opcode % 10)) {
            let instruction = opcode % 10;
            let mode_first = opcode / 100 % 10;
            let mode_second = opcode / 1000 % 10;

            let first_operand = if mode_first == 0 {
                program[program[*pc + 1] as usize]
            } else {
                program[*pc + 1]
            };
            let second_operand = if mode_second == 0 {
                program[program[*pc + 2] as usize]
            } else {
                program[*pc + 2]
            };
            let third_operand = program[*pc + 3] as usize;

            if instruction == 1 {
                program[third_operand] = first_operand + second_operand;
            } else if instruction == 2 {
                program[third_operand] = first_operand * second_operand;
            }
            *pc += 4;
        } else if [5, 6].contains(&(opcode % 10)) {
            let instruction = opcode % 10;
            let mode_first = opcode / 100 % 10;
            let mode_second = opcode / 1000 % 10;

            let first_operand = if mode_first == 0 {
                program[program[*pc + 1] as usize]
            } else {
                program[*pc + 1]
            };
            let second_operand = if mode_second == 0 {
                program[program[*pc + 2] as usize]
            } else {
                program[*pc + 2]
            };

            if instruction == 5 {
                if first_operand != 0 {
                    *pc = second_operand as usize;
                } else {
                    *pc += 3;
                }
            } else if instruction == 6 {
                if first_operand == 0 {
                    *pc = second_operand as usize;
                } else {
                    *pc += 3;
                }
            }
        } else if [7, 8].contains(&(opcode % 10)) {
            let instruction = opcode % 10;
            let mode_first = opcode / 100 % 10;
            let mode_second = opcode / 1000 % 10;

            let first_operand = if mode_first == 0 {
                program[program[*pc + 1] as usize]
            } else {
                program[*pc + 1]
            };
            let second_operand = if mode_second == 0 {
                program[program[*pc + 2] as usize]
            } else {
                program[*pc + 2]
            };
            let third_operand = program[*pc + 3] as usize;

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
            *pc += 4;
        } else {
            if opcode == 3 {
                if !waiting_input {
                    let first_operand = program[*pc + 1] as usize;
                    program[first_operand] = *input;
                    waiting_input = true;
                    *pc += 2;
                } else {
                    return false;
                }
            } else if opcode == 4 {
                let mode_first = opcode / 100 % 10;
                if mode_first == 0 {
                    *output = program[program[*pc + 1] as usize];
                } else {
                    *output = program[*pc + 1];
                }
                *pc += 2;
                return false;
            } else if opcode == 99 {
                return true;
            }
        }
    }
}

fn amplifier(phases: &Vec<i32>) -> i32 {
    let mut output = 0;

    for i in 0..5 {
        output = compute(phases[i], output);
    }

    output
}

fn feedback_amplifier(phases: &Vec<i32>) -> i32 {
    let mut programs = vec![parse_program(); 5];
    let mut program_counters: Vec<usize> = vec![0; 5];
    let mut halted = vec![false; 5];

    for i in 0..5 {
        let mut output = 0;
        halted[i] = compute_and_wait(
            &mut programs[i],
            &phases[i],
            &mut output,
            &mut program_counters[i],
        );
    }

    let mut prev_output = 0;
    let mut output = 0;
    while !halted[4] {
        for i in 0..5 {
            halted[i] = compute_and_wait(
                &mut programs[i],
                &prev_output,
                &mut output,
                &mut program_counters[i],
            );

            prev_output = output;
        }
    }

    prev_output
}

fn part1() -> i32 {
    let mut maximum = i32::MIN;

    for perm in (0..=4).permutations(5) {
        let signal = amplifier(&perm);
        if signal > maximum {
            maximum = signal;
        }
    }

    maximum
}

fn part2() -> i32 {
    let mut maximum = i32::MIN;

    for perm in (5..=9).permutations(5) {
        let signal = feedback_amplifier(&perm);
        if signal > maximum {
            maximum = signal;
        }
    }

    maximum
}

fn main() {
    println!("The solution to part 1 is {}.", part1());
    println!("The solution to part 2 is {}.", part2());
}
