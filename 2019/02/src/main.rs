fn compute(program: &mut Vec<u32>) -> u32 {
    let mut pointer = 0;

    loop {
        let instruction = program[pointer];

        if instruction == 1 {
            let i1 = program[pointer + 1] as usize;
            let i2 = program[pointer + 2] as usize;
            let i3 = program[pointer + 3] as usize;
            program[i3] = program[i1] + program[i2];
        } else if instruction == 2 {
            let i1 = program[pointer + 1] as usize;
            let i2 = program[pointer + 2] as usize;
            let i3 = program[pointer + 3] as usize;
            program[i3] = program[i1] * program[i2];
        } else if instruction == 99 {
            break;
        }

        pointer += 4
    }

    program[0]
}

fn parse_program() -> Vec<u32> {
    include_str!("input.txt")
        .split(",")
        .map(|int| int.trim().parse().unwrap())
        .collect()
}

fn part1(program: &mut Vec<u32>) -> u32 {
    program[1] = 12;
    program[2] = 2;

    compute(program)
}

fn part2() -> Option<u32> {
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut program = parse_program();
            program[1] = noun;
            program[2] = verb;
            if compute(&mut program) == 19690720 {
                return Some(100 * noun + verb);
            }
        }
    }

    None
}

fn main() {
    let mut program = parse_program();

    println!("The solution to part 1 is {}.", part1(&mut program));
    println!("The solution to part 2 is {}.", part2().unwrap());
}
