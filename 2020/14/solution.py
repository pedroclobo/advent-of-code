import copy
import itertools
import re


def masking(mask, value):
	mask = list(mask)

	# list of bits
	value = list(str(bin(value))[2:])

	# make sure there are 36 bits
	while len(value) != 36:
		value = ["0"] + value

	for i in range(36):
		if mask[i] == "X":
			continue
		else:
			value[i] = mask[i]

	# convert value into string again
	value = "".join([str(i) for _, i in enumerate(value)])

	return int(value, 2)


def masking_v2(mask, value):
	mask = list(mask)

	# list of bits
	value = list(str(bin(value))[2:])

	# make sure there are 36 bits
	while len(value) != 36:
		value = ["0"] + value

	x_count = 0
	for i in range(36):
		if mask[i] == '0':
			continue
		else:
			value[i] = mask[i]
			if mask[i] == "X":
				x_count += 1

	results = []
	# cartesian product
	products = list(itertools.product('01', repeat=x_count))
	i = 0
	while i < 2**x_count:
		value_copy = copy.deepcopy(value)
		j = 0
		for index in range(len(value_copy)):
			if value_copy[index] == 'X':
				value_copy[index] = products[i][j]
				j += 1

		value_copy = "".join([str(i) for _, i in enumerate(value_copy)])
		results.append(int(value_copy, 2))
		i += 1

	return results


def part1():
	file = open("input.txt", "r")

	array = {}

	mask = None
	line = file.readline()
	while (line):
		if "mask" in line:
			mask = line[7:]
		else:
			index_pattern = re.compile(r"\[.*\]")
			value_pattern = re.compile(r"=\ .*")
			index = int(index_pattern.findall(line)[0][1:-1])
			value = int(value_pattern.findall(line)[0][2:])

			array[index] = masking(mask, value)

		line = file.readline()

	return sum(array.values())


def part2():
	file = open("input.txt", "r")

	array = {}

	mask = None
	line = file.readline()
	while (line):
		if "mask" in line:
			mask = line[7:]
		else:
			index_pattern = re.compile(r"\[.*\]")
			value_pattern = re.compile(r"=\ .*")
			index = int(index_pattern.findall(line)[0][1:-1])
			value = int(value_pattern.findall(line)[0][2:])

			indexes = masking_v2(mask, index)
			for index in indexes:
				array[index] = value

		line = file.readline()

	return sum(array.values())


print("The solution to part 1 is {}.".format(part1()))
print("The solution to part 2 is {}.".format(part2()))
