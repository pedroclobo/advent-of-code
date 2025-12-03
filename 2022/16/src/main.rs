use regex::Regex;
use std::{
    collections::{BTreeSet, HashMap},
    str::FromStr,
    string::ParseError,
};

struct Valve {
    id: ValveID,
    flow_rate: usize,
    neighbors: Vec<ValveID>,
}

impl FromStr for Valve {
    type Err = ParseError;

    // "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB"
    // "Valve HH has flow rate=22; tunnel leads to valve GG"
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(
            r"Valve ([A-Z]+) has flow rate=(\d+); tunnels? leads? to valves? ([A-Z, ]+)$",
        )
        .unwrap();

        let caps = re.captures(s).unwrap();
        let name = ValveID::from(&caps[1]);
        let flow_rate = caps[2].parse().unwrap();
        let neighbors = caps[3]
            .split(", ")
            .map(|s| ValveID::from(s))
            .collect::<Vec<_>>();

        Ok(Self {
            id: name,
            flow_rate,
            neighbors,
        })
    }
}

fn parse() -> impl Iterator<Item = Valve> {
    include_str!("input.txt")
        .lines()
        .map(|line| Valve::from_str(line).unwrap())
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy)]
struct ValveID(pub usize);

impl From<&str> for ValveID {
    fn from(name: &str) -> Self {
        assert_eq!(name.len(), 2);

        let mut chars = name.chars();
        let first = chars.next().unwrap();
        let second = chars.next().unwrap();
        let id =
            ((first as usize - b'A' as usize) * 26 + (second as usize - b'A' as usize)) as usize;
        ValveID(id)
    }
}

const N: usize = 26 * 26;

struct Network {
    valves: HashMap<ValveID, Valve>,
    distances: [[Option<u8>; N]; N],
}

impl Network {
    fn new() -> Network {
        let valves = parse()
            .map(|valve| (valve.id, valve))
            .collect::<HashMap<ValveID, Valve>>();
        let distances = Self::floyd_warshall(&valves);
        Self { valves, distances }
    }

    fn split(&self) -> Vec<(BTreeSet<ValveID>, BTreeSet<ValveID>)> {
        let non_zero_valves = self
            .valves
            .iter()
            .filter(|(_, valve)| valve.flow_rate != 0)
            .map(|(id, _)| *id)
            .collect::<Vec<_>>();

        let n = non_zero_valves.len();
        let total_subsets = 1 << n;

        let mut splits = Vec::new();

        for mask in 1..total_subsets / 2 {
            let mut set1 = BTreeSet::new();
            let mut set2 = BTreeSet::new();

            for (i, &valve_id) in non_zero_valves.iter().enumerate() {
                if mask & (1 << i) != 0 {
                    set1.insert(valve_id);
                } else {
                    set2.insert(valve_id);
                }
            }

            splits.push((set1, set2));
        }

        splits
    }

    fn floyd_warshall(valves: &HashMap<ValveID, Valve>) -> [[Option<u8>; N]; N] {
        let mut distances = [[None; N]; N];

        let valves = valves.values();
        for valve in 0..N {
            distances[valve][valve] = Some(0);
        }
        for valve in valves {
            let valve_id = valve.id.0;
            for neighbor_id in &valve.neighbors {
                distances[valve_id][neighbor_id.0] = Some(1);
            }
        }

        for k in 0..N {
            for i in 0..N {
                for j in 0..N {
                    if let (Some(ik), Some(kj)) = (distances[i][k], distances[k][j]) {
                        let d = ik + kj;
                        if distances[i][j].is_none() || distances[i][j].unwrap() > d {
                            distances[i][j] = Some(d);
                        }
                    }
                }
            }
        }

        distances
    }
}

fn simulate(
    network: &Network,
    paritition: &BTreeSet<ValveID>,
    current: ValveID,
    open: BTreeSet<ValveID>,
    remaining: usize,
    memo: &mut HashMap<(ValveID, BTreeSet<ValveID>, usize), usize>,
) -> usize {
    if remaining == 0 {
        return 0;
    }
    if let Some(flow) = memo.get(&(current, open.clone(), remaining)) {
        return *flow;
    }

    let open_flow = if !open.contains(&current)
        && paritition.contains(&current)
        && network.valves[&current].flow_rate != 0
    {
        let mut new_open = open.clone();
        new_open.insert(current);
        network.valves[&current].flow_rate * (remaining - 1)
            + simulate(network, paritition, current, new_open, remaining - 1, memo)
    } else {
        0
    };

    let move_flow = network
        .valves
        .iter()
        .filter(|(id, valve)| {
            **id != current
                && !open.contains(&id)
                && paritition.contains(&id)
                && valve.flow_rate > 0
        })
        .map(|(_, valve)| {
            let distance = network.distances[current.0][valve.id.0].unwrap();
            simulate(
                network,
                paritition,
                valve.id,
                open.clone(),
                if remaining > distance.into() {
                    remaining - distance as usize
                } else {
                    0
                },
                memo,
            )
        })
        .max()
        .unwrap_or(0);

    let result = open_flow.max(move_flow);
    memo.insert((current, open.clone(), remaining), result);
    result
}

fn part1(network: &Network) -> usize {
    let all_valves = network
        .valves
        .iter()
        .filter(|(_, v)| v.flow_rate > 0)
        .map(|(id, _)| *id)
        .collect::<BTreeSet<_>>();

    simulate(
        network,
        &all_valves,
        ValveID::from("AA"),
        BTreeSet::new(),
        30,
        &mut HashMap::new(),
    )
}

fn part2(network: &Network, splits: Vec<(BTreeSet<ValveID>, BTreeSet<ValveID>)>) -> usize {
    splits
        .into_iter()
        .map(|(set1, set2)| {
            simulate(
                network,
                &set1,
                ValveID::from("AA"),
                BTreeSet::new(),
                26,
                &mut HashMap::new(),
            ) + simulate(
                network,
                &set2,
                ValveID::from("AA"),
                BTreeSet::new(),
                26,
                &mut HashMap::new(),
            )
        })
        .max()
        .unwrap()
}

fn main() {
    let network = Network::new();

    println!("The solution to part 1 is {}.", part1(&network));
    println!(
        "The solution to part 2 is {}.",
        part2(&network, network.split())
    );
}
