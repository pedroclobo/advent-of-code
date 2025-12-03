fn parse() -> impl Iterator<Item = impl Iterator<Item = u8>> {
    include_str!("input.txt")
        .lines()
        .map(|line| line.as_bytes().iter().map(|num| num - b'0'))
}

fn largest_joltage(bank: &[u8], digits: usize) -> usize {
    let n = bank.len();
    let mut joltage = 0usize;
    let mut i = 0;

    for joltage_size in 0..digits {
        let mut max_digit = 0;
        for j in i..n - digits + joltage_size + 1 {
            if bank[j] > max_digit {
                max_digit = bank[j];
                i = j + 1;
            }
        }
        joltage = joltage * 10 + max_digit as usize;
    }

    joltage
}

fn total_joltage(digits: usize) -> usize {
    parse()
        .map(|bank| largest_joltage(&bank.collect::<Vec<_>>(), digits))
        .sum()
}

fn part1() -> usize {
    total_joltage(2)
}

fn part2() -> usize {
    total_joltage(12)
}

fn main() {
    println!("The solution to part 1 is {}.", part1());
    println!("The solution to part 2 is {}.", part2());
}
