def part1():

	def parse():
		file = open("input.txt", "r")

		grid = []
		line = file.readline().strip()
		while line:
			grid.append(list(line))
			line = file.readline().strip()

		return [grid]

	def get_neighbours(node):
		z, y, x = node
		neighbours = []

		for i in range(-1, 2):
			for j in range(-1, 2):
				for k in range(-1, 2):
					if (i, j, k) != (0, 0, 0):
						neighbours.append((z + i, y + j, x + k))

		return neighbours

	def get_active_nodes(grid):
		active_nodes = []
		for i in range(len(grid)):
			for j in range(len(grid[0])):
				for k in range(len(grid[0][0])):
					if grid[i][j][k] == "#":
						active_nodes.append((i, j, k))

		return active_nodes

	def perform_cycle(active_nodes):
		new_active_nodes = set()
		inactive_neighbours = set()

		# active nodes
		for active_node in active_nodes:
			neighbours = get_neighbours(active_node)
			active_neighbours = [
			    neighbour for neighbour in neighbours
			    if neighbour in active_nodes
			]
			inactive_neighbours = inactive_neighbours.union({
			    neighbour
			    for neighbour in neighbours if neighbour not in active_nodes
			})
			if len(active_neighbours) in (2, 3):
				new_active_nodes.add(active_node)

		# inactive nodes
		for inactive_node in inactive_neighbours:
			neighbours = get_neighbours(inactive_node)
			active_neighbours = len([
			    neighbour for neighbour in neighbours
			    if neighbour in active_nodes
			])
			if active_neighbours == 3:
				new_active_nodes.add(inactive_node)

		return new_active_nodes

	active_nodes = get_active_nodes(parse())
	for _ in range(6):
		active_nodes = perform_cycle(active_nodes)

	return len(active_nodes)


def part2():

	def parse():
		file = open("input.txt", "r")

		grid = []
		line = file.readline().strip()
		while line:
			grid.append(list(line))
			line = file.readline().strip()

		return [[grid]]

	def get_neighbours(node):
		z, w, y, x = node
		neighbours = []

		for i in range(-1, 2):
			for j in range(-1, 2):
				for k in range(-1, 2):
					for m in range(-1, 2):
						if (i, j, k, m) != (0, 0, 0, 0):
							neighbours.append((z + i, w + j, y + k, x + m))

		return neighbours

	def get_active_nodes(grid):
		active_nodes = []
		for i in range(len(grid)):
			for j in range(len(grid[0])):
				for k in range(len(grid[0][0])):
					for m in range(len(grid[0][0][0])):
						if grid[i][j][k][m] == "#":
							active_nodes.append((i, j, k, m))

		return active_nodes

	def perform_cycle(active_nodes):
		new_active_nodes = set()
		inactive_neighbours = set()

		# active nodes
		for active_node in active_nodes:
			neighbours = get_neighbours(active_node)
			active_neighbours = [
			    neighbour for neighbour in neighbours
			    if neighbour in active_nodes
			]
			inactive_neighbours = inactive_neighbours.union({
			    neighbour
			    for neighbour in neighbours if neighbour not in active_nodes
			})
			if len(active_neighbours) in (2, 3):
				new_active_nodes.add(active_node)

		# inactive nodes
		for inactive_node in inactive_neighbours:
			neighbours = get_neighbours(inactive_node)
			active_neighbours = len([
			    neighbour for neighbour in neighbours
			    if neighbour in active_nodes
			])
			if active_neighbours == 3:
				new_active_nodes.add(inactive_node)

		return new_active_nodes

	active_nodes = get_active_nodes(parse())
	for _ in range(6):
		active_nodes = perform_cycle(active_nodes)

	return len(active_nodes)


print("The solution to part 1 is {}.".format(part1()))
print("The solution to part 2 is {}.".format(part2()))
