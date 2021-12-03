#include<iostream>
#include<fstream>
#include<vector>
#include<bitset>

#define LEN 12

using namespace std;

vector<bitset<LEN>> input;

void parse() {
	ifstream file;
	file.open("input.txt");
	bitset<LEN> number;

	while (file >> number) {
		input.push_back(number);
	}
}

int calculate_nth_bit(const vector<bitset<LEN>> vec, int nth, bool m) {
	nth = LEN - nth - 1;
	int n0 = 0, n1 = 0;

	for (bitset<LEN> number: vec) {
		int bit = number[nth];
		if (bit == 0) {
			n0++;
		} else if (bit == 1) {
			n1++;
		}
	}

	if (m == true) {
		return n1 >= n0 ? 1 : 0;
	} else {
		return n1 >= n0 ? 0 : 1;
	}
}

long long part1() {
	bitset<LEN> gamma = 0;
	bitset<LEN> eps = 0;

	for (int i = 0; i < LEN; i++) {
		gamma <<= 1;
		gamma.set(0, calculate_nth_bit(input, i, true));

		eps <<= 1;
		eps.set(0, calculate_nth_bit(input, i, false));
	}

	return gamma.to_ullong() * eps.to_ullong();
}

long long part2() {
	vector<bitset<LEN>> oxygen = input;
	vector<bitset<LEN>> co2 = input;
	vector<bitset<LEN>> copy;

	for (int i = 0; i < LEN; i++) {
		int bit = calculate_nth_bit(oxygen, i, true);
		for (bitset<LEN> number: oxygen) {
			if (number[LEN - i - 1] == bit) {
				copy.push_back(number);
			}
		}
		if (oxygen.size() == 1) {
			break;
		}
		oxygen = copy;
		copy.clear();
	}

	for (int i = 0; i < LEN; i++) {
		int bit = calculate_nth_bit(co2, i, false);
		for (bitset<LEN> number: co2) {
			if (number[LEN - i - 1] == bit) {
				copy.push_back(number);
			}
		}
		if (co2.size() == 1) {
			break;
		}
		co2 = copy;
		copy.clear();
	}

	return oxygen[0].to_ullong() * co2[0].to_ullong();
}

int main() {
	parse();

	cout << "The solution to part 1 is " << part1() << "." << endl;
	cout << "The solution to part 2 is " << part2() << "." << endl;

	return 0;
}
