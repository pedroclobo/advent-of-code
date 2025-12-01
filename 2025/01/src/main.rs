use std::{str::FromStr, string::ParseError};

#[derive(Debug)]
enum Rotation {
    Left(i32),
    Right(i32),
}

impl FromStr for Rotation {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, amount) = s.split_at(1);
        let amount = amount.parse::<i32>().unwrap();
        match dir {
            "L" => Ok(Rotation::Left(amount)),
            "R" => Ok(Rotation::Right(amount)),
            _ => panic!("Invalid rotation"),
        }
    }
}

fn parse() -> impl Iterator<Item = Result<Rotation, ParseError>> {
    include_str!("input.txt")
        .lines()
        .map(|line| Rotation::from_str(line))
}

fn part1() -> i32 {
    let rotations = parse();

    let mut dial = 50;
    let mut sol = 0;
    for rotation in rotations {
        let rotation = rotation.unwrap();
        dial += match rotation {
            Rotation::Left(amount) => -amount,
            Rotation::Right(amount) => amount,
        };
        if dial % 100 == 0 {
            sol += 1
        }
    }

    sol
}

fn part2() -> usize {
    let rotations = parse();

    let mut dial = 50;
    let mut sol = 0;
    for rotation in rotations {
        let rotation = rotation.unwrap();
        let new_dial = dial
            + match rotation {
                Rotation::Left(amount) => -amount,
                Rotation::Right(amount) => amount,
            };
        let range = if new_dial > dial {
            dial + 1..=new_dial
        } else {
            new_dial..=dial - 1
        };
        sol += range.filter(|i| i % 100 == 0).count();
        dial = new_dial;
    }

    sol
}

fn main() {
    println!("The solution to part 1 is {}.", part1());
    println!("The solution to part 2 is {}.", part2());
}
