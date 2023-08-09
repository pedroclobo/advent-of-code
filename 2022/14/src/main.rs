use regex::Regex;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point(usize, usize);

#[derive(Debug)]
struct RockLine {
    points: HashSet<Point>,
}

impl RockLine {
    fn new(lines: Vec<Point>) -> Self {
        let mut points = HashSet::new();

        for i in 1..lines.len() {
            let (x1, y1) = (lines[i - 1].0, lines[i - 1].1);
            let (x2, y2) = (lines[i].0, lines[i].1);
            if x1 == x2 {
                for y in y1..=y2 {
                    points.insert(Point(x1, y));
                }
                for y in y2..=y1 {
                    points.insert(Point(x1, y));
                }
            } else if y1 == y2 {
                for x in x1..=x2 {
                    points.insert(Point(x, y1));
                }
                for x in x2..=x1 {
                    points.insert(Point(x, y1));
                }
            } else {
                panic!("invalid rock line");
            }
        }

        RockLine { points }
    }
}

#[derive(Debug)]
struct RockLines {
    points: HashSet<Point>,
}

impl RockLines {
    fn new(lines: Vec<RockLine>) -> Self {
        let mut points = HashSet::new();
        for line in lines {
            points.extend(line.points);
        }
        RockLines { points }
    }

    fn contains(&self, p: &Point) -> bool {
        self.points.contains(p)
    }
}

#[derive(Debug)]
struct Cave {
    rocks: RockLines,
    sand: HashSet<Point>,
    max_y: usize,
}

impl Cave {
    fn add_sand(&mut self, p: Point) {
        self.sand.insert(p);
    }

    fn contains(&self, p: &Point) -> bool {
        self.rocks.contains(p) || self.sand.contains(p)
    }

    fn drop_sand(&mut self) -> bool {
        let (mut x, mut y) = (500, 0);

        while y < self.max_y {
            if !self.contains(&Point(x, y + 1)) {
                y += 1;
            } else if !self.contains(&Point(x - 1, y + 1)) {
                x -= 1;
                y += 1;
            } else if !self.contains(&Point(x + 1, y + 1)) {
                x += 1;
                y += 1;
            } else {
                self.add_sand(Point(x, y));
                return true;
            }
        }

        false
    }

    fn drop_sand_until_limit(&mut self) {
        let (mut x, mut y) = (500, 0);

        while y + 1 < self.max_y {
            if !self.contains(&Point(x, y + 1)) {
                y += 1;
            } else if !self.contains(&Point(x - 1, y + 1)) {
                x -= 1;
                y += 1;
            } else if !self.contains(&Point(x + 1, y + 1)) {
                x += 1;
                y += 1;
            } else {
                self.add_sand(Point(x, y));
                return;
            }
        }

        self.add_sand(Point(x, y));
    }
}

fn parse() -> Cave {
    let re = Regex::new(r"(\d+),(\d+)").unwrap();

    let mut rocks = Vec::new();
    let mut max_y = 0;

    for line in include_str!("input.txt").lines() {
        let mut lines = Vec::new();
        for cap in re.captures_iter(line) {
            lines.push(Point(
                cap[1].parse::<usize>().unwrap(),
                cap[2].parse::<usize>().unwrap(),
            ));
            max_y = max_y.max(cap[2].parse::<usize>().unwrap());
        }

        rocks.push(RockLine::new(lines));
    }

    Cave {
        rocks: RockLines::new(rocks),
        sand: HashSet::new(),
        max_y,
    }
}

fn part1(cave: &mut Cave) -> u32 {
    let mut i = 0;
    while cave.drop_sand() {
        i += 1;
    }
    i
}

fn part2(cave: &mut Cave) -> u32 {
    cave.max_y += 2;
    let mut i = 0;
    while !cave.contains(&Point(500, 0)) {
        cave.drop_sand_until_limit();
        i += 1;
    }
    i
}

fn main() {
    println!("The solution to part 1 is {}.", part1(&mut parse()));
    println!("The solution to part 2 is {}.", part2(&mut parse()));
}
