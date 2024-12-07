#include<cmath>
#include<iostream>
#include<vector>
#include<fstream>
#include<regex>

struct Equation {
  long res;
  std::vector<int> ops;
};

class Input {
public:
  std::vector<Equation> eqs;

  Input() {
    eqs = std::vector<Equation>();
  }

  static Input parse() {
    std::regex digit(R"(\d+)");
    std::ifstream file("input.txt");
    std::string line;

    Input input;

    while (std::getline(file, line)) {
      auto words_begin = std::sregex_iterator(line.begin(), line.end(), digit);
      auto words_end = std::sregex_iterator();
      std::regex_iterator i = words_begin;

      Equation eq;
      eq.res = std::stol((i++)->str());
      for (; i != words_end; ++i)
        eq.ops.push_back(std::stoi(i->str()));
      input.eqs.push_back(eq);
    }

    return input;
  }
};

int num_digits(long n) {
  int count = 0;
  for (; n; n /= 10)
    ++count;
  return count;
}

bool dfs(const Equation &eq, unsigned depth, long res, bool p2 = false) {
  if (depth == eq.ops.size())
    return res == eq.res;

  if (p2)
    return dfs(eq, depth + 1, res + eq.ops[depth], true) ||
           dfs(eq, depth + 1, res * eq.ops[depth], true) ||
           dfs(eq, depth + 1, res * pow(10, num_digits(eq.ops[depth])) + eq.ops[depth], true);

  return dfs(eq, depth + 1, res + eq.ops[depth]) ||
         dfs(eq, depth + 1, res * eq.ops[depth]);
}

long part1(const Input &input) {
  long sum = 0;

  for (const auto &eq : input.eqs)
    if (dfs(eq, 1, eq.ops[0]))
      sum += eq.res;

  return sum;
}

long part2(const Input &input) {
  long sum = 0;

  for (const auto &eq : input.eqs)
    if (dfs(eq, 1, eq.ops[0], true))
      sum += eq.res;

  return sum;
}

int main() {
  auto input = Input::parse();

  std::cout << "The solution to part 1 is " << part1(input) << "." << std::endl;
  std::cout << "The solution to part 2 is " << part2(input) << "." << std::endl;

  return 0;
}
