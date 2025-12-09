fn parse() -> Vec<(i64, i64)> {
    include_str!("input.txt")
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(",").unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect()
}

fn part1(vertices: &[(i64, i64)]) -> i64 {
    let n = vertices.len();
    (0..n)
        .flat_map(|i| {
            (i + 1..n).map(move |j| {
                let (x1, y1) = vertices[i];
                let (x2, y2) = vertices[j];
                Region::new(&[(x1, y1), (x2, y1), (x1, y2), (x2, y2)])
            })
        })
        .map(|square| square.area())
        .max()
        .unwrap()
}

#[derive(Debug)]
struct Region {
    vertices: Vec<(i64, i64)>,
}

impl Region {
    fn new(vertices: &[(i64, i64)]) -> Self {
        Self {
            vertices: vertices.to_vec(),
        }
    }

    fn area(&self) -> i64 {
        assert_eq!(self.vertices.len(), 4);
        ((self.vertices[0].0 - self.vertices[3].0).abs() + 1)
            * ((self.vertices[0].1 - self.vertices[3].1).abs() + 1)
    }

    fn contains_point(&self, p: (i64, i64)) -> bool {
        let n = self.vertices.len();
        let mut count = 0;

        for i in 0..n {
            let a = self.vertices[i];
            let b = self.vertices[(i + 1) % n];

            // Check if point is on the edge.
            if (p.1 - a.1) * (b.0 - a.0) == (p.0 - a.0) * (b.1 - a.1)
                && a.0.min(b.0) <= p.0
                && p.0 <= a.0.max(b.0)
                && a.1.min(b.1) <= p.1
                && p.1 <= a.1.max(b.1)
            {
                return true;
            }

            // Count intersections
            if (a.1 > p.1) != (b.1 > p.1) {
                let x_intersect = a.0 + (p.1 - a.1) * (b.0 - a.0) / (b.1 - a.1);
                if x_intersect > p.0 {
                    count += 1;
                }
            }
        }
        // If the rays traced by the point cross `self`'s edges an odd number of times, the point
        // must be inside `self`.
        count % 2 == 1
    }

    fn contains(&self, other: &Region) -> bool {
        // Check if the edges `p1 - p2` and `p3 - p4` cross each other.
        fn intersect(e1: ((i64, i64), (i64, i64)), e2: ((i64, i64), (i64, i64))) -> bool {
            // =  1: `c` is to the left  of line `a - b` (counterclockwise turn)
            // = -1: `c` is to the right of line `a - b` (clockwise turn)
            // =  0: `c` is              on line `a - b` (collinear)
            fn cross_product_sign(a: (i64, i64), b: (i64, i64), c: (i64, i64)) -> i64 {
                ((b.0 - a.0) * (c.1 - a.1) - (b.1 - a.1) * (c.0 - a.0)).signum()
            }

            let (p1, p2) = e1;
            let (p3, p4) = e2;

            // Segments intersect in interior if signs of cross products are different on each side
            cross_product_sign(p3, p4, p1) *   // Is `p1` left or right of `p3 - p4`?
            cross_product_sign(p3, p4, p2) < 0 // Is `p2` left or right of `p3 - p4`?
            &&
            cross_product_sign(p1, p2, p3) *   // Is `p3` left or right of `p1 - p2`?
            cross_product_sign(p1, p2, p4) < 0 // Is `p4` left or right of `p1 - p2`?
        }

        // All of `other`'s vertices must be inside `self`.
        // This is not sufficient as the `other` could be rotated, all of its vertices contained
        // inside `self`, and an edge between two vertices could cross outside `self`.
        if !other.vertices.iter().all(|p| self.contains_point(*p)) {
            return false;
        }

        // No edges of `other` can intersect edges of `self` (except at endpoints).
        // If they intersect in the middle, `other` is not fully contained in `self`.
        for i in 0..other.vertices.len() {
            let e1 = (
                other.vertices[i],
                other.vertices[(i + 1) % other.vertices.len()],
            );

            for j in 0..self.vertices.len() {
                let e2 = (
                    self.vertices[j],
                    self.vertices[(j + 1) % self.vertices.len()],
                );

                if intersect(e1, e2) {
                    return false;
                }
            }
        }

        true
    }
}

fn part2(vertices: &[(i64, i64)]) -> i64 {
    let region = Region::new(vertices);
    let n = vertices.len();

    (0..n)
        .flat_map(|i| {
            (i + 1..n).map(move |j| {
                let (x1, y1) = vertices[i];
                let (x2, y2) = vertices[j];
                Region::new(&[(x1, y1), (x2, y1), (x1, y2), (x2, y2)])
            })
        })
        .filter(|square| region.contains(square))
        .map(|square| square.area())
        .max()
        .unwrap()
}

fn main() {
    let vertices = parse();
    println!("The solution to part 1 is {}.", part1(&vertices));
    println!("The solution to part 2 is {}.", part2(&vertices));
}
