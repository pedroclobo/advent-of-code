import math

file = open("input.txt", "r")
input = [(line[0], int(line[1:])) for line in file]


def part1(navegation):

	def change_direction(ship_direction, turn_direction, degrees):
		if turn_direction == "L":
			return (ship_direction + degrees) % 360
		elif turn_direction == "R":
			return (ship_direction - degrees) % 360

	ns, we = 0, 0

	# degrees relative to east
	ship_direction = 0

	for (direction, steps) in navegation:
		if direction == "N":
			ns += steps
		elif direction == "S":
			ns -= steps
		elif direction == "W":
			we += steps
		elif direction == "E":
			we -= steps
		elif direction == "L":
			ship_direction = change_direction(ship_direction, "L", steps)
		elif direction == "R":
			ship_direction = change_direction(ship_direction, "R", steps)
		elif direction == "F":
			if ship_direction == 90:
				ns += steps
			elif ship_direction == 270:
				ns -= steps
			elif ship_direction == 180:
				we += steps
			elif ship_direction == 0:
				we -= steps

	return abs(ns) + abs(we)


def part2(navegation):

	# math is beautiful
	def rotate(way_point, deg):
		cos = int(math.cos(math.radians(deg)))
		sin = int(math.sin(math.radians(deg)))

		w, n = way_point

		return [w * cos + n * sin, n * cos - w * sin]

	ns, we = 0, 0

	# (west, north)
	way_point = [-10, 1]

	for (direction, steps) in navegation:
		if direction == "N":
			way_point[1] += steps
		elif direction == "S":
			way_point[1] -= steps
		elif direction == "W":
			way_point[0] += steps
		elif direction == "E":
			way_point[0] -= steps
		elif direction == "L":
			way_point = rotate(way_point, steps)
		elif direction == "R":
			way_point = rotate(way_point, -steps)
		elif direction == "F":
			we += steps * way_point[0]
			ns += steps * way_point[1]

	return abs(ns) + abs(we)


print("The solution to part 1 is {}.".format(part1(input)))
print("The solution to part 2 is {}.".format(part2(input)))
