import re


def parse():
    digit_re = re.compile(r"\d+")

    file = open("input.txt", "r")
    cards = []
    for line in file:
        line = line[line.find(":") + 1 :]
        splits = line.split("|")
        numbers = [int(n) for n in digit_re.findall(splits[0])]
        winning_numbers = [int(n) for n in digit_re.findall(splits[1])]

        cards += [[numbers, winning_numbers]]

    return cards


def part1(input):
    points = 0

    for card in input:
        numbers = card[0]
        winning_numbers = card[1]

        common_numbers = [number for number in numbers if number in winning_numbers]
        if len(common_numbers) > 0:
            points += 2 ** (len(common_numbers) - 1)

    return points


def part2(input):
    scratchpads = {}

    for i, card in enumerate(input):
        if i not in scratchpads:
            scratchpads[i] = 0
        scratchpads[i] += 1

        numbers = card[0]
        winning_numbers = card[1]
        common_numbers = [number for number in numbers if number in winning_numbers]

        for j in range(1, len(common_numbers) + 1):
            if j + i not in scratchpads:
                scratchpads[j + i] = 0
            scratchpads[j + i] += scratchpads[i]

    return sum(scratchpads.values())


cards = parse()
print("The solution to part 1 is {}.".format(part1(cards)))
print("The solution to part 2 is {}.".format(part2(cards)))
