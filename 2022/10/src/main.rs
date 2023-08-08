#[derive(Debug)]
enum InstructionKind {
    Nop,
    Add,
}

#[derive(Debug)]
struct Instruction {
    opcode: InstructionKind,
    arg: Option<i32>,
}

impl Instruction {
    fn new(opcode: InstructionKind, arg: Option<i32>) -> Self {
        Self { opcode, arg }
    }
}

fn parse() -> Vec<Instruction> {
    include_str!("input.txt")
        .lines()
        .into_iter()
        .map(|line| {
            let mut iter = line.split_whitespace();
            let opcode = match iter.next().unwrap() {
                "noop" => InstructionKind::Nop,
                "addx" => InstructionKind::Add,
                _ => panic!("invalid instruction"),
            };
            let arg = match iter.next() {
                Some(arg) => Some(arg.parse::<i32>().unwrap()),
                None => None,
            };

            Instruction::new(opcode, arg)
        })
        .collect()
}

fn part1(instructions: &Vec<Instruction>) -> i32 {
    let mut cycle = 1;
    let mut x = 1;
    let mut x_values = Vec::new();

    let is_important_cycle = |cycle| -> bool {
        let mut cycle = cycle - 20;

        while cycle > 0 {
            cycle -= 40;
        }

        cycle == 0
    };

    for instruction in instructions {
        match instruction.opcode {
            InstructionKind::Nop => {
                cycle += 1;
            }
            InstructionKind::Add => {
                cycle += 1;
                if is_important_cycle(cycle) {
                    x_values.push(cycle * x);
                }

                x += instruction.arg.expect("arg expected");
                cycle += 1;
            }
        }
        if is_important_cycle(cycle) {
            x_values.push(cycle * x);
        }
    }

    x_values.into_iter().sum()
}

fn part2(instructions: &Vec<Instruction>) -> String {
    let mut crt = ['.'; 40 * 6];

    let mut sprite_pos = [0, 1, 2];

    let mut cycle = 1;
    let mut x = 1;

    for instruction in instructions {
        if sprite_pos.contains(&(&(cycle - 1) % 40)) {
            crt[cycle as usize - 1] = '■';
        } else {
            crt[cycle as usize - 1] = '.';
        }

        match instruction.opcode {
            InstructionKind::Nop => {
                cycle += 1;
            }
            InstructionKind::Add => {
                cycle += 1;

                if sprite_pos.contains(&(&(cycle - 1) % 40)) {
                    crt[cycle as usize - 1] = '■';
                } else {
                    crt[cycle as usize - 1] = '.';
                }

                x += instruction.arg.expect("arg expected");
                sprite_pos = [x - 1, x, x + 1];

                cycle += 1;
            }
        }
    }

    let mut answer = String::new();
    for (i, c) in crt.into_iter().enumerate() {
        if i % 40 == 0 {
            answer.push('\n');
        }
        answer.push(c);
    }

    answer
}

fn main() {
    let instructions = parse();
    println!("The solution to part 1 is {}.", part1(&instructions));
    println!("The solution to part 2 is {}.", part2(&instructions));
}
