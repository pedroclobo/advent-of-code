from functools import reduce

file = open("input.txt", "r")
input = [[c for c in line.strip()] for line in file]


def number_coords(input):
    numbers = []

    is_number = False
    current_number = ""
    current_coord = []

    for i in range(len(input)):
        for j in range(len(input[i])):
            current_symbol = input[i][j]

            if current_symbol.isdigit():
                is_number = True
                current_number += current_symbol
                current_coord += ((i, j),)
            elif is_number:
                numbers += [(int(current_number), current_coord)]
                current_number = ""
                current_coord = []
                is_number = False

    return numbers


def part1(input):
    sum = 0
    n = 0
    is_part = False

    for i in range(len(input)):
        for j in range(len(input[i])):
            current_symbol = input[i][j]

            if current_symbol.isdigit():
                n = n * 10 + int(current_symbol)

                for x in (-1, 0, 1):
                    for y in (-1, 0, 1):
                        try:
                            symbol = input[i + x][j + y]
                            if symbol != "." and not symbol.isdigit():
                                is_part = True
                        except IndexError:
                            pass

            else:
                if is_part:
                    sum += n
                n = 0
                is_part = False

    return sum


def part2(input):
    nums = number_coords(input)
    gear_ratios = 0

    for i in range(len(input)):
        for j in range(len(input[i])):
            if input[i][j] == "*":
                coords = [(i + x, j + y) for x in (-1, 0, 1) for y in (-1, 0, 1)]

                part_numbers = []
                for coord in coords:
                    for num, num_coords in nums:
                        if coord in num_coords and num not in part_numbers:
                            part_numbers += [num]

                if len(part_numbers) == 2:
                    gear_ratios += reduce(lambda x, y: x * y, part_numbers)

    return gear_ratios


print("The solution to part 1 is {}.".format(part1(input)))
print("The solution to part 2 is {}.".format(part2(input)))
