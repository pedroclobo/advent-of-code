import copy

file = open("input.txt", "r")
input = [line.strip().split(" ") for line in file]


def parse_input(input):
	for i in range(len(input)):
		input[i] = [input[i][0], int(input[i][1])]

	return input


def part1():

	def calculate_acc(i, visited):
		if i == len(input) or i in visited:
			return 0

		[cmd, value] = input[i]

		if cmd == "nop":
			return calculate_acc(i + 1, visited + [i])
		elif cmd == "acc":
			return value + calculate_acc(i + 1, visited + [i])
		elif cmd == "jmp":
			return calculate_acc(i + value, visited + [i])

	return calculate_acc(0, [])


def part2():

	def calculate_acc(mutation, i):
		if i == len(mutation):
			return 0

		[cmd, value] = mutation[i]

		if cmd == "nop":
			return calculate_acc(mutation, i + 1)
		elif cmd == "acc":
			return value + calculate_acc(mutation, i + 1)
		elif cmd == "jmp":
			return calculate_acc(mutation, i + value)

	def check_cycle(mutation, i, visited):
		if i == len(mutation):
			return False

		if i in visited:
			return True

		[cmd, value] = mutation[i]

		if cmd == "nop":
			return False or check_cycle(mutation, i + 1, visited + [i])
		elif cmd == "acc":
			return False or check_cycle(mutation, i + 1, visited + [i])
		elif cmd == "jmp":
			return False or check_cycle(mutation, i + value, visited + [i])

	visited = []
	while True:
		mutation = copy.deepcopy(input)
		for i in range(len(mutation)):
			cmd = mutation[i][0]
			if cmd == "nop" and i not in visited:
				mutation[i][0] = "jmp"
				visited.append(i)
				break
			elif cmd == "jmp" and i not in visited:
				mutation[i][0] = "nop"
				visited.append(i)
				break

		if not check_cycle(mutation, 0, []):
			return calculate_acc(mutation, 0)


input = parse_input(input)

print("The solution to part 1 is {}.".format(part1()))
print("The solution to part 2 is {}.".format(part2()))
