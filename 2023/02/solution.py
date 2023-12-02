import re
from functools import reduce

file = open("input.txt", "r")
input = [x.strip() for x in file]


def parse(input):
    set_re = re.compile(r"(\d+) (red|green|blue)")

    parsed = []

    for line in input:
        line = line[line.index(":") + 1 :]
        sets = line.split(";")
        line_parsed = ()

        for set in sets:
            set = set.strip()
            set_parsed = ()
            for match in set_re.finditer(set):
                set_parsed += (
                    (
                        int(match.group(1)),
                        match.group(2),
                    ),
                )
            line_parsed += (set_parsed,)

        parsed.append(line_parsed)

    return parsed


def part1(input):
    def is_possible(game, bag):
        for set in game:
            for reveal in set:
                quantity = reveal[0]
                color = reveal[1]

                if bag[color] < quantity:
                    return False

        return True

    sum = 0
    bag = {"red": 12, "green": 13, "blue": 14}

    for id, game in enumerate(input):
        if is_possible(game, bag):
            sum += id + 1

    return sum


def part2(input):
    def fewest_cubes(game):
        max = {"red": -1, "green": -1, "blue": -1}

        for set in game:
            for reveal in set:
                quantity = reveal[0]
                color = reveal[1]

                if max[color] == -1 or quantity > max[color]:
                    max[color] = quantity

        return max

    sum = 0
    for game in input:
        cubes = fewest_cubes(game)
        sum += reduce(lambda x, y: x * y, cubes.values())

    return sum

input = parse(input)
print("The solution to part 1 is {}.".format(part1(input)))
print("The solution to part 2 is {}.".format(part2(input)))
