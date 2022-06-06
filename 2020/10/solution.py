file = open("input.txt", "r")
input = [int(line) for line in file]


def part1():
	maximum = max(input) + 3

	sorted_input = sorted(input + [0, maximum])
	diffs = [
	    sorted_input[i + 1] - sorted_input[i]
	    for i in range(len(sorted_input) - 1)
	]

	return diffs.count(1) * diffs.count(3)


def part2():

	def count_paths(num, target, mem):
		if num in mem:
			return mem[num]

		if num == target:
			return 1

		sum = 0
		for j in (1, 2, 3):
			if num + j in expanded_input:
				sum += count_paths(num + j, target, mem)

		mem[num] = sum

		return sum

	maximum = max(input) + 3
	expanded_input = input + [0, maximum]

	return count_paths(0, maximum, {})


print("The solution to part 1 is {}.".format(part1()))
print("The solution to part 2 is {}.".format(part2()))
