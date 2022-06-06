file = open("input.txt", "r")
input = [int(line) for line in file]


def check_sum(number, previous):
	for num in previous:
		if number - num in previous:
			return True

	return False


def part1():
	i = 25
	for num in input[25:]:
		if check_sum(num, input[i - 25:i]) == False:
			return num
		i += 1


def part2():
	sum = part1()

	for i in range(len(input)):
		sequence = [input[i]]
		curr_sum = input[i]
		j = i + 1

		while curr_sum < sum:
			curr_sum += input[j]
			sequence.append(input[j])
			j += 1

		if curr_sum == sum:
			return min(sequence) + max(sequence)


print("The solution to part 1 is {}.".format(part1()))
print("The solution to part 2 is {}.".format(part2()))
