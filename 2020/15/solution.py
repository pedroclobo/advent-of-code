file = open("input.txt", "r")
input = [int(x) for x in file.readline().split(",")]


def compute(round):
	time = 0
	plays = {}

	last = input[-1]
	for num in input:
		plays[num] = [time]
		time += 1

	while time < round:
		if last in plays and len(plays[last]) >= 2:
			last = plays[last][-1] - plays[last][-2]
			if last in plays:
				plays[last].append(time)
			else:
				plays[last] = [time]
		else:
			last = 0
			plays[last].append(time)

		time += 1

	return last


def part1():
	return compute(2020)


def part2():
	return compute(30000000)


print("The solution to part 1 is {}.".format(part1()))
print("The solution to part 2 is {}.".format(part2()))
