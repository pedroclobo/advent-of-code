use std::ops::RangeInclusive;

#[derive(Debug)]
struct Database {
    ranges: Vec<RangeInclusive<u64>>,
    ids: Vec<u64>,
}

fn parse() -> Database {
    let (ranges, ids) = include_str!("input.txt").split_once("\n\n").unwrap();
    let ranges = ranges
        .lines()
        .map(|line| {
            let (n1, n2) = line.split_once("-").unwrap();
            n1.parse().unwrap()..=n2.parse().unwrap()
        })
        .collect();
    let ids = ids
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .collect();

    Database { ranges, ids }
}

fn part1(db: &Database) -> usize {
    db.ids
        .iter()
        .filter(|id| db.ranges.iter().any(|range| range.contains(id)))
        .count()
}

fn part2(db: Database) -> u64 {
    let mut ranges = db.ranges.into_iter().map(Some).collect::<Vec<_>>();
    let len = ranges.len();

    loop {
        let mut changed = false;
        for i in 0..len {
            for j in i + 1..len {
                if let (Some(r1), Some(r2)) = (&ranges[i], &ranges[j]) {
                    let (r1, r2) = if r1.start() >= r2.end() {
                        (r2, r1)
                    } else {
                        (r1, r2)
                    };
                    if r1.end() >= r2.start() {
                        ranges[i] = Some(*r1.start().min(r2.start())..=(*r1.end().max(r2.end())));
                        ranges[j] = None;
                        changed = true;
                    }
                }
            }
        }
        if !changed {
            break;
        }
    }

    ranges
        .into_iter()
        .flatten()
        .map(|range| range.end() - range.start() + 1)
        .sum()
}

fn main() {
    let db = parse();
    println!("The solution to part 1 is {}.", part1(&db));
    println!("The solution to part 2 is {}.", part2(db));
}
