file = open("input.txt", "r")
input = [line.strip() for line in file]

def is_tree(position):
	return position == '#'

def count_trees(input, x_inc, y_inc):
	x_len = len(input[0])
	y_len = len(input)
	x, y = 0, 0
	count = 0

	while y < y_len:
	    if is_tree(input[y][x]):
	        count += 1
	    x = (x + x_inc) % x_len
	    y += y_inc

	return count

def part1(input):
	return count_trees(input, 3, 1)

def part2(input):
	slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
	count = 1

	for (x, y) in slopes:
	    count *= count_trees(input, x, y)

	return count

print("The solution to part 1 is {}.".format(part1(input)))
print("The solution to part 1 is {}.".format(part2(input)))
