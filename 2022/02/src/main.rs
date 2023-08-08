const ROCK_SCORE: u32 = 1;
const PAPER_SCORE: u32 = 2;
const SCISSORS_SCORE: u32 = 3;

#[derive(PartialEq, Debug)]
struct Item {
    kind: ItemKind,
    score: u32,
}

impl Item {
    fn new(kind: ItemKind) -> Self {
        match kind {
            ItemKind::Rock => Item {
                kind,
                score: ROCK_SCORE,
            },
            ItemKind::Paper => Item {
                kind,
                score: PAPER_SCORE,
            },
            ItemKind::Scissors => Item {
                kind,
                score: SCISSORS_SCORE,
            },
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum ItemKind {
    Rock,
    Paper,
    Scissors,
}

fn parse_part1() -> Vec<(Item, Item)> {
    include_str!("input.txt")
        .lines()
        .into_iter()
        .map(|line| {
            let mut iter = line.split(" ");
            let first_item = match iter.next().unwrap() {
                "A" => Item::new(ItemKind::Rock),
                "B" => Item::new(ItemKind::Paper),
                "C" => Item::new(ItemKind::Scissors),
                _ => panic!("invalid input"),
            };
            let second_item = match iter.next().unwrap() {
                "X" => Item::new(ItemKind::Rock),
                "Y" => Item::new(ItemKind::Paper),
                "Z" => Item::new(ItemKind::Scissors),
                _ => panic!("invalid input"),
            };
            (first_item, second_item)
        })
        .collect()
}

fn parse_part2() -> Vec<(Item, char)> {
    include_str!("input.txt")
        .lines()
        .into_iter()
        .map(|line| {
            let mut iter = line.split(" ");
            match iter.next().unwrap() {
                "A" => (
                    Item::new(ItemKind::Rock),
                    iter.next().unwrap().chars().next().unwrap(),
                ),
                "B" => (
                    Item::new(ItemKind::Paper),
                    iter.next().unwrap().chars().next().unwrap(),
                ),
                "C" => (
                    Item::new(ItemKind::Scissors),
                    iter.next().unwrap().chars().next().unwrap(),
                ),
                _ => panic!("invalid input"),
            }
        })
        .collect()
}

fn get_score(play: &(Item, Item)) -> u32 {
    let (elf, me) = play;
    let play_score = match (elf.kind, me.kind) {
        (ItemKind::Paper, ItemKind::Rock) => 0,
        (ItemKind::Scissors, ItemKind::Paper) => 0,
        (ItemKind::Rock, ItemKind::Scissors) => 0,
        (ItemKind::Rock, ItemKind::Rock) => 3,
        (ItemKind::Paper, ItemKind::Paper) => 3,
        (ItemKind::Scissors, ItemKind::Scissors) => 3,
        (ItemKind::Paper, ItemKind::Scissors) => 6,
        (ItemKind::Scissors, ItemKind::Rock) => 6,
        (ItemKind::Rock, ItemKind::Paper) => 6,
    };

    play_score + me.score
}

fn part1() -> u32 {
    let plays = parse_part1();

    plays.iter().map(|play| get_score(play)).sum()
}

fn part2() -> u32 {
    let plays = parse_part2();

    let mut score = 0;
    for play in plays {
        match play.1 {
            'X' => match play.0.kind {
                ItemKind::Rock => score += 0 + SCISSORS_SCORE,
                ItemKind::Paper => score += 0 + ROCK_SCORE,
                ItemKind::Scissors => score += 0 + PAPER_SCORE,
            },
            'Y' => score += 3 + play.0.score,
            'Z' => match play.0.kind {
                ItemKind::Rock => score += 6 + PAPER_SCORE,
                ItemKind::Paper => score += 6 + SCISSORS_SCORE,
                ItemKind::Scissors => score += 6 + ROCK_SCORE,
            },
            _ => panic!("invalid input"),
        }
    }

    score
}

fn main() {
    println!("The solution to part 1 is {}.", part1());
    println!("The solution to part 2 is {}.", part2());
}
