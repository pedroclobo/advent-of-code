#include<map>
#include<cassert>
#include<iostream>
#include<vector>
#include<fstream>
#include<algorithm>

class Input {
public:
  std::vector<int> left;
  std::vector<int> right;
  std::map<int, int> right_count;

  Input() {
    left = std::vector<int>();
    right = std::vector<int>();
    right_count = std::map<int, int>();
  }

};

Input parse() {
  std::ifstream file;
  file.open("input.txt");

  Input input;

  int number;
  unsigned count = 0;

  while (file >> number)
    count++ % 2 == 0 ?
      input.left.push_back(number) :
      input.right.push_back(number);

  file.close();

  for (auto &number : input.right)
    input.right_count[number]++;

  return std::move(input);
}

unsigned part1(const Input &input) {
  assert(input.left.size() == input.right.size());

  unsigned total = 0;
  for (unsigned i = 0, e = input.left.size(); i < e; ++i)
    total += abs(input.left[i] - input.right[i]);

  return total;
}

int part2(const Input &input) {
  assert(input.left.size() == input.right.size());

  unsigned total = 0;
  for (unsigned i = 0, e = input.left.size(); i < e; ++i)
    if (input.right_count.count(input.left[i]))
      total += input.left[i] * input.right_count.at(input.left[i]);

  return total;
}

int main() {
  auto input = parse();
  std::sort(input.left.begin(), input.left.end());
  std::sort(input.right.begin(), input.right.end());

  std::cout << "The solution to part 1 is " << part1(input) << "." << std::endl;
  std::cout << "The solution to part 2 is " << part2(input) << "." << std::endl;

  return 0;
}
