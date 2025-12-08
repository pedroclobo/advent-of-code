use std::collections::HashMap;

fn parse() -> HashMap<usize, (i64, i64, i64)> {
    include_str!("input.txt")
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let mut splits = line.split(",");
            let coords = (
                splits.next().unwrap().parse().unwrap(),
                splits.next().unwrap().parse().unwrap(),
                splits.next().unwrap().parse().unwrap(),
            );
            (i, coords)
        })
        .collect()
}

fn calculate_distances(boxes: &HashMap<usize, (i64, i64, i64)>) -> Vec<(usize, usize, i64)> {
    fn distance(p1: (i64, i64, i64), p2: (i64, i64, i64)) -> i64 {
        let (x1, y1, z1) = p1;
        let (x2, y2, z2) = p2;
        (x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1) + (z2 - z1) * (z2 - z1)
    }

    let n = boxes.len();
    let mut distances = Vec::new();

    for i in 0..n {
        for j in i + 1..n {
            if i == j {
                continue;
            }
            distances.push((i, j, distance(boxes[&i], boxes[&j])));
        }
    }

    distances.sort_by(|a, b| b.2.cmp(&a.2));
    distances
}

struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
        }
    }

    fn find(&mut self, v: usize) -> usize {
        if v == self.parent[v] {
            v
        } else {
            let parent = self.find(self.parent[v]);
            self.parent[v] = parent;
            parent
        }
    }

    fn merge(&mut self, a: usize, b: usize) {
        let a = self.find(a);
        let b = self.find(b);
        self.parent[b] = a;
    }

    fn components(&mut self) -> Vec<i64> {
        let mut counts = HashMap::new();

        for v in 0..self.parent.len() {
            let parent = self.find(v);
            *counts.entry(parent).or_insert(0) += 1;
        }

        let mut components = counts.into_values().collect::<Vec<_>>();
        components.sort_by(|a, b| b.cmp(a));
        components
    }
}

fn part1(distances: &mut Vec<(usize, usize, i64)>, uf: &mut UnionFind) -> i64 {
    for _ in 0..1000 {
        let (a, b, _) = distances.pop().unwrap();
        uf.merge(a, b);
    }

    uf.components().iter().take(3).product()
}

fn part2(
    boxes: &HashMap<usize, (i64, i64, i64)>,
    distances: &mut Vec<(usize, usize, i64)>,
    uf: &mut UnionFind,
) -> i64 {
    loop {
        let (a, b, _) = distances.pop().unwrap();
        uf.merge(a, b);

        if uf.components().len() == 1 {
            return boxes[&a].0 * boxes[&b].0;
        }
    }
}

fn main() {
    let boxes = parse();
    let mut distances = calculate_distances(&boxes);
    let mut uf = UnionFind::new(boxes.len());

    println!(
        "The solution to part 1 is {}.",
        part1(&mut distances, &mut uf)
    );
    println!(
        "The solution to part 2 is {}.",
        part2(&boxes, &mut distances, &mut uf)
    );
}
