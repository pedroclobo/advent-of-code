#include<iostream>
#include<fstream>
#include<vector>
#include<bits/stdc++.h>

#define B_SIZE 5

using namespace std;

typedef struct {
	int b[B_SIZE][B_SIZE];
} board;

vector<int> numbers;
vector<board> bingos;

void build_bingo(const vector<int> bingo_v) {
	board bingo;
	int line = 0, col = 0;

	for (int number: bingo_v) {
		bingo.b[line][(col++) % B_SIZE] = number;
		if (col % B_SIZE == 0) {
			line++;
		}
	}

	bingos.push_back(bingo);
}

void parse() {
	ifstream file;
	file.open("input.txt");

	// Parse bingo numbers
	string line;
	file >> line;
	stringstream input(line);

	for (int number; input >> number;) {
		numbers.push_back(number);
		if (input.peek() == ',') {
			input.ignore();
		}
	}


	// Parse bingo boards
	int number;
	vector<int> bingo_numbers;
	while (file >> number) {
		bingo_numbers.push_back(number);
	}

	vector<int> bingo;
	for (size_t i = 0; i < bingo_numbers.size(); i++) {
		if (i > 0 && (i % (B_SIZE * B_SIZE) == 0)) {
			build_bingo(bingo);
			bingo.clear();
		}
		bingo.push_back(bingo_numbers[i]);
	}
	build_bingo(bingo);
	bingo.clear();
}

void check_board_number(board *b, int number) {
	for (int line = 0; line < B_SIZE; line++) {
		for (int col = 0; col < B_SIZE; col++) {
			if (b->b[line][col] == number) {
				b->b[line][col] = -1;
			}
		}
	}
}

void check_number(int number) {
	size_t size = bingos.size();

	for (size_t i = 0; i < size; i++) {
		check_board_number(&bingos[i], number);
	}
}

bool is_winner(board *b) {
	for (int line = 0; line < B_SIZE; line++) {
		int eq = 0;
		for (int col = 0; col < B_SIZE; col++) {
			if (b->b[line][col] == -1) {
				eq++;
				if (eq == B_SIZE) {
					return true;
				}
			}
		}
	}

	for (int col = 0; col < B_SIZE; col++) {
		int eq = 0;
		for (int line = 0; line < B_SIZE; line++) {
			if (b->b[line][col] == -1) {
				eq++;
				if (eq == B_SIZE) {
					return true;
				}
			}
		}
	}

	return false;
}

int has_winner() {
	size_t size = bingos.size();

	for (size_t i = 0; i < size; i++) {
		if (is_winner(&bingos[i])) {
			return i;
		}
	}

	return -1;
}

int get_number_winners() {
	int winners = 0;
	size_t size = bingos.size();

	for (size_t i = 0; i < size; i++) {
		if (is_winner(&bingos[i])) {
			winners++;
		}
	}

	return winners;
}

int get_board_sum(board *b) {
	int sum = 0;

	for (int line = 0; line < B_SIZE; line++) {
		for (int col = 0; col < B_SIZE; col++) {
			if (b->b[line][col] != -1) {
				sum += b->b[line][col];
			}
		}
	}

	return sum;
}

int loser() {
	size_t size = bingos.size();

	for (size_t i = 0; i < size; i++) {
		if (!is_winner(&bingos[i])) {
			return i;
		}
	}

	return -1;
}

int part1() {
	int winner;
	for (int number: numbers) {
		check_number(number);
		if ((winner = has_winner()) != -1) {
			return get_board_sum(&bingos[winner]) * number;
		}
	}

	return -1;
}

int part2() {
	int board;
	int size = numbers.size();
	size_t i;

	for (i = 0; i < size; i++) {
		check_number(numbers[i]);
		if (get_number_winners() == bingos.size() - 1) {
			board = loser();
			break;
		}
	}

	while (i < size && !is_winner(&bingos[board])) {
		check_number(numbers[i]);
		if (is_winner(&bingos[board])) {
			return get_board_sum(&bingos[board]) * numbers[i];
		}
		i++;
	}

	return -1;
}

int main() {
	parse();

	cout << "The solution to part 1 is " << part1() << "." << endl;
	cout << "The solution to part 2 is " << part2() << "." << endl;

	return 0;
}
