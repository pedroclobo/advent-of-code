use std::collections::HashMap;
use std::collections::VecDeque;

enum SquareKind {
    Start,
    End,
    Middle,
}

struct Square {
    id: usize,
    kind: SquareKind,
    elevation: u32,
}

impl Square {
    fn new(id: usize, kind: SquareKind, elevation: u32) -> Self {
        Square {
            id,
            kind,
            elevation,
        }
    }
}

struct Graph {
    start_id: usize,
    end_id: usize,
    vertices: Vec<Square>,
    edges: HashMap<usize, Vec<usize>>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            vertices: Vec::new(),
            edges: HashMap::new(),
            start_id: 0,
            end_id: 0,
        }
    }

    fn add_vertex(&mut self, square: Square) -> usize {
        let id = square.id;
        match square.kind {
            SquareKind::Start => self.start_id = id,
            SquareKind::End => self.end_id = id,
            _ => {}
        }
        self.vertices.push(square);
        id
    }

    fn add_edge(&mut self, from: usize, to: usize) {
        self.edges.entry(from).or_insert(Vec::new()).push(to);
    }

    fn get_edges(&self, id: usize) -> Option<&Vec<usize>> {
        self.edges.get(&id)
    }

    fn get_elevation(&self, id: usize) -> Option<u32> {
        if id >= self.vertices.len() {
            return None;
        }

        Some(self.vertices[id].elevation)
    }

    fn contains(&self, id: usize) -> bool {
        if id >= self.vertices.len() {
            return false;
        }

        true
    }

    fn traverse(&self, start: usize) -> Option<u32> {
        let mut visited = vec![false; self.vertices.len()];
        let mut queue = VecDeque::new();

        queue.push_back((start, 0));

        while let Some((current_id, steps)) = queue.pop_front() {
            if current_id == self.end_id {
                return Some(steps);
            }

            if visited[current_id] {
                continue;
            }

            visited[current_id] = true;

            self.get_edges(current_id)
                .unwrap_or(&vec![])
                .iter()
                .for_each(|id| {
                    queue.push_back((*id, steps + 1));
                });
        }

        None
    }
}

fn parse() -> Graph {
    let mut graph = Graph::new();

    let input = include_str!("input.txt");
    let y = input.lines().next().unwrap().chars().count();

    let mut id = 0;
    for line in input.lines() {
        for c in line.chars() {
            graph.add_vertex(match c {
                'S' => Square::new(id, SquareKind::Start, 0),
                'E' => Square::new(id, SquareKind::End, 'z' as u32 - 'a' as u32),
                _ => Square::new(id, SquareKind::Middle, c as u32 - 'a' as u32),
            });
            id += 1;
        }
    }

    let get_adjacents = |id: usize| {
        if id % y == 0 {
            vec![id.checked_add(1), id.checked_sub(y), id.checked_add(y)]
        } else if id % y == y - 1 {
            vec![id.checked_sub(1), id.checked_sub(y), id.checked_add(y)]
        } else {
            vec![
                id.checked_sub(1),
                id.checked_add(1),
                id.checked_sub(y),
                id.checked_add(y),
            ]
        }
    };

    for id in 0..graph.vertices.len() {
        let adjacent_ids: Vec<usize> = get_adjacents(id)
            .into_iter()
            .flatten()
            .filter(|id| graph.contains(*id))
            .collect();

        for adjacent_id in adjacent_ids {
            let elevation = graph.vertices[id].elevation;
            let adjacent_elevation = graph.get_elevation(adjacent_id).unwrap();
            match graph.vertices[id].kind {
                SquareKind::Start => {
                    if adjacent_elevation <= 1 {
                        graph.add_edge(id, adjacent_id);
                    }
                }
                SquareKind::End => {}
                SquareKind::Middle => {
                    if adjacent_elevation <= elevation + 1 {
                        graph.add_edge(id, adjacent_id);
                    }
                }
            }
        }
    }

    graph
}

fn part1(graph: &Graph) -> u32 {
    let start_id = graph.start_id;
    graph.traverse(start_id).expect("no path found")
}

fn part2(graph: &Graph) -> u32 {
    graph
        .vertices
        .iter()
        .filter(|square| square.elevation == 0)
        .map(|square| graph.traverse(square.id).unwrap_or(u32::MAX))
        .min()
        .unwrap()
}

fn main() {
    let graph = parse();
    println!("The solution to part 1 is {}.", part1(&graph));
    println!("The solution to part 2 is {}.", part2(&graph));
}
