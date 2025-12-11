use std::collections::{BTreeSet, HashMap};

fn parse() -> HashMap<&'static str, Vec<&'static str>> {
    include_str!("input.txt")
        .lines()
        .map(|line| {
            let (input, outputs) = line.split_once(":").unwrap();
            let outputs = outputs.split_whitespace().collect::<Vec<_>>();
            (input, outputs)
        })
        .collect()
}

fn paths(
    map: &HashMap<&'static str, Vec<&'static str>>,
    current: &'static str,
    to: &'static str,
    to_visit: BTreeSet<&'static str>,
    memo: &mut HashMap<(&'static str, BTreeSet<&'static str>), u64>,
) -> u64 {
    if current == to {
        to_visit.is_empty() as u64
    } else {
        if let Some(res) = memo.get(&(current, to_visit.clone())) {
            return *res;
        }

        let mut new_to_visit = to_visit.clone();
        if to_visit.contains(current) {
            new_to_visit.remove(current);
        }
        let res = map
            .get(current)
            .unwrap()
            .iter()
            .map(|output| paths(map, output, to, new_to_visit.clone(), memo))
            .sum();
        memo.insert((current, to_visit), res);
        res
    }
}

fn part1(map: &HashMap<&'static str, Vec<&'static str>>) -> u64 {
    paths(map, "you", "out", BTreeSet::new(), &mut HashMap::new())
}

fn part2(map: &HashMap<&'static str, Vec<&'static str>>) -> u64 {
    paths(
        map,
        "svr",
        "out",
        BTreeSet::from_iter(["dac", "fft"]),
        &mut HashMap::new(),
    )
}

fn main() {
    let map = parse();
    println!("The solution to part 1 is {}.", part1(&map));
    println!("The solution to part 2 is {}.", part2(&map));
}
