use std::ops::RangeInclusive;

fn parse() -> impl Iterator<Item = RangeInclusive<usize>> {
    include_str!("input.txt").split(',').map(|range| {
        let mut nums = range.split('-');
        let start = nums.next().unwrap().parse::<usize>().unwrap();
        let end = nums.next().unwrap().trim_end().parse::<usize>().unwrap();
        start..=end
    })
}

fn is_invalid(num: usize, partitions: Option<usize>) -> bool {
    let num_digits = |mut num| -> usize {
        let mut digits = 0;
        while num > 0 {
            digits += 1;
            num /= 10;
        }
        digits
    };
    let n_digits = num_digits(num);

    let nth_digit = |num, n| -> usize { (num / 10_usize.pow(n as u32)) % 10 };
    let partitions = partitions.unwrap_or(n_digits);

    for partition_size in 2..=partitions {
        // Can't divide into equal partitions
        if n_digits % partition_size != 0 {
            continue;
        }

        let mut invalid = true;
        for i in 0..n_digits / partition_size {
            // i'th digits for a given partition size
            let ith_digits = (0..partition_size)
                .map(|offset| nth_digit(num, i + (n_digits / partition_size) * offset))
                .collect::<Vec<_>>();
            let first = ith_digits.first().unwrap();
            invalid &= ith_digits.iter().all(|e| e == first);
        }
        // A number is invalid if all the i'th digits for i=0..=n_digits/partition_size in a
        // given partition are equal
        if invalid {
            return true;
        }
    }

    false
}

fn part1() -> usize {
    parse()
        .flatten()
        .filter(|num| is_invalid(*num, Some(2)))
        .sum()
}

fn part2() -> usize {
    parse().flatten().filter(|num| is_invalid(*num, None)).sum()
}

fn main() {
    println!("The solution to part 1 is {}.", part1());
    println!("The solution to part 2 is {}.", part2());
}
