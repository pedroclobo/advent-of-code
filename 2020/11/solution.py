import copy

file = open("input.txt", "r")
input = [list(line.split()[0]) for line in file]


def count_ocupied_seats(map):
	count = 0

	for i in range(len(map)):
		for j in range(len(map[i])):
			if map[i][j] == "#":
				count += 1

	return count


def part1(map):

	def get_adjacent_seats(map, i, j):
		seats = []

		for inc_x in (-1, 0, 1):
			for inc_y in (-1, 0, 1):
				if 0 <= i + inc_y < len(map) and \
				   0 <= j + inc_x < len(map[i]) and \
				   (inc_x, inc_y) != (0, 0):
					seats.append(map[i + inc_y][j + inc_x])

		return seats

	updated = True

	while updated:
		updated = False
		new_map = copy.deepcopy(map)
		for i in range(len(map)):
			for j in range(len(map[i])):
				seat = map[i][j]
				adjacent = get_adjacent_seats(map, i, j)
				if seat == 'L' and adjacent.count("#") == 0:
					new_map[i][j] = "#"
					updated = True
				elif seat == "#" and adjacent.count("#") >= 4:
					new_map[i][j] = "L"
					updated = True

		map = new_map

	return count_ocupied_seats(map)


def part2(map):

	def get_adjacent_seats(map, i, j):
		seats = []

		# left
		y, x = i, j
		while 0 <= x < len(map[0]):
			if map[y][x] != "." and (y, x) != (i, j):
				seats.append(map[y][x])
				break
			x -= 1

		# right
		y, x = i, j
		while 0 <= x < len(map[0]):
			if map[y][x] != "." and (y, x) != (i, j):
				seats.append(map[y][x])
				break
			x += 1

		# down
		y, x = i, j
		while 0 <= y < len(map):
			if map[y][x] != "." and (y, x) != (i, j):
				seats.append(map[y][x])
				break
			y += 1

		# up
		y, x = i, j
		while 0 <= y < len(map):
			if map[y][x] != "." and (y, x) != (i, j):
				seats.append(map[y][x])
				break
			y -= 1

		# diag left up
		y, x = i, j
		while 0 <= x < len(map[0]) and 0 <= y < len(map):
			if map[y][x] != "." and (y, x) != (i, j):
				seats.append(map[y][x])
				break
			x -= 1
			y -= 1

		# diag left down
		y, x = i, j
		while 0 <= x < len(map[0]) and 0 <= y < len(map):
			if map[y][x] != "." and (y, x) != (i, j):
				seats.append(map[y][x])
				break
			x -= 1
			y += 1

		# diag right up
		y, x = i, j
		while 0 <= x < len(map[0]) and 0 <= y < len(map):
			if map[y][x] != "." and (y, x) != (i, j):
				seats.append(map[y][x])
				break
			x += 1
			y -= 1

		# diag right down
		y, x = i, j
		while 0 <= x < len(map[0]) and 0 <= y < len(map):
			if map[y][x] != "." and (y, x) != (i, j):
				seats.append(map[y][x])
				break
			x += 1
			y += 1

		return seats

	updated = True

	while updated:
		updated = False
		new_map = copy.deepcopy(map)
		for i in range(len(map)):
			for j in range(len(map[i])):
				seat = map[i][j]
				adjacent = get_adjacent_seats(map, i, j)
				if seat == 'L' and adjacent.count("#") == 0:
					new_map[i][j] = "#"
					updated = True
				elif seat == "#" and adjacent.count("#") >= 5:
					new_map[i][j] = "L"
					updated = True

		map = new_map

	return count_ocupied_seats(map)


print("The solution to part 1 is {}.".format(part1(input)))
print("The solution to part 2 is {}.".format(part2(input)))
