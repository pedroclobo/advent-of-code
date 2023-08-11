use range_set::RangeSet;
use regex::Regex;
use std::collections::BTreeSet;

const P1_Y: i64 = 2000000;
const P2_SIDE: i64 = 4000000;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Point(i64, i64);

impl Point {
    fn distance(&self, other: &Point) -> i64 {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }

    fn points_in_row_at_distance(&self, row: i64, distance: i64) -> std::ops::RangeInclusive<i64> {
        let dy = (self.1 - row).abs();
        let dx = distance - dy;
        self.0 - dx..=self.0 + dx
    }

    fn border(&self, distance: i64) -> Border {
        let up = (self.0, self.1 - distance);
        let left = (self.0 - distance, self.1);
        let down = (self.0, self.1 + distance);

        Border {
            lines: vec![
                // up -> right
                Line {
                    start: up,
                    slope: (1, 1),
                    interval: distance,
                },
                // left -> up
                Line {
                    start: left,
                    slope: (1, -1),
                    interval: distance,
                },
                // left -> down
                Line {
                    start: left,
                    slope: (1, 1),
                    interval: distance,
                },
                // down -> right
                Line {
                    start: down,
                    slope: (1, -1),
                    interval: distance,
                },
            ],
        }
    }

    fn in_square(&self) -> bool {
        0 <= self.0 && self.0 <= P2_SIDE && 0 <= self.1 && self.1 <= P2_SIDE
    }

    fn tuning_frequency(&self) -> i64 {
        self.0 * 4000000 + self.1
    }
}

#[derive(Debug)]
struct Line {
    start: (i64, i64),
    slope: (i64, i64),
    interval: i64,
}

impl Line {
    fn intersection(&self, other: &Line) -> Option<Point> {
        let (x0, y0) = self.start;
        let (sx, sy) = self.slope;

        let (_x0, _y0) = other.start;
        let (_sx, _sy) = other.slope;

        if self.slope == other.slope || (sx, sy) == (-_sx, -_sy) {
            return None;
        }

        let _n = (y0 + ((_x0 - x0) * sy / sx) - _y0) / (_sy - (_sx * sy) / sx);
        let n = (_x0 + _n * _sx - x0) / sx;

        if n <= self.interval && _n <= other.interval {
            return Some(Point(x0 + n * sx, y0 + n * sy));
        }

        None
    }
}

#[derive(Debug)]
struct Border {
    lines: Vec<Line>,
}

impl Border {
    fn intersection(&self, other: &Border) -> BTreeSet<Point> {
        let mut points = BTreeSet::new();

        for line in &self.lines {
            for other_line in &other.lines {
                if let Some(point) = line.intersection(other_line) {
                    points.insert(point);
                }
            }
        }

        points
    }
}

fn parse() -> Vec<(Point, Point, i64)> {
    let re = Regex::new(r"Sensor at x=(\d+), y=(\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
        .unwrap();

    let mut points = Vec::new();

    for cap in re.captures_iter(include_str!("input.txt").trim()) {
        let sensor = Point(cap[1].parse().unwrap(), cap[2].parse().unwrap());
        let beacon = Point(cap[3].parse().unwrap(), cap[4].parse().unwrap());
        let distance = sensor.distance(&beacon);
        points.push((sensor, beacon, distance));
    }

    points
}

fn part1(pairs: &[(Point, Point, i64)]) -> i64 {
    let mut beacons = BTreeSet::new();

    let mut range: RangeSet<[std::ops::RangeInclusive<i64>; 1]> = RangeSet::new();
    for pair in pairs {
        let (sensor, beacon, distance) = pair;
        beacons.insert(beacon);

        range.insert_range(sensor.points_in_row_at_distance(P1_Y, *distance));
    }

    range
        .iter()
        .filter(|x| !beacons.contains(&Point(*x, P1_Y)))
        .count() as i64
}

fn part2(pairs: &[(Point, Point, i64)]) -> i64 {
    let mut borders = Vec::new();

    for pair in pairs {
        let sensor = &pair.0;
        let distance = pair.2;

        borders.push(sensor.border(distance + 1));
    }

    let mut points = BTreeSet::new();
    for i in 0..borders.len() {
        for j in i + 1..borders.len() {
            points.extend(borders[i].intersection(&borders[j]));
        }
    }

    points.retain(|p| pairs.iter().all(|pair| pair.0.distance(p) > pair.2) && p.in_square());
    points.into_iter().next().unwrap().tuning_frequency()
}

fn main() {
    println!("The solution to part 1 is {}.", part1(&mut parse()));
    println!("The solution to part 2 is {}.", part2(&mut parse()));
}
