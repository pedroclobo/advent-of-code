use std::collections::HashMap;

fn two_adjacent_digits(num: u32) -> bool {
    let s = num.to_string();

    // it is always a six digit number
    for i in 0..5 {
        if s.chars().nth(i).unwrap() == s.chars().nth(i + 1).unwrap() {
            return true;
        }
    }

    false
}

fn two_adjacent_digits_strict(num: u32) -> bool {
    let s = num.to_string();
    let mut sequences: HashMap<usize, u32> = HashMap::new();

    // map sequence's start index to size of sequence
    let mut i = 0;
    while i < 5 {
        let index = i;
        while i < 5 && s.chars().nth(i).unwrap() == s.chars().nth(i + 1).unwrap() {
            i += 1
        }
        sequences.insert(index, (i - index) as u32 + 1);
        i += 1;
    }

    // there must be at least a sequence of size 2
    for &count in sequences.values() {
        if count == 2 {
            return true;
        }
    }

    false
}

fn decreases(num: u32) -> bool {
    let s = num.to_string();

    // it is always a six digit number
    for i in 0..5 {
        if s.chars().nth(i).unwrap() > s.chars().nth(i + 1).unwrap() {
            return false;
        }
    }

    true
}

fn part1(low: u32, high: u32) -> u32 {
    let mut count = 0;

    for num in low..=high {
        if two_adjacent_digits(num) && decreases(num) {
            count += 1;
        }
    }

    count
}

fn part2(low: u32, high: u32) -> u32 {
    let mut count = 0;

    for num in low..=high {
        if two_adjacent_digits_strict(num) && decreases(num) {
            count += 1;
        }
    }

    count
}

fn main() {
    let input: Vec<u32> = include_str!("input.txt")
        .trim()
        .split("-")
        .map(|num| num.parse().unwrap())
        .collect();

    println!("The solution to part 1 is {}.", part1(input[0], input[1]));
    println!("The solution to part 2 is {}.", part2(input[0], input[1]));
}
