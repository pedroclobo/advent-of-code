file = open("input.txt", "r")
input = [line.strip() for line in file]

def read_answers_anyone(input):
	answers = []
	answer = []

	for line in input:
		if line == '':
			answers += [answer]
			answer = []
		else:
			for char in line:
				if char not in answer:
					answer += [char]

	answers += [answer]
	answer = []

	return answers

def common_answers(group):
	answer = list(group[0])
	for i in range(1, len(group)):
		answer = list(set(answer).intersection(group[i]))

	return answer

def read_answers_everyone(input):
	answers = []
	group = []

	for line in input:
		if line == '':
			answers += [common_answers(group)]
			group = []
		else:
			group += [line]

	answers += [common_answers(group)]
	group = []

	return answers

def count_answers(answers):
	count = 0
	for group in answers:
		count += len(group)

	return count

print("The solution to part 1 is {}.".format(count_answers(read_answers_anyone(input))))
print("The solution to part 2 is {}.".format(count_answers(read_answers_everyone(input))))
