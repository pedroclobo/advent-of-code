fn fuel(mass: i32) -> i32 {
    if mass / 3 - 2 < 0 {
        0
    } else {
        mass / 3 - 2
    }
}

fn part1(masses: &Vec<i32>) -> i32 {
    let mut total_fuel = 0;

    for &mass in masses {
        total_fuel += fuel(mass);
    }

    total_fuel
}

fn part2(masses: &Vec<i32>) -> i32 {
    let mut total_fuel = 0;

    for &mass in masses {
        let mut current_fuel = fuel(mass);
        while current_fuel != 0 {
            total_fuel += current_fuel;
            current_fuel = fuel(current_fuel);
        }
    }

    total_fuel
}

fn main() {
    let masses: Vec<i32> = include_str!("../input.txt")
        .lines()
        .map(|num| num.parse().unwrap())
        .collect();

    println!("The solution to part 1 is {}.", part1(&masses));
    println!("The solution to part 2 is {}.", part2(&masses));
}
