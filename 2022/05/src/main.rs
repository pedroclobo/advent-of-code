use regex::Regex;

fn parse() -> (Vec<Vec<char>>, Vec<(u32, usize, usize)>) {
    let input: Vec<&str> = include_str!("input.txt").split("\n\n").collect();

    let number_stacks: usize = input[0]
        .split("\n")
        .into_iter()
        .last()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let index_to_stack = |i: usize| -> usize { (i + 3) / 4 - 1 };

    let mut stacks: Vec<Vec<char>> = Vec::new();
    stacks.resize(number_stacks, Vec::<char>::new());

    for line in input[0].split("\n").into_iter() {
        for (i, c) in line.chars().enumerate() {
            if c.is_uppercase() {
                stacks[index_to_stack(i)].push(c);
            }
        }
    }

    // ouch
    for i in 0..stacks.len() {
        stacks[i] = stacks[i].clone().into_iter().rev().collect();
    }

    let mut instructions = Vec::new();

    let re = Regex::new(r"\d+").unwrap();
    for line in input[1].split("\n").into_iter() {
        if line.is_empty() {
            continue;
        }
        let caps: Vec<u32> = re
            .find_iter(line)
            .filter_map(|digits| digits.as_str().parse().ok())
            .collect();

        instructions.push((caps[0], caps[1] as usize, caps[2] as usize));
    }

    (stacks, instructions)
}

fn part1() -> String {
    let (mut stacks, instructions) = parse();

    for (n, from, to) in instructions {
        for _ in 0..n {
            let el = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(el);
        }
    }

    stacks
        .into_iter()
        .flat_map(|stack| stack.into_iter().last())
        .collect::<String>()
}

fn part2() -> String {
    let (mut stacks, instructions) = parse();

    for (n, from, to) in instructions {
        let mut els = Vec::new();
        for _ in 0..n {
            let el = stacks[from - 1].pop().unwrap();
            els.push(el);
        }
        for el in els.into_iter().rev() {
            stacks[to - 1].push(el);
        }
    }

    stacks
        .into_iter()
        .flat_map(|stack| stack.into_iter().last())
        .collect::<String>()
}

fn main() {
    println!("The solution to part 1 is {}.", part1());
    println!("The solution to part 2 is {}.", part2());
}
