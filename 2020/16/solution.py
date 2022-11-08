import re
import numpy as np


def is_invalid(num, rules):
	for rule1, rule2 in rules.values():
		if rule1[0] <= num <= rule1[1] or rule2[0] <= num <= rule2[1]:
			return False

	return True


def parse():
	file = open("input.txt", "r")
	line = file.readline()

	# rules
	rules = {}
	while line != "\n":
		field_pattern = re.compile(r".*:")
		range_pattern = re.compile(r"(\d*)-(\d*)")
		field = field_pattern.match(line)[0][:-1]
		[n1, n2], [n3, n4] = range_pattern.findall(line)
		n1, n2, n3, n4 = int(n1), int(n2), int(n3), int(n4)

		rules[field] = [[n1, n2], [n3, n4]]

		line = file.readline()

	# my ticket
	_ = file.readline()
	my_ticket = [int(num) for num in file.readline().split(",")]

	# nearby tickets
	_ = file.readline()
	_ = file.readline()

	tickets = []
	line = file.readline()
	while line:
		tickets += [[int(num) for num in line.split(",")]]
		line = file.readline()

	return rules, my_ticket, tickets


def part1():
	rules, _, tickets = parse()

	invalid_sum = 0
	for ticket in tickets:
		for num in ticket:
			if is_invalid(num, rules):
				invalid_sum += num

	return invalid_sum


def fits(values, rules):
	rule1, rule2 = rules
	for num in values:
		if not (rule1[0] <= num <= rule1[1] or rule2[0] <= num <= rule2[1]):
			return False

	return True


def part2():
	rules, my_ticket, all_tickets = parse()

	# Remove invalid tickets
	tickets = []
	for ticket in all_tickets:
		if all(not is_invalid(num, rules) for num in ticket):
			tickets += [ticket]

	fields = list(rules.keys())
	field_values = np.array(tickets).transpose()
	mat = np.zeros([len(fields), len(fields)])

	# Assign 1 if field can hold values
	for i in range(len(fields)):
		for j in range(len(fields)):
			if fits(field_values[j], rules[fields[i]]):
				mat[i, j] = 1

	changed = True
	while changed:
		changed = False
		for i in range(len(fields)):
			if np.count_nonzero(mat[i] == 1) == 1:
				index = np.where(mat[i] == 1)[0][0]
				for j in range(len(fields)):
					if j != i and mat[j, index] != 0:
						mat[j, index] = 0
						changed = True

	field_indexes = {}
	for i in range(len(fields)):
		index = np.where(mat[i] == 1)[0][0]
		field_indexes[fields[i]] = index

	res = 1
	for field in field_indexes:
		if "departure" in field:
			res *= my_ticket[field_indexes[field]]

	return res


print("The solution to part 1 is {}.".format(part1()))
print("The solution to part 2 is {}.".format(part2()))
