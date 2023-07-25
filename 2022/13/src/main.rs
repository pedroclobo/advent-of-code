use core::str::FromStr;
use serde_json::Value;

struct PacketPair {
	left: Packet,
	right: Packet,
}

#[derive(Debug, Clone)]
enum Packet {
	Integer(u64),
	List(Vec<Packet>),
}

impl PartialEq for Packet {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Packet::Integer(a), Packet::Integer(b)) => a == b,
			(Packet::Integer(a), Packet::List(_)) => {
				let left = Packet::List(vec![Packet::Integer(*a)]);
				left == *other
			}
			(Packet::List(_), Packet::Integer(b)) => {
				let right = Packet::List(vec![Packet::Integer(*b)]);
				*self == right
			}
			(Packet::List(a), Packet::List(b)) => {
				for (a, b) in a.iter().zip(b.iter()) {
					if a != b {
						return false;
					}
				}
				a.len() == b.len()
			}
		}
	}
}

impl Eq for Packet {}

impl PartialOrd for Packet {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		match (self, other) {
			(Packet::Integer(a), Packet::Integer(b)) => a.partial_cmp(b),
			(Packet::Integer(a), Packet::List(_)) => {
				let left = Packet::List(vec![Packet::Integer(*a)]);
				left.partial_cmp(other)
			}
			(Packet::List(_), Packet::Integer(b)) => {
				let right = Packet::List(vec![Packet::Integer(*b)]);
				self.partial_cmp(&right)
			}
			(Packet::List(a), Packet::List(b)) => {
				for (a, b) in a.iter().zip(b.iter()) {
					match a.partial_cmp(b) {
						Some(std::cmp::Ordering::Equal) => (),
						x => return x,
					}
				}
				a.len().partial_cmp(&b.len())
			}
		}
	}
}

impl Ord for Packet {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		match (self, other) {
			(Packet::Integer(a), Packet::Integer(b)) => a.cmp(b),
			(Packet::Integer(a), Packet::List(_)) => {
				let left = Packet::List(vec![Packet::Integer(*a)]);
				left.cmp(other)
			}
			(Packet::List(_), Packet::Integer(b)) => {
				let right = Packet::List(vec![Packet::Integer(*b)]);
				self.cmp(&right)
			}
			(Packet::List(a), Packet::List(b)) => {
				for (a, b) in a.iter().zip(b.iter()) {
					match a.cmp(b) {
						std::cmp::Ordering::Equal => (),
						x => return x,
					}
				}
				a.len().cmp(&b.len())
			}
		}
	}
}

#[derive(Debug)]
struct PacketParserError;
impl FromStr for Packet {
	type Err = PacketParserError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		fn from_json(input: &Value) -> Packet {
			match input {
				Value::Number(x) => Packet::Integer(x.as_u64().expect("error parsing integer")),
				Value::Array(a) => Packet::List(a.iter().map(from_json).collect()),
				_ => panic!("value is neither number nor array"),
			}
		}

		let json: serde_json::Value = serde_json::from_str(s).unwrap();

		let items = json
			.as_array()
			.expect("invalid input")
			.iter()
			.map(from_json)
			.collect();

		Ok(Packet::List(items))
	}
}

fn parse_packets() -> Vec<Packet> {
	let mut result = Vec::new();

	for pair in include_str!("input.txt").split("\n\n") {
		let (left, right) = pair.split_at(pair.find('\n').unwrap());

		result.push(Packet::from_str(left.trim()).unwrap());
		result.push(Packet::from_str(right.trim()).unwrap());
	}

	result
}

fn parse_pairs() -> Vec<PacketPair> {
	let mut result = Vec::new();

	for pair in include_str!("input.txt").split("\n\n") {
		let (left, right) = pair.split_at(pair.find('\n').unwrap());

		result.push(PacketPair {
			left: Packet::from_str(left.trim()).unwrap(),
			right: Packet::from_str(right.trim()).unwrap(),
		});
	}

	result
}

fn part1(pairs: &Vec<PacketPair>) -> usize {
	let mut sum = 0;

	for (i, PacketPair { left, right }) in pairs.iter().enumerate() {
		if left < right {
			sum += i + 1;
		}
	}

	sum
}

fn part2(packets: &mut Vec<Packet>) -> usize {
	let dividers = vec![
		Packet::from_str("[[2]]").unwrap(),
		Packet::from_str("[[6]]").unwrap(),
	];
	packets.extend_from_slice(&dividers);
	packets.sort();

	let mut signal = 1;
	for (i, packet) in packets.into_iter().enumerate() {
		if dividers.contains(packet) {
			signal *= i + 1;
		}
	}

	signal
}

fn main() {
	println!("The solution to part 1 is {}.", part1(&parse_pairs()));
	println!("The solution to part 2 is {}.", part2(&mut parse_packets()));
}
