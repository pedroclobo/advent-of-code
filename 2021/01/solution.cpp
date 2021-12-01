#include<iostream>
#include<vector>
#include<fstream>

using namespace std;

vector<int> input;

void parse() {
	ifstream file;
	file.open("input.txt");
	int depth;

	while (file >> depth) {
		input.push_back(depth);
	}
}

int part1() {
	int count = 0;
	size_t length = input.size();

	for (size_t i = 1; i < length; i++) {
		if (input[i] > input[i-1]) {
			count++;
		}
	}

	return count;
}

int part2() {
	int count = 0;
	size_t length = input.size();

	for (size_t i = 1; i + 3 <= length; i++) {
		int b_sum = input[i-1] + input[i] + input[i+1];
		int c_sum = input[i] + input[i+1] + input[i+2];
		if (c_sum > b_sum) {
			count++;
		}
	}

	return count;
}

int main() {
	parse();

	cout << "The solution to part 1 is " << part1() << "." << endl;
	cout << "The solution to part 2 is " << part2() << "." << endl;

	return 0;
}
