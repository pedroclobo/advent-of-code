from graph import Graph

file = open("input.txt", "r")
input = [line.strip() for line in file]


def parse(input):

	def get_bags(line):
		result = []

		while len(line) > 0:
			result.append((line[1] + ' ' + line[2], int(line[0])))
			del line[0:4]

		return result

	result = {}

	for line in input:
		line = line.split(" ")
		bag = line[0] + ' ' + line[1]
		if "no" in line:
			result[bag] = []
		else:
			result[bag] = get_bags(line[4:])

	return result


def part1():
	g = Graph(input.keys())

	for bag in input:
		sub_bags = input[bag]
		if sub_bags != []:
			for sub_bags in input[bag]:
				(sub_bag, value) = (sub_bags[0], sub_bags[1])
				g.add_edge(sub_bag, bag, value)

	return g.part1('shiny gold')


def part2():
	g = Graph(input.keys())

	for bag in input:
		sub_bags = input[bag]
		if sub_bags != []:
			for sub_bags in input[bag]:
				(sub_bag, value) = (sub_bags[0], sub_bags[1])
				g.add_edge(bag, sub_bag, value)

	return g.part2('shiny gold')


input = parse(input)

print("The solution to part 1 is {}.".format(part1()))
print("The solution to part 2 is {}.".format(part2()))
