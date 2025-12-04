enum Part {
    Part1,
    Part2,
}

struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn race(&self, boat: Boat, distance: u64, remaining: u64) -> u64 {
        if remaining == 0 {
            (distance > self.distance) as u64
        } else if boat.moved {
            (distance + remaining * boat.speed > self.distance) as u64
        } else {
            let mut b1 = boat.clone();
            b1.speed += 1;
            self.race(b1, distance, remaining - 1)
                + (distance + remaining * boat.speed > self.distance) as u64
        }
    }
}

#[derive(Debug, Clone, Default)]
struct Boat {
    moved: bool,
    speed: u64,
}

fn parse(p: Part) -> impl Iterator<Item = Race> {
    let mut time = Vec::new();
    let mut distance = Vec::new();

    for line in include_str!("input.txt").lines() {
        let (label, values) = line.split_once(':').unwrap();
        let nums = if let Part::Part1 = p {
            values
                .split_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        } else {
            vec![values.replace(" ", "").parse::<u64>().unwrap()]
        };
        match label {
            "Time" => time = nums,
            "Distance" => distance = nums,
            _ => panic!("Invalid input"),
        }
    }

    time.into_iter()
        .zip(distance)
        .map(|(time, distance)| Race { time, distance })
}

fn part1() -> u64 {
    parse(Part::Part1)
        .map(|race| {
            let boat = Boat::default();
            race.race(boat, 0, race.time)
        })
        .product()
}

fn part2() -> u64 {
    parse(Part::Part2)
        .map(|race| {
            let boat = Boat::default();
            race.race(boat, 0, race.time)
        })
        .product()
}

fn main() {
    println!("The solution to part 1 is {}.", part1());
    println!("The solution to part 1 is {}.", part2());
}
