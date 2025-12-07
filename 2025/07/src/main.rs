use std::collections::{HashMap, HashSet};

#[derive(Debug)]
enum Position {
    Empty,
    Splitter,
    Beam,
}

#[derive(Debug)]
struct Manifold {
    start: (usize, usize),
    grid: Vec<Vec<Position>>,
}

fn parse() -> Manifold {
    let (mut sy, mut sx) = (0, 0);
    Manifold {
        grid: include_str!("input.txt")
            .lines()
            .enumerate()
            .map(|(i, line)| {
                line.chars()
                    .enumerate()
                    .map(|(j, c)| match c {
                        '.' => Position::Empty,
                        'S' => {
                            (sy, sx) = (i, j);
                            Position::Beam
                        }
                        '^' => Position::Splitter,
                        _ => panic!("Invalid input"),
                    })
                    .collect()
            })
            .collect(),
        start: (sy, sx),
    }
}

fn part1(manifold: &Manifold) -> u64 {
    let n = manifold.grid.len();
    let m = manifold.grid[0].len();

    let mut splits = 0;

    let mut queue = vec![manifold.start];
    let mut visited = HashSet::new();

    while let Some((y, x)) = queue.pop() {
        if visited.contains(&(y, x)) {
            continue;
        }
        visited.insert((y, x));

        let (ny, nx) = (y + 1, x);
        if ny >= n || nx >= m {
            continue;
        }

        if let Position::Empty = manifold.grid[ny][nx] {
            queue.push((ny, nx));
        } else if let Position::Splitter = manifold.grid[ny][nx] {
            splits += 1;
            if nx > 0 && matches!(manifold.grid[ny][nx - 1], Position::Empty) {
                queue.push((ny, nx - 1));
            }
            if nx < m - 1 && matches!(manifold.grid[ny][nx + 1], Position::Empty) {
                queue.push((ny, nx + 1));
            }
        }
    }

    splits
}

fn paths(manifold: &Manifold, start: (usize, usize), dp: &mut HashMap<(usize, usize), u64>) -> u64 {
    if let Some(res) = dp.get(&start) {
        return *res;
    }

    let (y, x) = start;
    if y == manifold.grid.len() {
        return 1;
    }

    let res = if let Position::Splitter = manifold.grid[y][x] {
        paths(manifold, (y, x - 1), dp) + paths(manifold, (y, x + 1), dp)
    } else {
        paths(manifold, (y + 1, x), dp)
    };

    dp.insert((y, x), res);
    res
}

fn part2(manifold: &Manifold) -> u64 {
    paths(manifold, manifold.start, &mut HashMap::new())
}

fn main() {
    let manifold = parse();
    println!("The solution to part 1 is {}.", part1(&manifold));
    println!("The solution to part 2 is {}.", part2(&manifold));
}
