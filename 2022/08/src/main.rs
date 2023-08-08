use itertools;
use std::cmp;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn parse() -> Vec<Vec<u32>> {
    let mut grid = Vec::new();

    for line in include_str!("input.txt").lines() {
        grid.push(line.chars().map(|c| c.to_digit(10).unwrap()).collect());
    }

    grid
}

fn is_visible(grid: &Vec<Vec<u32>>, x: usize, y: usize) -> bool {
    if x == 0 || x == grid.len() - 1 || y == 0 || y == grid.len() - 1 {
        return true;
    }

    let mut column = Vec::new();
    for y in 0..grid.len() {
        column.push(grid[y].iter().nth(x).unwrap())
    }

    return itertools::all(&grid[y][0..x], |tree| tree < &grid[y][x])
        || itertools::all(&grid[y][x + 1..grid.len()], |tree| tree < &grid[y][x])
        || itertools::all(&column[0..y], |tree| tree < &&grid[y][x])
        || itertools::all(&column[y + 1..grid.len()], |tree| tree < &&grid[y][x]);
}

fn part1(grid: &Vec<Vec<u32>>) -> u32 {
    let mut count = 0;
    for y in 0..grid.len() {
        for x in 0..grid.len() {
            if is_visible(grid, x, y) {
                count += 1;
            }
        }
    }

    count
}

fn get_number_visible_trees(grid: &Vec<Vec<u32>>, x: usize, y: usize, direction: Direction) -> u32 {
    match direction {
        Direction::Up => {
            if y == 0 {
                return 0;
            }

            let mut column = Vec::new();
            for y in 0..grid.len() {
                column.push(grid[y].iter().nth(x).unwrap())
            }

            let mut count = 0;
            for j in (0..y).rev() {
                if column[j] < &grid[y][x] {
                    count += 1;
                } else {
                    count += 1;
                    break;
                }
            }

            return count;
        }
        Direction::Down => {
            if y == grid.len() - 1 {
                return 0;
            }

            let mut column = Vec::new();
            for y in 0..grid.len() {
                column.push(grid[y].iter().nth(x).unwrap())
            }

            let mut count = 0;
            for j in y + 1..grid.len() {
                if column[j] < &grid[y][x] {
                    count += 1;
                } else {
                    count += 1;
                    break;
                }
            }

            return count;
        }
        Direction::Left => {
            if x == 0 {
                return 0;
            }

            let mut count = 0;
            for i in (0..x).rev() {
                if grid[y][i] < grid[y][x] {
                    count += 1;
                } else {
                    count += 1;
                    break;
                }
            }

            return count;
        }
        Direction::Right => {
            if x == grid.len() - 1 {
                return 0;
            }

            let mut count = 0;
            for i in x + 1..grid.len() {
                if grid[y][i] < grid[y][x] {
                    count += 1;
                } else {
                    count += 1;
                    break;
                }
            }

            return count;
        }
    }
}

fn part2(grid: &Vec<Vec<u32>>) -> u32 {
    let mut highest_score = 0;
    for y in 0..grid.len() {
        for x in 0..grid.len() {
            highest_score = cmp::max(
                highest_score,
                get_number_visible_trees(&grid, x, y, Direction::Up)
                    * get_number_visible_trees(&grid, x, y, Direction::Down)
                    * get_number_visible_trees(&grid, x, y, Direction::Left)
                    * get_number_visible_trees(&grid, x, y, Direction::Right),
            );
        }
    }

    highest_score
}

fn main() {
    let grid = parse();
    println!("The solution to part 1 is {}.", part1(&grid));
    println!("The solution to part 2 is {}.", part2(&grid));
}
