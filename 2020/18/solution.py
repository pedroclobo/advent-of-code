def evaluate(tree):
	if not isinstance(tree[1], list) and not isinstance(tree[2], list):
		return eval(tree[1] + tree[0] + tree[2])
	elif not isinstance(tree[1], list):
		return eval(tree[1] + tree[0] + str(evaluate(tree[2])))
	elif not isinstance(tree[2], list):
		return eval(str(evaluate(tree[1])) + tree[0] + tree[2])
	else:
		return eval(str(evaluate(tree[1])) + tree[0] + str(evaluate(tree[2])))


def part1():

	def parse():
		file = open("input.txt", "r")

		trees = []
		line = file.readline().strip()
		while line:
			trees.append(parse_tree(line))
			line = file.readline().strip()

		return trees

	def parse_tree(line):
		tokens = [token for token in line if token != " "]
		operators = []
		output = []

		for token in tokens:
			if token.isdigit():
				output.append(token)
			elif token in ("+", "*"):
				while operators and operators[-1] != "(":
					output.append(operators.pop())
				operators.append(token)
			elif token == "(":
				operators.append(token)
			elif token == ")":
				while operators and operators[-1] != "(":
					output.append(operators.pop())
				operators.pop()

		while operators:
			output.append(operators.pop())

		# convert to tree
		stack = []
		for token in output:
			if token.isdigit():
				stack.append(token)
			else:
				m2 = stack.pop()
				m1 = stack.pop()
				stack.append([token, m1, m2])

		return stack.pop()

	sum = 0
	for tree in parse():
		sum += evaluate(tree)

	return sum


def part2():

	def parse():
		file = open("input.txt", "r")

		trees = []
		line = file.readline().strip()
		while line:
			trees.append(parse_tree(line))
			line = file.readline().strip()

		return trees

	def parse_tree(line):
		tokens = [token for token in line if token != " "]
		operators = []
		output = []

		for token in tokens:
			if token.isdigit():
				output.append(token)
			elif token in ("+", "*"):
				while operators and operators[-1] != "(" and operators[
				    -1] == "+":
					output.append(operators.pop())
				operators.append(token)
			elif token == "(":
				operators.append(token)
			elif token == ")":
				while operators and operators[-1] != "(":
					output.append(operators.pop())
				operators.pop()

		while operators:
			output.append(operators.pop())

		# convert to tree
		stack = []
		for token in output:
			if token.isdigit():
				stack.append(token)
			else:
				m2 = stack.pop()
				m1 = stack.pop()
				stack.append([token, m1, m2])

		return stack.pop()

	sum = 0
	for tree in parse():
		sum += evaluate(tree)

	return sum


print("The solution to part 1 is {}.".format(part1()))
print("The solution to part 2 is {}.".format(part2()))
