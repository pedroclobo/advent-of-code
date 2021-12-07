#include<bits/stdc++.h>

using namespace std;

vector<int> positions;

void parse() {
	ifstream file("input.txt");
	string line;
	file >> line;
	stringstream input(line);

	for (int number; input >> number;) {
		positions.push_back(number);
		if (input.peek() == ',') {
			input.ignore();
		}
	}
}

int find_max() {
	int max = 0;
	for (int number: positions) {
		if (number > max) {
			max = number;
		}
	}

	return max;
}

int compute_cost(int n) {
	int cost = 0;

	for (int number: positions) {
		cost += abs(number - n);
	}

	return cost;
}

int compute_cost_with_increase(int n) {
	int cost = 0;

	for (int number: positions) {
		int dist = abs(number - n);
		cost += (dist * (dist + 1)) / 2;
	}

	return cost;
}

int part1() {
	int min = INT_MAX;
	int max = find_max();
	int cost;

	for (int i = 0; i <= max; i++) {
		if ((cost = compute_cost(i)) < min) {
			min = cost;
		}
	}

	return min;
}

int part2() {
	int min = INT_MAX;
	int max = find_max();
	int cost;

	for (int i = 0; i <= max; i++) {
		if ((cost = compute_cost_with_increase(i)) < min) {
			min = cost;
		}
	}

	return min;
}

int main() {
	parse();

	cout << "The solution to part 1 is " << part1() << "." << endl;
	cout << "The solution to part 2 is " << part2() << "." << endl;

	return 0;
}
