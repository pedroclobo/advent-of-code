#[derive(Copy, Clone, Debug)]
enum Operator {
    Plus,
    Times,
}

#[derive(Debug)]
struct Problem {
    operands: Vec<u64>,
    operator: Operator,
}

impl Problem {
    fn solve(&self) -> u64 {
        match self.operator {
            Operator::Plus => self.operands.iter().sum(),
            Operator::Times => self.operands.iter().product(),
        }
    }
}

fn parse_p1() -> Vec<Problem> {
    let operands = include_str!("input.txt")
        .lines()
        .take_while(|line| !line.starts_with("+") && !line.starts_with("*"))
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let operators = include_str!("input.txt")
        .lines()
        .rev()
        .take(1)
        .map(|line| {
            line.split_whitespace()
                .map(|op| match op {
                    "+" => Operator::Plus,
                    "*" => Operator::Times,
                    _ => panic!("Invalid input"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let n = operands.len();
    let m = operands[0].len();

    let mut problems = Vec::new();

    for j in 0..m {
        let mut ops = Vec::new();
        for i in 0..n {
            ops.push(operands[i][j]);
        }
        problems.push(Problem {
            operands: ops,
            operator: operators[0][j],
        });
    }

    problems
}

fn parse_p2() -> Vec<Problem> {
    let lines: Vec<&str> = include_str!("input.txt").lines().collect();
    let operand_lines: Vec<&str> = lines
        .iter()
        .take_while(|line| !line.starts_with("+") && !line.starts_with("*"))
        .copied()
        .collect();
    let operator_line = lines.last().unwrap();

    // Operators are always left-aligned. Use their position to check for the most significant
    // digit of the operands.
    let operator_positions: Vec<(usize, Operator)> = operator_line
        .chars()
        .enumerate()
        .filter(|(_, ch)| *ch == '+' || *ch == '*')
        .map(|(pos, ch)| {
            let op = match ch {
                '+' => Operator::Plus,
                '*' => Operator::Times,
                _ => panic!("Invalid operator"),
            };
            (pos, op)
        })
        .collect();

    let mut problems = Vec::new();

    // For a given operator, find the span of the numbers above it by finding the start and end
    // positions of numbers that contain `op_pos`.
    for (op_pos, operator) in operator_positions {
        let mut number_spans: Vec<(usize, usize)> = Vec::new();

        for line in &operand_lines {
            let mut in_number = false;
            let mut start = 0;

            for (pos, ch) in line
                .chars()
                .enumerate()
                .chain(std::iter::once((line.len(), ' ')))
            {
                if ch.is_numeric() {
                    if !in_number {
                        (start, in_number) = (pos, true);
                    }
                } else if in_number {
                    let end = pos - 1;
                    if start <= op_pos && op_pos <= end {
                        number_spans.push((start, end));
                        break;
                    }
                    in_number = false;
                }
            }
        }

        // For each character position, read top-to-bottom.
        if let Some(&(_, _)) = number_spans
            .iter()
            .min_by_key(|&&(s, _)| s)
            .and_then(|_| number_spans.iter().max_by_key(|&&(_, e)| e))
        {
            let starts = number_spans.iter().map(|&(s, _)| s).min().unwrap();
            let ends = number_spans.iter().map(|&(_, e)| e).max().unwrap();

            let mut operands = Vec::new();

            // Read right-to-left through the character positions.
            for char_pos in (starts..=ends).rev() {
                let mut num_str = String::new();

                for line in &operand_lines {
                    if let Some(ch) = line.chars().nth(char_pos)
                        && ch.is_numeric()
                    {
                        num_str.push(ch);
                    }
                }

                if !num_str.is_empty() {
                    operands.push(num_str.parse::<u64>().unwrap());
                }
            }

            problems.push(Problem { operands, operator });
        }
    }

    problems
}

fn part1() -> u64 {
    parse_p1().iter().map(|problem| problem.solve()).sum()
}

fn part2() -> u64 {
    parse_p2().iter().map(|problem| problem.solve()).sum()
}

fn main() {
    println!("The solution to part 1 is {}.", part1());
    println!("The solution to part 2 is {}.", part2());
}
