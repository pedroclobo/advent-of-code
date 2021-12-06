#include<iostream>
#include<fstream>
#include<vector>
#include<sstream>
#include<map>

using namespace std;

vector<int> numbers;

void parse() {
	ifstream file("input.txt");
	string line;
	file >> line;
	stringstream input(line);

	for (int number; input >> number;) {
		numbers.push_back(number);
		if (input.peek() == ',') {
			input.ignore();
		}
	}
}

int part1() {
	for (int k = 0; k < 80; k++) {
		for (size_t i = 0; i < numbers.size(); i++) {
			if (numbers[i] == 0) {
				numbers[i] = 6;
				// account for current iteration decrease
				numbers.push_back(9);
			} else {
				numbers[i]--;
			}
		}
	}

	return numbers.size();
}

long part2() {
	numbers.clear();
	parse();

	long quantities[9] = { 0 };

	for (int number: numbers) {
		quantities[number]++;
	}

	for (int k = 0; k < 256; k++) {
		long quant_prev = quantities[8];
		long quant_curr;

		for (int i = 7; i >= 0; i--) {
			quant_curr = quantities[i];
			quantities[i] = quant_prev;
			quant_prev = quant_curr;
		}

		quantities[6] += quant_prev;
		quantities[8] = quant_prev;
	}

	long sum = 0;
	for (size_t i = 0; i < 9; i++) {
		sum += quantities[i];
	}

	return sum;
}

int main() {
	parse();

	cout << "The solution to part 1 is " << part1() << "." << endl;
	cout << "The solution to part 2 is " << part2() << "." << endl;

	return 0;
}
