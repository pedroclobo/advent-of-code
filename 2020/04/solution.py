import re

file = open("input.txt", "r")
input = [line.strip() for line in file]

def read_passports(input):
	passports = []
	passport = {}

	for i in range(len(input)):
		if input[i] == '':
			passports += [passport]
			passport = {}
		else:
			for field in fields:
				match = re.search(r"{}:#*(\w+)".format(field), input[i])
				if match:
					match = match.group()
					passport[field] = match[4:]

	# Add last passport
	passports += [passport]

	return passports


def count_valid_passports(passports, criteria):
	valid = 0

	for passport in passports:
		if criteria(passport):
			valid += 1

	return valid


def part1(passport):
	fields_without_cid = fields.copy()
	fields_without_cid.remove("cid")
	return all(field in passport for field in fields) or \
	       all(field in passport for field in fields_without_cid)


def is_valid_byr(passport):
	return 1920 <= int(passport["byr"]) <= 2002

def is_valid_iyr(passport):
	return 2010 <= int(passport["iyr"]) <= 2020

def is_valid_eyr(passport):
	return 2020 <= int(passport["eyr"]) <= 2030

def is_valid_hgt(passport):
	height = int(re.sub(r"\D", "", passport["hgt"]))
	if "cm" in passport["hgt"]:
		return 150 <= height <= 193
	elif "in" in passport["hgt"]:
		return 59 <= height <= 76

def is_valid_hcl(passport):
	return bool(re.search(r"#[0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f]", passport["hcl"]))

def is_valid_ecl(passport):
	return passport["ecl"] in ("amb", "blu", "brn", "gry", "grn", "hzl", "oth")

def is_valid_pid(passport):
	return len(passport["pid"]) == 9 and passport["pid"].isdigit()

def part2(passport):
	return part1(passport) and \
	       is_valid_byr(passport) and \
	       is_valid_iyr(passport) and \
	       is_valid_eyr(passport) and \
	       is_valid_hgt(passport) and \
	       is_valid_hcl(passport) and \
	       is_valid_ecl(passport) and \
	       is_valid_pid(passport)


fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"]
passports = read_passports(input)

print("The solution to part 1 is {}.".format(count_valid_passports(passports, part1)))
print("The solution to part 2 is {}.".format(count_valid_passports(passports, part2)))
