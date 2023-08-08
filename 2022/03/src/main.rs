fn parse_part1() -> Vec<(Vec<char>, Vec<char>)> {
    include_str!("input.txt")
        .lines()
        .map(|line| {
            let (left, right) = line.split_at(line.len() / 2);
            (left.chars().collect(), right.chars().collect())
        })
        .collect()
}

fn get_priority(token: char) -> u32 {
    if 'A' <= token && token <= 'Z' {
        token as u32 - 38
    } else {
        token as u32 - 96
    }
}

fn part1() -> u32 {
    let rucksacks = parse_part1();

    let mut sum = 0;
    for rucksack in rucksacks {
        let (first_compartment, second_compartment) = rucksack;
        let token = first_compartment
            .into_iter()
            .find(|token| second_compartment.contains(token))
            .unwrap();

        sum += get_priority(token);
    }

    sum
}

fn parse_part2() -> Vec<(Vec<char>, Vec<char>, Vec<char>)> {
    let mut lines = include_str!("input.txt").lines();
    let mut groups = Vec::new();

    while let [Some(a), Some(b), Some(c)] = [lines.next(), lines.next(), lines.next()] {
        groups.push((
            a.chars().collect(),
            b.chars().collect(),
            c.chars().collect(),
        ));
    }

    groups
}

fn part2() -> u32 {
    let groups = parse_part2();

    let mut sum = 0;
    for (a, b, c) in &groups {
        let token = a
            .into_iter()
            .find(|token| b.contains(token) && c.contains(token))
            .unwrap();

        sum += get_priority(*token);
    }

    sum
}

fn main() {
    println!("The solution to part 1 is {}.", part1());
    println!("The solution to part 2 is {}.", part2());
}
