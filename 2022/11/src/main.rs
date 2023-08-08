use num::integer;
use regex::Regex;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    test: Test,
    inspected: u64,
}

impl Monkey {
    fn new(items: VecDeque<u64>, operation: Operation, test: Test) -> Self {
        Self {
            items,
            operation,
            test,
            inspected: 0,
        }
    }
}

#[derive(Debug, Clone)]
enum OperationKind {
    Add,
    Multiply,
}

#[derive(Debug, Clone)]
struct Operation {
    kind: OperationKind,
    arg: Option<u64>,
}

impl Operation {
    fn perform(&self, old: u64) -> u64 {
        match self.arg {
            Some(arg) => match self.kind {
                OperationKind::Add => old + arg,
                OperationKind::Multiply => old * arg,
            },
            None => match self.kind {
                OperationKind::Add => old + old,
                OperationKind::Multiply => old * old,
            },
        }
    }

    fn perform_mod(&self, old: u64, divisor: u64) -> u64 {
        match self.arg {
            Some(arg) => match self.kind {
                OperationKind::Add => (old + arg) % divisor,
                OperationKind::Multiply => (old * arg) % divisor,
            },
            None => match self.kind {
                OperationKind::Add => (old + old) % divisor,
                OperationKind::Multiply => (old % divisor * old % divisor) % divisor,
            },
        }
    }
}

#[derive(Debug, Clone)]
struct Test {
    divisor: u64,
    true_monkey: usize,
    false_monkey: usize,
}

impl Test {
    fn perform(&self, new: u64) -> bool {
        new % self.divisor == 0
    }
}

fn parse() -> Vec<Monkey> {
    let mut monkeys = Vec::new();

    for monkey in include_str!("input.txt").split("\n\n").into_iter() {
        let mut lines = monkey.lines().into_iter();

        // items
        let re = Regex::new(r"\d+").unwrap();
        let starting_items = lines.nth(1).unwrap().trim();
        let starting_items: VecDeque<u64> = re
            .find_iter(starting_items)
            .filter_map(|digits| digits.as_str().parse().ok())
            .collect();

        // operation
        let operation_str = lines.next().unwrap().trim();
        let number = re.find_iter(operation_str).next();
        let operation;
        if let Some(number) = number {
            let number = number.as_str().parse().unwrap();
            operation = Operation {
                kind: match operation_str.contains("+") {
                    true => OperationKind::Add,
                    false => OperationKind::Multiply,
                },
                arg: Some(number),
            };
        } else {
            operation = Operation {
                kind: match operation_str.contains("+") {
                    true => OperationKind::Add,
                    false => OperationKind::Multiply,
                },
                arg: None,
            };
        }

        // test
        let test = lines.next().unwrap().trim();
        let divisor = re
            .find_iter(test)
            .next()
            .unwrap()
            .as_str()
            .parse::<u64>()
            .unwrap();
        let true_condition = lines.next().unwrap().trim();
        let true_monkey = re
            .find_iter(true_condition)
            .next()
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();
        let false_condition = lines.next().unwrap().trim();
        let false_monkey = re
            .find_iter(false_condition)
            .next()
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();

        let test = Test {
            divisor,
            true_monkey,
            false_monkey,
        };

        monkeys.push(Monkey::new(starting_items, operation, test));
    }

    monkeys
}

fn lcm(numbers: &[u64]) -> u64 {
    let mut result = 1;
    for number in numbers {
        result = result * number / integer::gcd(result, *number);
    }
    result
}

fn compute_score(rounds: u32, part1: bool) -> u64 {
    let mut monkeys = parse();
    let divisor = lcm(&monkeys
        .iter()
        .map(|monkey| monkey.test.divisor)
        .collect::<Vec<u64>>());

    for _ in 0..rounds {
        let mut copy = monkeys.clone();
        for i in 0..monkeys.len() {
            let monkey = &monkeys[i];
            for item in &monkeys[i].items {
                let new = match part1 {
                    true => copy[i].operation.perform(*item) / 3,
                    false => copy[i].operation.perform_mod(*item, divisor),
                };
                copy[i].inspected += 1;
                if monkey.test.perform(new) {
                    copy[monkey.test.true_monkey].items.push_back(new);
                } else {
                    copy[monkey.test.false_monkey].items.push_back(new);
                }
            }
            copy[i].items.clear();
            monkeys = copy.clone();
        }
    }

    let mut inspected_items: Vec<u64> = monkeys.iter().map(|monkey| monkey.inspected).collect();
    inspected_items.sort_by(|a, b| b.cmp(a));

    inspected_items[0] * inspected_items[1]
}

fn part1() -> u64 {
    compute_score(20, true)
}

fn part2() -> u64 {
    compute_score(10000, false)
}

fn main() {
    println!("The solution to part 1 is {}.", part1());
    println!("The solution to part 2 is {}.", part2());
}
