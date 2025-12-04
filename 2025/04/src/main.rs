#[derive(Debug)]
enum Position {
    Paper,
    Empty,
}

type Grid = Vec<Vec<Position>>;

fn parse() -> Grid {
    include_str!("input.txt")
        .lines()
        .map(|line| {
            {
                line.chars().map(|char| match char {
                    '@' => Position::Paper,
                    '.' => Position::Empty,
                    _ => panic!("Invalid input"),
                })
            }
            .collect()
        })
        .collect()
}

fn part1(grid: &Grid) -> usize {
    let m = grid.len() as isize;
    let n = grid[0].len() as isize;

    let mut rolls = 0;
    for y in 0isize..m {
        for x in 0isize..n {
            if let Position::Empty = grid[y as usize][x as usize] {
                continue;
            }
            let mut adj = 0;
            for (dx, dy) in [
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ] {
                let nx = x + dx;
                let ny = y + dy;
                if nx < 0 || nx >= n || ny < 0 || ny >= m {
                    continue;
                }
                if let Position::Paper = grid[ny as usize][nx as usize] {
                    adj += 1;
                }
            }
            if adj < 4 {
                rolls += 1;
            }
        }
    }

    rolls
}

fn part2(grid: &mut Grid) -> usize {
    let m = grid.len() as isize;
    let n = grid[0].len() as isize;
    let mut total_rolls = 0;

    loop {
        let mut rolls = 0;
        for y in 0isize..m {
            for x in 0isize..n {
                if let Position::Empty = grid[y as usize][x as usize] {
                    continue;
                }
                let mut adj = 0;
                for (dx, dy) in [
                    (-1, -1),
                    (-1, 0),
                    (-1, 1),
                    (0, -1),
                    (0, 1),
                    (1, -1),
                    (1, 0),
                    (1, 1),
                ] {
                    let nx = x + dx;
                    let ny = y + dy;
                    if nx < 0 || nx >= n || ny < 0 || ny >= m {
                        continue;
                    }
                    if let Position::Paper = grid[ny as usize][nx as usize] {
                        adj += 1;
                    }
                }
                if adj < 4 {
                    grid[y as usize][x as usize] = Position::Empty;
                    rolls += 1;
                }
            }
        }
        total_rolls += rolls;
        if rolls == 0 {
            break;
        }
    }

    total_rolls
}

fn main() {
    let mut grid = parse();
    println!("The solution to part 1 is {}.", part1(&grid));
    println!("The solution to part 2 is {}.", part2(&mut grid));
}
