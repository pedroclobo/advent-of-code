#include<iostream>
#include<fstream>
#include<vector>
#include<map>
#include<stdint.h>

uint64_t num_digits(uint64_t n) {
  uint64_t digits = 0;
  while (n) {
    n /= 10;
    ++digits;
  }
  return digits;
}

uint64_t power(uint64_t base, uint64_t exp) {
  uint64_t result = 1;
  for (uint64_t i = 0; i < exp; ++i) result *= base;
  return result;
}

using dp = std::map<std::pair<uint64_t, unsigned>, uint64_t>;

struct Input {
  std::vector<uint64_t> stones;

  static Input parse() {
    std::ifstream file("input.txt");
    std::vector<uint64_t> stones;
    std::string line;

    while (std::getline(file, line)) {
      size_t start = 0, end = 0;
      while ((end = line.find(' ', start)) != std::string::npos) {
        stones.push_back(std::stoi(line.substr(start, end - start)));
        start = end + 1;
      }
      stones.push_back(std::stoi(line.substr(start)));
    }

    return {stones};
  }

  uint64_t rec(uint64_t stone, unsigned t, unsigned t_max, dp &dp) const {
    if (t == t_max)
      return 1;

    if (dp.find({stone, t}) != dp.end())
      return dp[{stone, t}];

    uint64_t n_digits = num_digits(stone);
    if (stone == 0) dp[{stone, t}] = rec(1, t + 1, t_max, dp);
    else if (n_digits % 2 == 0)
      dp[{stone, t}] = rec(stone / power(10, n_digits / 2), t + 1, t_max, dp) +
                       rec(stone % power(10, n_digits / 2), t + 1, t_max, dp);
    else dp[{stone, t}] = rec(stone * 2024, t + 1, t_max, dp);

    return dp[{stone, t}];
  }

};

uint64_t part1(const Input &input) {
  uint64_t sum = 0;
  dp dp;
  for (uint64_t stone : input.stones)
    sum += input.rec(stone, 0, 25, dp);
  return sum;
}

uint64_t part2(const Input &input) {
  uint64_t sum = 0;
  dp dp;
  for (uint64_t stone : input.stones)
    sum += input.rec(stone, 0, 75, dp);
  return sum;
}

int main() {
  auto input = Input::parse();
  std::cout << "The solution to part 1 is " << part1(input) << "." << std::endl;
  std::cout << "The solution to part 2 is " << part2(input) << "." << std::endl;

  return 0;
}
