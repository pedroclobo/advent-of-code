use std::{cell::RefCell, cmp};

#[derive(Debug)]
struct Motion {
    direction: Direction,
    steps: i32,
}

impl Motion {
    fn new(direction: Direction, steps: i32) -> Self {
        Self { direction, steps }
    }
}

#[derive(Debug, PartialEq, Ord, Eq, PartialOrd)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn is_touching(&self, other: &Point) -> bool {
        (self.x == other.x && (self.y - other.y).abs() <= 1)
            || (self.y == other.y && (self.x - other.x).abs() <= 1)
            || (self.x == other.x && self.y == other.y)
            || ((self.x - other.x).abs() == 1 && (self.y - other.y).abs() == 1)
    }

    fn move_closer_to(&mut self, other: &Point) {
        if self.x == other.x {
            match self.y.cmp(&other.y) {
                cmp::Ordering::Less => self.y += 1,
                cmp::Ordering::Greater => self.y -= 1,
                cmp::Ordering::Equal => (),
            }
        } else if self.y == other.y {
            match self.x.cmp(&other.x) {
                cmp::Ordering::Less => self.x += 1,
                cmp::Ordering::Greater => self.x -= 1,
                cmp::Ordering::Equal => (),
            }
        } else {
            match self.x.cmp(&other.x) {
                cmp::Ordering::Less => self.x += 1,
                cmp::Ordering::Greater => self.x -= 1,
                cmp::Ordering::Equal => (),
            }
            match self.y.cmp(&other.y) {
                cmp::Ordering::Less => self.y += 1,
                cmp::Ordering::Greater => self.y -= 1,
                cmp::Ordering::Equal => (),
            }
        }
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Left,
    Down,
}

fn parse() -> Vec<Motion> {
    include_str!("input.txt")
        .lines()
        .into_iter()
        .map(|line| {
            let mut iter = line.split_whitespace();
            let direction = match iter.next().unwrap() {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!("invalid direction"),
            };
            let steps = iter.next().unwrap().parse::<i32>().unwrap();
            Motion::new(direction, steps)
        })
        .collect()
}

fn part1(motions: &Vec<Motion>) -> usize {
    let mut h = Point::new(0, 0);
    let mut t = Point::new(0, 0);

    let mut positions = vec![Point::new(0, 0)];

    for motion in motions {
        match motion.direction {
            Direction::Up => {
                for _ in 0..motion.steps {
                    h.y += 1;
                    if h.is_touching(&t) {
                        continue;
                    } else {
                        t.move_closer_to(&h);
                        positions.push(Point::new(t.x, t.y));
                    }
                }
            }
            Direction::Down => {
                for _ in 0..motion.steps {
                    h.y -= 1;
                    if h.is_touching(&t) {
                        continue;
                    } else {
                        t.move_closer_to(&h);
                        positions.push(Point::new(t.x, t.y));
                    }
                }
            }
            Direction::Right => {
                for _ in 0..motion.steps {
                    h.x += 1;
                    if h.is_touching(&t) {
                        continue;
                    } else {
                        t.move_closer_to(&h);
                        positions.push(Point::new(t.x, t.y));
                    }
                }
            }
            Direction::Left => {
                for _ in 0..motion.steps {
                    h.x -= 1;
                    if h.is_touching(&t) {
                        continue;
                    } else {
                        t.move_closer_to(&h);
                        positions.push(Point::new(t.x, t.y));
                    }
                }
            }
        }
    }

    positions.sort();
    positions.dedup();
    positions.len()
}

fn part2(motions: &Vec<Motion>) -> usize {
    let mut knots = Vec::new();
    for _ in 0..10 {
        knots.push(RefCell::new(Point::new(0, 0)));
    }

    let mut positions = vec![Point::new(0, 0)];

    for motion in motions {
        match motion.direction {
            Direction::Up => {
                for _ in 0..motion.steps {
                    for i in 0..knots.len() - 1 {
                        if i == 0 {
                            knots[i].borrow_mut().y += 1;
                        }

                        if knots[i].borrow().is_touching(&knots[i + 1].borrow()) {
                            continue;
                        } else {
                            knots[i + 1].borrow_mut().move_closer_to(&knots[i].borrow());
                            if i == 8 {
                                positions.push(Point::new(
                                    knots[i + 1].borrow().x,
                                    knots[i + 1].borrow().y,
                                ));
                            }
                        }
                    }
                }
            }

            Direction::Down => {
                for _ in 0..motion.steps {
                    for i in 0..knots.len() - 1 {
                        if i == 0 {
                            knots[i].borrow_mut().y -= 1;
                        }

                        if knots[i].borrow().is_touching(&knots[i + 1].borrow()) {
                            continue;
                        } else {
                            knots[i + 1].borrow_mut().move_closer_to(&knots[i].borrow());
                            if i == 8 {
                                positions.push(Point::new(
                                    knots[i + 1].borrow().x,
                                    knots[i + 1].borrow().y,
                                ));
                            }
                        }
                    }
                }
            }

            Direction::Right => {
                for _ in 0..motion.steps {
                    for i in 0..knots.len() - 1 {
                        if i == 0 {
                            knots[i].borrow_mut().x += 1;
                        }

                        if knots[i].borrow().is_touching(&knots[i + 1].borrow()) {
                            continue;
                        } else {
                            knots[i + 1].borrow_mut().move_closer_to(&knots[i].borrow());
                            if i == 8 {
                                positions.push(Point::new(
                                    knots[i + 1].borrow().x,
                                    knots[i + 1].borrow().y,
                                ));
                            }
                        }
                    }
                }
            }

            Direction::Left => {
                for _ in 0..motion.steps {
                    for i in 0..knots.len() - 1 {
                        if i == 0 {
                            knots[i].borrow_mut().x -= 1;
                        }

                        if knots[i].borrow().is_touching(&knots[i + 1].borrow()) {
                            continue;
                        } else {
                            knots[i + 1].borrow_mut().move_closer_to(&knots[i].borrow());
                            if i == 8 {
                                positions.push(Point::new(
                                    knots[i + 1].borrow().x,
                                    knots[i + 1].borrow().y,
                                ));
                            }
                        }
                    }
                }
            }
        }
    }

    positions.sort();
    positions.dedup();
    positions.len()
}

fn main() {
    let motions = parse();
    println!("The solution to part 1 is {}.", part1(&motions));
    println!("The solution to part 2 is {}.", part2(&motions));
}
