const X: usize = 25;
const Y: usize = 6;

fn parse() -> Vec<Vec<u8>> {
    let mut layers = Vec::new();
    let mut layer = Vec::new();

    let mut i = 0;
    for c in include_str!("input.txt").trim().chars().into_iter() {
        if i % (X * Y) == 0 && i != 0 {
            layers.push(layer.clone());
            layer.clear();
        }
        layer.push(c.to_digit(10).unwrap() as u8);
        i += 1;
    }
    layers.push(layer.clone());

    layers
}

fn part1(layers: &Vec<Vec<u8>>) -> usize {
    let mut minimum = usize::MAX;
    let mut index_maximum = 0;

    for i in 0..layers.len() {
        let number_zeros = layers[i].iter().filter(|&n| *n == 0).count();
        if number_zeros < minimum {
            minimum = number_zeros;
            index_maximum = i;
        }
    }

    layers[index_maximum].iter().filter(|&n| *n == 1).count()
        * layers[index_maximum].iter().filter(|&n| *n == 2).count()
}

fn part2(layers: &Vec<Vec<u8>>) {
    for j in 0..layers[0].len() {
        let mut i = 0;
        let mut node = layers[i][j];

        while node == 2 {
            i += 1;
            node = layers[i][j];
        }

        if j % X == 0 && j != 0 {
            print!("\n");
        }

        if node == 1 {
            print!("{}", node);
        } else {
            print!(" ");
        }
    }

    print!("\n");
}

fn main() {
    let layers = parse();
    println!("The solution to part 1 is {}.", part1(&layers));
    println!("The solution to part 2 is");
    part2(&layers);
}
