use std::fmt::Debug;
use std::{collections::HashMap, str::FromStr, string::ParseError};

#[derive(Debug)]
enum Instruction {
    Left,
    Right,
}

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone, PartialOrd, Ord)]
struct NodeID(u16);

impl FromStr for NodeID {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        assert_eq!(s.len(), 3);
        let mut chars = s.chars();
        let mut id = (chars.next().unwrap() as u8 - b'A') as u16;
        id = 26 * id + (chars.next().unwrap() as u8 - b'A') as u16;
        id = 26 * id + (chars.next().unwrap() as u8 - b'A') as u16;
        Ok(NodeID(id))
    }
}

impl NodeID {
    fn is_start(&self) -> bool {
        self.0 % 26 == 0
    }

    fn is_end(&self) -> bool {
        self.0 % 26 == 25
    }
}

#[derive(Debug)]
struct Network {
    instructions: Vec<Instruction>,
    map: HashMap<NodeID, [NodeID; 2]>,
}

fn parse() -> Network {
    let (instructions, map) = include_str!("input.txt").split_once("\n\n").unwrap();

    let instructions = instructions
        .chars()
        .map(|c| match c {
            'L' => Instruction::Left,
            'R' => Instruction::Right,
            _ => panic!("Invalid input"),
        })
        .collect();

    let map = map
        .lines()
        .map(|line| {
            let (start, neighbors) = line.split_once("=").unwrap();

            let start = NodeID::from_str(start.trim()).unwrap();
            let (left, right) = neighbors
                .replace("(", "")
                .replace(")", "")
                .split_once(",")
                .map(|(s1, s2)| {
                    (
                        NodeID::from_str(s1.trim()).unwrap(),
                        NodeID::from_str(s2.trim()).unwrap(),
                    )
                })
                .unwrap();

            (start, [left, right])
        })
        .collect();

    Network { instructions, map }
}

fn part1(network: &Network, start: NodeID) -> usize {
    let mut cur = start;
    let mut instructions = network.instructions.iter().cycle();
    let mut steps = 0;

    loop {
        let instruction = instructions.next().unwrap();
        cur = match instruction {
            Instruction::Left => network.map[&cur][0],
            Instruction::Right => network.map[&cur][1],
        };
        steps += 1;
        if cur.is_end() {
            break;
        }
    }

    steps
}

fn part2(network: &Network) -> usize {
    fn lcm(a: usize, b: usize) -> usize {
        let gcd = |mut a: usize, mut b: usize| {
            while b != 0 {
                (a, b) = (b, a % b);
            }
            a
        };

        if a == 0 || b == 0 {
            return 0;
        }
        a / gcd(a, b) * b
    }

    network
        .map
        .keys()
        .filter(|id| id.is_start())
        .map(|start_id| part1(network, *start_id))
        .fold(1, lcm)
}

fn main() {
    let network = parse();
    println!(
        "The solution to part 1 is {}.",
        part1(&network, NodeID::from_str("AAA").unwrap())
    );
    println!("The solution to part 2 is {}.", part2(&network));
}
