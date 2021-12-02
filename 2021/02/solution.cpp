#include<iostream>
#include<fstream>
#include<vector>

using namespace std;

vector<string> directions;
vector<int> steps;

void parse() {
	ifstream file;
	file.open("input.txt");
	string direction;
	int step;

	while (file >> direction >> step) {
		directions.push_back(direction);
		steps.push_back(step);
	}
}

int part1() {
	string direction;
	int step;
	int d = 0;
	int h = 0;

	for (size_t i = 0; i < directions.size(); i++) {
		direction = directions[i];
		step = steps[i];
		if (direction == "forward") {
			h += step;
		} else if (direction == "down") {
			d += step;
		} else if (direction == "up") {
			d -= step;
		}
	}

	return d * h;
}

int part2() {
	string direction;
	int step;
	int d = 0;
	int h = 0;
	int a = 0;

	for (size_t i = 0; i < directions.size(); i++) {
		direction = directions[i];
		step = steps[i];
		if (direction == "forward") {
			h += step;
			d += a * step;
		} else if (direction == "down") {
			a += step;
		} else if (direction == "up") {
			a -= step;
		}
	}

	return d * h;
}

int main() {
	parse();

	cout << "The solution to part 1 is " << part1() << "." << endl;
	cout << "The solution to part 2 is " << part2() << "." << endl;

	return 0;
}
