use std::fmt::Debug;
use std::hash::Hash;
use std::{collections::HashMap, str::FromStr, string::ParseError};

#[derive(Debug, Eq, PartialEq, Hash, PartialOrd, Ord, Clone)]
enum CardP1 {
    A,
    K,
    Q,
    J,
    T,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

impl Card for CardP1 {
    fn rank(cards: &[Self; 5]) -> Type {
        let mut freqs = HashMap::new();
        for card in cards {
            *freqs.entry(card).or_insert(0) += 1;
        }

        let mut counts: Vec<_> = freqs.values().collect();
        counts.sort_unstable_by(|a, b| b.cmp(a));

        match counts.as_slice() {
            [5] => Type::FiveOfAKind,
            [4, 1] => Type::FourOfAKind,
            [3, 2] => Type::FullHouse,
            [3, 1, 1] => Type::ThreeOfAKind,
            [2, 2, 1] => Type::TwoPair,
            [2, 1, 1, 1] => Type::OnePair,
            [1, 1, 1, 1, 1] => Type::HighCard,
            _ => unreachable!("invalid hand"),
        }
    }

    fn mutated_hand(cards: &[Self; 5]) -> [Self; 5] {
        cards.clone()
    }
}

impl FromStr for CardP1 {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::A),
            "K" => Ok(Self::K),
            "Q" => Ok(Self::Q),
            "J" => Ok(Self::J),
            "T" => Ok(Self::T),
            "9" => Ok(Self::Nine),
            "8" => Ok(Self::Eight),
            "7" => Ok(Self::Seven),
            "6" => Ok(Self::Six),
            "5" => Ok(Self::Five),
            "4" => Ok(Self::Four),
            "3" => Ok(Self::Three),
            "2" => Ok(Self::Two),
            _ => panic!("Invalid input"),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, PartialOrd, Ord, Clone)]
enum CardP2 {
    A,
    K,
    Q,
    T,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Joker,
}

impl Card for CardP2 {
    fn rank(cards: &[Self; 5]) -> Type {
        let cards = Self::mutated_hand(cards);

        let mut freqs = HashMap::new();
        for card in cards {
            *freqs.entry(card).or_insert(0) += 1;
        }

        let mut counts: Vec<_> = freqs.values().collect();
        counts.sort_unstable_by(|a, b| b.cmp(a));

        match counts.as_slice() {
            [5] => Type::FiveOfAKind,
            [4, 1] => Type::FourOfAKind,
            [3, 2] => Type::FullHouse,
            [3, 1, 1] => Type::ThreeOfAKind,
            [2, 2, 1] => Type::TwoPair,
            [2, 1, 1, 1] => Type::OnePair,
            [1, 1, 1, 1, 1] => Type::HighCard,
            _ => unreachable!("invalid hand"),
        }
    }

    fn mutated_hand(cards: &[Self; 5]) -> [Self; 5] {
        let mut freqs = HashMap::new();
        for card in cards {
            *freqs.entry(card).or_insert(0) += 1;
        }

        let (card, _) = freqs
            .into_iter()
            .filter(|(c, _)| !matches!(c, CardP2::Joker))
            .max_by(|(_, f1), (_, f2)| f1.cmp(f2))
            .unwrap_or((&CardP2::A, 5)); // we got five jokers

        cards
            .clone()
            .into_iter()
            .map(|c| {
                if let CardP2::Joker = c {
                    card.clone()
                } else {
                    c
                }
            })
            .collect::<Vec<_>>()
            .try_into()
            .expect("A hand should have 5 cards")
    }
}

impl FromStr for CardP2 {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::A),
            "K" => Ok(Self::K),
            "Q" => Ok(Self::Q),
            "T" => Ok(Self::T),
            "9" => Ok(Self::Nine),
            "8" => Ok(Self::Eight),
            "7" => Ok(Self::Seven),
            "6" => Ok(Self::Six),
            "5" => Ok(Self::Five),
            "4" => Ok(Self::Four),
            "3" => Ok(Self::Three),
            "2" => Ok(Self::Two),
            "J" => Ok(Self::Joker),
            _ => panic!("Invalid input"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand<C> {
    cards: [C; 5],
    bid: u32,
}

trait Card: Clone + Eq + PartialEq + PartialOrd + Ord + Clone + Debug + Hash {
    fn rank(cards: &[Self; 5]) -> Type;
    fn mutated_hand(cards: &[Self; 5]) -> [Self; 5];
}

impl<C: Card> Hand<C> {
    fn rank(&self) -> Type {
        C::rank(&self.cards)
    }
}

impl<C: Card> PartialOrd for Hand<C> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<C: Card> Ord for Hand<C> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.rank(), &self.cards).cmp(&(other.rank(), &other.cards))
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum Type {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

fn parse_part1() -> Vec<Hand<CardP1>> {
    include_str!("input.txt")
        .lines()
        .map(|line| {
            let (cards, bid) = line.split_once(" ").unwrap();
            let bid = bid.parse().unwrap();
            let cards = cards
                .chars()
                .map(|card| CardP1::from_str(&card.to_string()).unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .expect("A hand should have 5 cards");
            Hand { cards, bid }
        })
        .collect()
}

fn part1() -> u32 {
    let mut hands = parse_part1();
    hands.sort_unstable_by(|a, b| b.cmp(a));

    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) as u32 * hand.bid)
        .sum()
}

fn parse_part2() -> Vec<Hand<CardP2>> {
    include_str!("input.txt")
        .lines()
        .map(|line| {
            let (cards, bid) = line.split_once(" ").unwrap();
            let bid = bid.parse().unwrap();
            let cards = cards
                .chars()
                .map(|card| CardP2::from_str(&card.to_string()).unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .expect("A hand should have 5 cards");
            Hand { cards, bid }
        })
        .collect()
}

fn part2() -> u32 {
    let mut hands = parse_part2();
    hands.sort_unstable_by(|a, b| b.cmp(a));

    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) as u32 * hand.bid)
        .sum()
}

fn main() {
    println!("The solution to part 1 is {}.", part1());
    println!("The solution to part 2 is {}.", part2());
}
