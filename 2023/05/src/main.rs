use std::ops::RangeInclusive;

#[derive(Debug)]
struct Range {
    source: usize,
    destination: usize,
    length: usize,
}

impl Range {
    fn source_range(&self) -> RangeInclusive<usize> {
        self.source..=self.source + self.length
    }

    fn map(&self, val: usize) -> Option<usize> {
        if self.source_range().contains(&val) {
            Some(self.destination + (val - self.source))
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Map {
    ranges: Vec<Range>,
}

impl Map {
    fn map(&self, val: usize) -> usize {
        for range in &self.ranges {
            if let Some(v) = range.map(val) {
                return v;
            }
        }
        val
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<usize>,
    maps: Vec<Map>,
}

impl Almanac {
    fn to_location(&self, mut seed: usize) -> usize {
        for map in &self.maps {
            seed = map.map(seed);
        }
        seed
    }
}

fn parse() -> Almanac {
    let input = include_str!("input.txt");

    let seeds_line = input.lines().next().unwrap();
    let seeds = seeds_line["seeds:".len()..]
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut maps = Vec::new();
    for block in input.split("\n\n").skip(1) {
        let mut lines = block.lines();
        lines.next();

        let mut ranges = Vec::new();

        for line in lines {
            if line.trim().is_empty() {
                continue;
            }
            let nums = line
                .split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            ranges.push(Range {
                source: nums[1],
                destination: nums[0],
                length: nums[2],
            });
        }

        maps.push(Map { ranges });
    }

    Almanac { seeds, maps }
}

fn part1(almanac: &Almanac) -> usize {
    almanac
        .seeds
        .iter()
        .map(|seed| almanac.to_location(*seed))
        .min()
        .unwrap()
}

fn part2(almanac: &Almanac) -> usize {
    almanac
        .seeds
        .chunks(2)
        .flat_map(|chunk| {
            let [v1, v2] = chunk else {
                panic!("expected pairs")
            };
            *v1..=(v1 + v2)
        })
        .map(|seed| almanac.to_location(seed))
        .min()
        .unwrap()
}

fn main() {
    let almanac = parse();
    println!("The solution to part 1 is {}.", part1(&almanac));
    println!("The solution to part 2 is {}.", part2(&almanac));
}
