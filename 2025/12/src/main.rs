#[derive(Debug)]
struct Input {
    regions: Vec<((u64, u64), Vec<u64>)>,
}

fn parse() -> Input {
    let mut regions = Vec::new();

    for split in include_str!("input.txt").split("\n\n") {
        for line in split.lines() {
            if !line.contains("x") {
                continue;
            }

            let (dims, mask) = line.split_once(":").unwrap();
            let (x, y) = dims.split_once("x").unwrap();
            let indices = mask
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect::<Vec<_>>();
            regions.push(((x.parse().unwrap(), y.parse().unwrap()), indices));
        }
    }

    Input { regions }
}

fn part1(input: &Input) -> usize {
    input
        .regions
        .iter()
        // Check if we can fit each of the presents in a 3x3 grid.
        .filter(|((x, y), presents)| x * y >= presents.iter().sum::<u64>() * 9)
        .count()
}

fn main() {
    println!("The solution to part 1 is {}.", part1(&parse()));
}
