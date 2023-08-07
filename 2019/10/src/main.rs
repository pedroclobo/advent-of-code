use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
enum Content {
    Asteroid,
    Empty,
}

#[derive(Debug)]
struct Place {
    position: Position,
    content: Content,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn step(&self, other: &Position) -> (i32, i32) {
        fn gcd(mut a: i32, mut b: i32) -> i32 {
            while b != 0 {
                let remainder = a % b;
                a = b;
                b = remainder;
            }

            a.abs()
        }

        let (x_start, y_start);
        let (x_end, y_end);

        match self.partial_cmp(&other).unwrap() {
            Ordering::Less => {
                (x_start, y_start) = (self.x, self.y);
                (x_end, y_end) = (other.x, other.y);
            }
            Ordering::Equal => todo!(),
            Ordering::Greater => {
                (x_start, y_start) = (other.x, other.y);
                (x_end, y_end) = (self.x, self.y);
            }
        }

        let (x_diff, y_diff) = (x_end - x_start, y_end - y_start);
        let gcd = gcd(x_diff, y_diff);

        (x_diff / gcd, y_diff / gcd)
    }

    /// angle between y-axis and vector from station's position to position
    fn angle(&self, station: &Position) -> f64 {
        let (x1, y1) = (0, 1);
        let (x2, y2) = (station.x - self.x, station.y - self.y);

        let angle = ((x1 * y2 - y1 * x2) as f64).atan2((x1 * x2 + y1 * y2) as f64);
        if angle < 0.0 {
            angle + 2.0 * std::f64::consts::PI
        } else {
            angle
        }
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self.y.cmp(&other.y), self.x.cmp(&other.x)) {
            (Ordering::Less, _) => Some(Ordering::Less),
            (Ordering::Equal, Ordering::Less) => Some(Ordering::Less),
            (Ordering::Equal, Ordering::Equal) => Some(Ordering::Equal),
            (Ordering::Equal, Ordering::Greater) => Some(Ordering::Greater),
            (Ordering::Greater, _) => Some(Ordering::Greater),
        }
    }
}

#[derive(Debug)]
struct Map {
    positions: Vec<Vec<Place>>,
}

impl Map {
    fn asteroid_positions(&self) -> Vec<Position> {
        self.positions
            .iter()
            .flatten()
            .filter(|p| p.content == Content::Asteroid)
            .map(|p| p.position.clone())
            .collect()
    }

    fn intermediate_places(&self, a: &Position, b: &Position) -> Vec<&Place> {
        let mut places = vec![];

        let (x_start, y_start);
        let (x_end, y_end);
        if a > b {
            (x_start, y_start) = (b.x, b.y);
            (x_end, y_end) = (a.x, a.y);
        } else {
            (x_start, y_start) = (a.x, a.y);
            (x_end, y_end) = (b.x, b.y);
        }

        let (x_step, y_step) = a.step(&b);
        let (mut x, mut y) = (x_start + x_step, y_start + y_step);

        while (x, y) != (x_end, y_end) {
            places.push(&self.positions[y as usize][x as usize]);

            x += x_step;
            y += y_step;
        }

        places
    }

    fn view_depth(&self, a: &Position, b: &Position) -> usize {
        self.intermediate_places(a, b)
            .into_iter()
            .filter(|p| p.content == Content::Asteroid)
            .count()
    }

    fn can_detect_each_other(&self, a: &Position, b: &Position) -> bool {
        self.view_depth(a, b) == 0
    }

    fn num_detections(&self, position: &Position) -> u32 {
        self.asteroid_positions()
            .into_iter()
            .filter(|p| {
                if p == position {
                    return false;
                }
                self.can_detect_each_other(&position, &p)
            })
            .count() as u32
    }

    fn station_detections(&self) -> (Position, u32) {
        self.asteroid_positions()
            .into_iter()
            .map(|p| (p.clone(), self.num_detections(&p)))
            .max_by(|(_, a), (_, b)| a.cmp(b))
            .unwrap()
    }
}

fn parse() -> Map {
    Map {
        positions: include_str!("input.txt")
            .lines()
            .enumerate()
            .map(|(i, line)| {
                line.chars()
                    .enumerate()
                    .map(|(j, c)| Place {
                        content: match c {
                            '#' => Content::Asteroid,
                            '.' => Content::Empty,
                            _ => panic!("invalid input"),
                        },
                        position: Position {
                            x: j as i32,
                            y: i as i32,
                        },
                    })
                    .collect()
            })
            .collect(),
    }
}

fn part1(map: &Map) -> (Position, u32) {
    map.station_detections()
}

fn part2(map: &mut Map, station: &Position) -> u32 {
    let mut asteroids = map
        .asteroid_positions()
        .into_iter()
        .filter(|p| p != station)
        .collect::<Vec<_>>();

    asteroids.sort_by(|p, q| {
        match map
            .view_depth(&station, &p)
            .partial_cmp(&map.view_depth(&station, &q))
            .unwrap()
        {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => p.angle(&station).partial_cmp(&q.angle(&station)).unwrap(),
            Ordering::Greater => Ordering::Greater,
        }
    });

    let asteroid = &asteroids[199];
    (asteroid.x * 100 + asteroid.y) as u32
}

fn main() {
    let mut map = parse();
    let (station, part1) = part1(&map);

    println!("The solution to part 1 is {}.", part1);
    println!("The solution to part 2 is {}.", part2(&mut map, &station));
}
