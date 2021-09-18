import re

file = open("input.txt", "r")
pattern = r"(\d+)-(\d+) (\w): (\w+)"
input = [re.match(pattern, line).groups() for line in file]

def count_valid_passwords(input, criteria):
	valid = 0
	for (min, max, char, password) in input:
	    if criteria(password, char, int(min), int(max)):
	        valid += 1

	return valid

def part1(password, char, min, max):
	return min <= password.count(char) <= max

def part2(password, char, min, max):
	min -= 1
	max -= 1
	return ((password[min] == char or password[max] == char) and not \
	        (password[min] == char and password[max] == char))

print("The solution to part 1 is {}.".format(count_valid_passwords(input, part1)))
print("The solution to part 2 is {}.".format(count_valid_passwords(input, part2)))
