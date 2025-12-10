use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum State {
    On,
    Off,
}

impl State {
    fn toggle(self) -> Self {
        match self {
            State::On => State::Off,
            State::Off => State::On,
        }
    }
}

#[derive(Debug)]
struct Machine {
    diagram: Vec<State>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<u64>,
}

impl Machine {
    fn configure_lights(&self) -> u64 {
        fn aux(
            target: &[State],
            buttons: &Vec<Vec<usize>>,
            not_pressed: u16, // pressing the same button twice is the same as not pressing it
            current: Vec<State>,
            memo: &mut HashMap<(Vec<State>, u16), Option<u64>>,
        ) -> Option<u64> {
            if not_pressed == 0 {
                return None;
            }
            if current == *target {
                return Some(0);
            }
            if let Some(cached) = memo.get(&(current.clone(), not_pressed)) {
                return *cached;
            }

            let min_presses = buttons
                .iter()
                .enumerate()
                .filter(|(i, _)| (not_pressed >> i) & 1 != 0)
                .map(|(i, button)| {
                    let mut new_diagram = current.clone();
                    for i in button {
                        new_diagram[*i] = new_diagram[*i].toggle();
                    }
                    aux(target, buttons, not_pressed & !(1 << i), new_diagram, memo)
                        .map(|res| res + 1)
                });

            let res = min_presses.into_iter().flatten().min();
            memo.insert((current, not_pressed), res);
            res
        }

        aux(
            &self.diagram,
            &self.buttons,
            (2_usize.pow(self.buttons.len() as u32) - 1) as u16,
            vec![State::Off; self.diagram.len()],
            &mut HashMap::new(),
        )
        .unwrap()
    }

    fn configure_joltage(&self) -> u64 {
        let optimizer = z3::Optimize::new();

        let n = self.joltage.len();
        let m = self.buttons.len();

        let presses: Vec<_> = (0..m)
            .map(|i| z3::ast::Int::new_const(format!("x{}", i)))
            .collect();
        for x in &presses {
            optimizer.assert(&x.ge(z3::ast::Int::from_i64(0)));
        }

        for coord in 0..n {
            // For `coord` = 3, if `self.buttons` is [(1, 3), (3, )], `sum` is `2 * x3`.
            let mut sum = z3::ast::Int::from_u64(0);
            for (i, button) in self.buttons.iter().enumerate() {
                if button.contains(&coord) {
                    sum += presses[i].clone();
                }
            }
            // Assert that `a * x_n == self.joltage[n]`
            optimizer.assert(&sum.eq(z3::ast::Int::from_u64(self.joltage[coord])));
        }

        let total_presses = presses
            .iter()
            .fold(z3::ast::Int::from_i64(0), |acc, x| acc + x);
        optimizer.minimize(&total_presses);

        assert!(matches!(optimizer.check(&[]), z3::SatResult::Sat));
        let model = optimizer.get_model().unwrap();
        presses
            .iter()
            .map(|press| model.eval(press, true).unwrap().as_u64().unwrap())
            .sum()
    }
}

fn parse() -> Vec<Machine> {
    include_str!("input.txt")
        .lines()
        .map(|line| {
            let (start, end) = (line.find('[').unwrap(), line.find(']').unwrap());
            let diagram = line[start + 1..end]
                .chars()
                .filter_map(|c| match c {
                    '.' => Some(State::Off),
                    '#' => Some(State::On),
                    _ => None,
                })
                .collect::<Vec<_>>();

            let mut rest = &line[end + 1..];
            let mut buttons = Vec::new();
            while let Some(open) = rest.find('(') {
                let close = rest[open..].find(')').unwrap() + open;
                let bs = &rest[open + 1..close];

                let indices = if bs.trim().is_empty() {
                    Vec::new()
                } else {
                    bs.split(',')
                        .map(|s| s.trim().parse::<usize>().unwrap())
                        .collect()
                };

                buttons.push(indices);
                rest = &rest[close + 1..];
            }

            let (start, end) = (rest.find('{').unwrap(), rest.find('}').unwrap());
            let joltage = rest[start + 1..end]
                .split(',')
                .map(|s| s.trim().parse::<u64>().unwrap())
                .collect();

            Machine {
                diagram,
                buttons,
                joltage,
            }
        })
        .collect()
}

fn part1(machines: &[Machine]) -> u64 {
    machines
        .iter()
        .map(|machine| machine.configure_lights())
        .sum()
}

fn part2(machines: &[Machine]) -> u64 {
    machines
        .iter()
        .map(|machine| machine.configure_joltage())
        .sum()
}

fn main() {
    let machines = parse();
    println!("The solution to part 1 is {}.", part1(&machines));
    println!("The solution to part 2 is {}.", part2(&machines));
}
