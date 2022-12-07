use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
struct Directory {
	name: String,
	size: RefCell<usize>,
	parent: Option<Rc<Directory>>,
	children: RefCell<HashMap<String, Rc<Directory>>>,
}

impl Directory {
	fn new(
		name: String,
		size: RefCell<usize>,
		parent: Option<Rc<Directory>>,
		children: RefCell<HashMap<String, Rc<Directory>>>,
	) -> Self {
		Self {
			name,
			size,
			parent,
			children,
		}
	}

	fn get_total_size(&self) -> usize {
		if self.children.borrow().len() == 0 {
			*self.size.borrow()
		} else {
			let mut total_size = *self.size.borrow();
			for child in self.children.borrow().iter().map(|(_, dir)| dir) {
				total_size += child.get_total_size()
			}
			total_size
		}
	}
}

fn parse() -> Vec<Rc<Directory>> {
	let root_dir = Rc::new(Directory::new(
		"/".to_string(),
		RefCell::new(0),
		None,
		RefCell::new(HashMap::new()),
	));

	let mut directories = Vec::new();
	directories.push(Rc::clone(&root_dir));

	let mut cwd = Rc::clone(&root_dir);

	for line in include_str!("input.txt").lines() {
		if line.starts_with("$ cd") {
			let dir_name = line.split_whitespace().nth(2).unwrap();
			match dir_name {
				"/" => continue,
				".." => cwd = Rc::clone(&cwd.parent.as_ref().expect("no parent found")),
				_ => {
					let new_dir = cwd.children.borrow().get(dir_name).unwrap().clone();
					cwd = new_dir;
				}
			}
		} else if line.starts_with("$ ls") {
			continue;
		} else if line.starts_with("dir") {
			let dir_name = line.split_whitespace().nth(1).unwrap();

			let new_dir = Rc::new(Directory::new(
				dir_name.to_string(),
				RefCell::new(0),
				Some(Rc::clone(&cwd)),
				RefCell::new(HashMap::new()),
			));

			directories.push(Rc::clone(&new_dir));

			cwd.children
				.borrow_mut()
				.insert(dir_name.to_string(), new_dir);
		} else {
			let file_size = line
				.split_whitespace()
				.nth(0)
				.unwrap()
				.parse::<usize>()
				.unwrap();

			*cwd.size.borrow_mut() += file_size;
		}
	}

	directories
}

fn part1(directories: &Vec<Rc<Directory>>) -> usize {
	directories
		.into_iter()
		.filter(|x| x.get_total_size() <= 100000)
		.map(|x| x.get_total_size())
		.into_iter()
		.sum()
}

fn part2(directories: &Vec<Rc<Directory>>) -> usize {
	let root_dir = directories.iter().find(|x| x.name == "/").unwrap();
	let free_space = 70000000 - root_dir.get_total_size();
	let to_free = 30000000 - free_space;

	directories
		.into_iter()
		.filter(|x| x.get_total_size() >= to_free)
		.map(|x| x.get_total_size())
		.min()
		.unwrap()
}

fn main() {
	let directories = parse();
	println!("The solution to part 1 is {}.", part1(&directories));
	println!("The solution to part 2 is {}.", part2(&directories));
}
