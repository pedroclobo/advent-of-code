use std::collections::HashMap;

fn parse_orbits() -> HashMap<String, String> {
    let mut orbits = HashMap::new();
    for line in include_str!("input.txt").lines() {
        let mut split = line.split(")");
        let key = split.next().unwrap().to_string();
        let value = split.next().unwrap().to_string();
        orbits.insert(value, key);
    }

    orbits
}

fn parse_paths() -> HashMap<String, Vec<String>> {
    let mut paths = HashMap::new();
    for line in include_str!("input.txt").lines() {
        let mut split = line.split(")");
        let key = split.next().unwrap().to_string();
        let value = split.next().unwrap().to_string();

        if !paths.contains_key(&key) {
            paths.insert(key.clone(), Vec::new());
        }
        if !paths.contains_key(&value) {
            paths.insert(value.clone(), Vec::new());
        }

        paths.get_mut(&key).unwrap().push(value.clone());
        paths.get_mut(&value).unwrap().push(key.clone());
    }

    paths
}

fn number_orbits(orbits: &HashMap<String, String>, planet: String) -> u32 {
    if !orbits.contains_key(&planet) {
        return 0;
    } else {
        return 1 + number_orbits(orbits, orbits.get(&planet).unwrap().to_string());
    }
}

fn part1(orbits: &HashMap<String, String>) -> u32 {
    let mut sum = 0;

    orbits.keys().for_each(|planet| {
        sum += number_orbits(orbits, planet.to_string());
    });

    sum
}

fn part2(paths: &HashMap<String, Vec<String>>) -> u32 {
    let mut visited = Vec::new();
    visited.push("YOU".to_string());

    let mut distance = HashMap::new();
    paths.keys().for_each(|planet| {
        distance.insert(planet.to_string(), u32::MAX);
    });
    distance.insert("YOU".to_string(), 0);

    while !visited.is_empty() {
        let planet = visited.pop().unwrap();
        for neighbor in paths.get(&planet).unwrap() {
            if *distance.get(neighbor).unwrap() == u32::MAX {
                visited.push(neighbor.to_string());
                distance.insert(neighbor.to_string(), distance.get(&planet).unwrap() + 1);
            }
        }
    }

    *distance.get("SAN").unwrap() - 2
}

fn main() {
    let orbits = parse_orbits();
    println!("The solution to part 1 is {}.", part1(&orbits));

    let paths = parse_paths();
    println!("The solution to part 2 is {}.", part2(&paths));
}
