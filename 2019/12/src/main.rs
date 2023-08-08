use regex::Regex;
use std::cmp::Ordering;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
struct Position {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug, Clone, PartialEq)]
struct Velocity {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug, Clone, PartialEq)]
struct Moon {
    position: Position,
    velocity: Velocity,
}

#[derive(Debug, Clone, PartialEq)]
struct Moons {
    moons: Vec<Moon>,
}

impl Moons {
    fn apply_gravity(&mut self) {
        for i in 0..self.moons.len() {
            for j in i..self.moons.len() {
                if i != j {
                    let mut first = self.moons[i].clone();
                    let second = &mut self.moons[j];
                    first.apply_gravity(second);
                    self.moons[i] = first;
                }
            }
        }
    }

    fn apply_velocity(&mut self) {
        for moon in self.moons.iter_mut() {
            moon.apply_velocity();
        }
    }

    fn step(&mut self) {
        self.apply_gravity();
        self.apply_velocity();
    }

    fn total_energy(&self) -> i32 {
        self.moons.iter().map(|m| m.total_energy()).sum()
    }
}

impl Moon {
    fn apply_gravity(&mut self, other: &mut Moon) {
        match self.position.x.cmp(&other.position.x) {
            Ordering::Less => {
                self.velocity.x += 1;
                other.velocity.x -= 1;
            }
            Ordering::Equal => {}
            Ordering::Greater => {
                self.velocity.x -= 1;
                other.velocity.x += 1;
            }
        }
        match self.position.y.cmp(&other.position.y) {
            Ordering::Less => {
                self.velocity.y += 1;
                other.velocity.y -= 1;
            }
            Ordering::Equal => {}
            Ordering::Greater => {
                self.velocity.y -= 1;
                other.velocity.y += 1;
            }
        }
        match self.position.z.cmp(&other.position.z) {
            Ordering::Less => {
                self.velocity.z += 1;
                other.velocity.z -= 1;
            }
            Ordering::Equal => {}
            Ordering::Greater => {
                self.velocity.z -= 1;
                other.velocity.z += 1;
            }
        }
    }

    fn apply_velocity(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
        self.position.z += self.velocity.z;
    }

    fn total_energy(&self) -> i32 {
        (self.position.x.abs() + self.position.y.abs() + self.position.z.abs())
            * (self.velocity.x.abs() + self.velocity.y.abs() + self.velocity.z.abs())
    }
}

#[derive(Debug)]
struct PositionParseError;
impl FromStr for Position {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^<x=(-?[0-9]+), y=(-?[0-9]+), z=(-?[0-9]+)>$").unwrap();

        let captures = re.captures(s).unwrap();
        let x = captures[1].parse()?;
        let y = captures[2].parse()?;
        let z = captures[3].parse()?;

        Ok(Position { x, y, z })
    }
}

fn parse() -> Moons {
    Moons {
        moons: include_str!("input.txt")
            .lines()
            .map(|l| Moon {
                position: Position::from_str(l).unwrap(),
                velocity: Velocity { x: 0, y: 0, z: 0 },
            })
            .collect(),
    }
}

fn part1(moons: &mut Moons) -> i32 {
    for _ in 1..=1000 {
        moons.step();
    }

    moons.total_energy()
}

fn part2(moons: &mut Moons) -> i64 {
    fn gcd(a: i64, b: i64) -> i64 {
        if b == 0 {
            a
        } else {
            gcd(b, a % b)
        }
    }

    fn lcm(a: i64, b: i64) -> i64 {
        (a * b) / gcd(a, b)
    }

    let initial_state = moons.clone();

    let state_x = |state: &Moons| {
        let mut res = Vec::new();
        for moon in &state.moons {
            res.push((moon.position.x, moon.velocity.x));
        }
        res
    };
    let state_y = |state: &Moons| {
        let mut res = Vec::new();
        for moon in &state.moons {
            res.push((moon.position.y, moon.velocity.y));
        }
        res
    };
    let state_z = |state: &Moons| {
        let mut res = Vec::new();
        for moon in &state.moons {
            res.push((moon.position.z, moon.velocity.z));
        }
        res
    };

    let (mut px, mut py, mut pz) = (None, None, None);
    let mut i = 0;
    loop {
        moons.step();
        i += 1;

        if state_x(moons) == state_x(&initial_state) {
            px = Some(i);
        }
        if state_y(moons) == state_y(&initial_state) {
            py = Some(i);
        }
        if state_z(moons) == state_z(&initial_state) {
            pz = Some(i);
        }
        if !px.is_none() && !py.is_none() && !pz.is_none() {
            break;
        }
    }

    lcm(px.unwrap(), lcm(py.unwrap(), pz.unwrap()))
}

fn main() {
    println!("The solution to part 1 is {}.", part1(&mut parse()));
    println!("The solution to part 2 is {}.", part2(&mut parse()));
}
