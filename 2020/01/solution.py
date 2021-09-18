file = open("input.txt", "r")
input = [int(x) for x in file]

def part1(input, num):
	for i in range(len(input)):
		if num - input[i] in input:
			return input[i] * (num - input[i])

def part2(input, num):
	for i in range(len(input)):
		for j in range(i, len(input)):
			# make sure the numbers are all diferent
			third = num - input[i] - input[j]
			if third in input and third != input[i] and third != input[j]:
				return input[i] * input[j] * (num - input[i] - input[j])

print("The solution to part 1 is {}.".format(part1(input, 2020)))
print("The solution to part 2 is {}.".format(part2(input, 2020)))
