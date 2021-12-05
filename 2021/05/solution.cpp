#include<iostream>
#include<fstream>
#include<vector>
#include<sstream>

#define B_SIZE 1000

using namespace std;

vector<int> numbers;
int board[B_SIZE][B_SIZE];

void parse() {
	ifstream file("input.txt");
	string line;

	int value;
	char c;

	while (getline(file, line)) {
		stringstream ss(line);

		// 1st number
		ss >> value;
		numbers.push_back(value);

		// comma
		ss.ignore();

		// 2nd number
		ss >> value;
		numbers.push_back(value);

		ss.ignore();
		ss.ignore();
		ss.ignore();
		ss.ignore();

		// 3rd number
		ss >> value;
		numbers.push_back(value);

		ss.ignore();

		// 4th number
		ss >> value;
		numbers.push_back(value);
	}
}

void init_board() {
	for (size_t i = 0; i < B_SIZE; i++) {
		for (size_t j = 0; j < B_SIZE; j++) {
			board[i][j] = 0;
		}
	}
}

void fill_board(const vector<int> *pairs) {
	size_t size = pairs->size();

	for (size_t i = 0; i < size; i += 4) {
		if (pairs->at(i) == pairs->at(i+2)) {
			if (pairs->at(i+1) < pairs->at(i+3)) {
				for (int j = pairs->at(i+1); j <= pairs->at(i+3); j++) {
					board[j][pairs->at(i)] += 1;
				}
			} else {
				for (int j = pairs->at(i+3); j <= pairs->at(i+1); j++) {
					board[j][pairs->at(i)] += 1;
				}
			}
		} else if (pairs->at(i+1) == pairs->at(i+3)) {
			if (pairs->at(i) < pairs->at(i+2)) {
				for (int j = pairs->at(i); j <= pairs->at(i+2); j++) {
					board[pairs->at(i+1)][j] += 1;
				}
			} else {
				for (int j = pairs->at(i+2); j <= pairs->at(i); j++) {
					board[pairs->at(i+1)][j] += 1;
				}
			}
		} else {
			if (pairs->at(i+2) > pairs->at(i) && pairs->at(i+3) > pairs->at(i+1)) {
				for (int j = 0; j <= pairs->at(i+2) - pairs->at(i); j++) {
					board[pairs->at(i+1)+j][pairs->at(i)+j] += 1;
				}
			} else if (pairs->at(i+2) > pairs->at(i) && pairs->at(i+3) < pairs->at(i+1)) {
				for (int j = 0; j <= pairs->at(i+2) - pairs->at(i); j++) {
					board[pairs->at(i+1)-j][pairs->at(i)+j] += 1;
				}
			} else if (pairs->at(i+2) < pairs->at(i) && pairs->at(i+3) > pairs->at(i+1)) {
				for (int j = 0; j <= pairs->at(i) - pairs->at(i+2); j++) {
					board[pairs->at(i+1)+j][pairs->at(i)-j] += 1;
				}
			} else if (pairs->at(i+2) < pairs->at(i) && pairs->at(i+3) < pairs->at(i+1)) {
				for (int j = 0; j <= pairs->at(i) - pairs->at(i+2); j++) {
					board[pairs->at(i+1)-j][pairs->at(i)-j] += 1;
				}
			}
		}
	}
}

int search_board(int number) {
	int count = 0;

	for (size_t i = 0; i < B_SIZE; i++) {
		for (size_t j = 0; j < B_SIZE; j++) {
			if (board[i][j] >= number) {
				count++;
			}
		}
	}

	return count;
}

int part1() {
	init_board();

	vector<int> hv_pairs;
	size_t size = numbers.size();

	for (size_t i = 0; i < size; i += 4) {
		if (numbers[i] == numbers[i+2] || numbers[i+1] == numbers[i+3]) {
			for (int j = 0; j < 4; j++) {
				hv_pairs.push_back(numbers[i+j]);
			}
		}
	}

	fill_board(&hv_pairs);

	return search_board(2);
}

int part2() {
	init_board();

	vector<int> hvd_pairs;
	size_t size = numbers.size();

	for (size_t i = 0; i < size; i += 4) {
		for (int j = 0; j < 4; j++) {
			hvd_pairs.push_back(numbers[i+j]);
		}
	}

	fill_board(&hvd_pairs);

	return search_board(2);
}

int main() {
	parse();

	cout << "The solution to part 1 is " << part1() << "." << endl;
	cout << "The solution to part 2 is " << part2() << "." << endl;

	return 0;
}
