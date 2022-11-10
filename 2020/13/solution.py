from sympy.ntheory.modular import crt

file = open("input.txt", "r")

departure = int(file.readline())
input = [line.split(",") for line in file]
buses = [int(bus) if bus != 'x' else None for bus in input[0]]


def part1(departure, buses):
	time = departure

	while True:
		for bus in buses:
			if bus != None and time % bus == 0:
				return bus * (time - departure)

		time += 1


def part2(buses):

	# time remainder 0          mod bus[0]
	# time remainder bus[n] - n mod bus[n]
	mods = [buses[0]]
	remainders = [0]
	for i in range(1, len(buses)):
		if buses[i] != None:
			mods.append(buses[i])
			remainders.append(buses[i] - i)

	return crt(mods, remainders)[0]


print("The solution to part 1 is {}.".format(part1(departure, buses)))
print("The solution to part 2 is {}.".format(part2(buses)))
