file = open("input.txt", "r")
input = [x.strip() for x in file]


def part1(input):
    numbers = []

    for word in input:
        current_numbers = ()
        for letter in word:
            if letter.isdigit():
                current_numbers += (int(letter),)

        numbers.append(current_numbers)

    result = 0
    for number_set in numbers:
        result += number_set[0] * 10 + number_set[-1]

    return result


def part2(input):
    NUMBERS = {
        "one": 1,
        "two": 2,
        "three": 3,
        "four": 4,
        "five": 5,
        "six": 6,
        "seven": 7,
        "eight": 8,
        "nine": 9,
    }

    numbers = []
    for word in input:
        indices = ()

        # Find the indexes of the words
        for i in range(len(word)):
            for number in NUMBERS.keys():
                if word[i:].startswith(number):
                    indices += ((NUMBERS[number], i),)

        # Find the indexes of the numbers
        for i in range(len(word)):
            if word[i].isdigit():
                indices += ((int(word[i]), i),)

        numbers.append(
            (
                min(indices, key=lambda x: x[1])[0],
                max(indices, key=lambda x: x[1])[0],
            )
        )

    result = 0
    for number_set in numbers:
        result += number_set[0] * 10 + number_set[1]

    return result


print("The solution to part 1 is {}.".format(part1(input)))
print("The solution to part 2 is {}.".format(part2(input)))
