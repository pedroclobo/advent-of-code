from math import floor, ceil

file = open("input.txt", "r")
passports = [passport.strip() for passport in file]

def get_position(passport):
	row_min, row_max = 0, 127
	column_min, column_max = 0, 7

	for letter in passport:
		if letter == 'F':
			row_max = row_min + floor((row_max - row_min) / 2)
			row = row_max
		elif letter == 'B':
			row_min = row_min + ceil((row_max - row_min) / 2)
			row = row_min
		elif letter == 'L':
			column_max = column_min + floor((column_max - column_min) / 2)
			column = column_min
		elif letter == 'R':
			column_min = column_min + ceil((column_max - column_min) / 2)
			column = column_max

	return (row, column)

def get_id(passport):
	(row, column) = get_position(passport)
	return 8 * row + column

def part1(passports):
	ids = [get_id(passport) for passport in passports]
	return max(ids)

def part2(passports):
	ids = [get_id(passport) for passport in passports]

	for id in range(part1(passports)):
		if id not in ids and id-1 in ids and id+1 in ids:
			return id

print("The solution to part 1 is {}.".format(part1(passports)))
print("The solution to part 2 is {}.".format(part2(passports)))
