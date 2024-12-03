#include<cassert>
#include<iostream>
#include<vector>
#include<fstream>
#include<sstream>
#include<regex>

class Mul {
public:
  int x, y;

  Mul(int x, int y) : x(x), y(y) {}

  unsigned res() const {
  return x * y;
  }
};

class Input {
public:
  std::vector<Mul> muls;

  Input() {
    muls = std::vector<Mul>();
  }

  static Input parse(bool p2 = false) {
    std::regex pattern(R"(mul\(\d+,\d+\)|do\(\)|don\'t\(\))");

    std::ifstream file("input.txt");
    std::stringstream input;
    input << file.rdbuf();
    std::string fileContents = input.str();

    auto words_begin = std::sregex_iterator(fileContents.begin(), fileContents.end(), pattern);
    auto words_end = std::sregex_iterator();

    Input result;
    bool add = true;
    for (std::sregex_iterator i = words_begin; i != words_end; ++i) {
      std::smatch match = *i;
      std::string matchStr = match.str();

      size_t openParen = matchStr.find('(');
      size_t comma = matchStr.find(',');
      size_t closeParen = matchStr.find(')');

      size_t doPos = matchStr.find("do");
      size_t dontPos = matchStr.find("don't");

      if (openParen != std::string::npos &&
          comma != std::string::npos &&
          closeParen != std::string::npos) {
        int num1 = std::stoi(matchStr.substr(openParen + 1, comma - openParen - 1));
        int num2 = std::stoi(matchStr.substr(comma + 1, closeParen - comma - 1));
        if (add) result.muls.push_back(Mul(num1, num2));
      } else {
        assert(doPos != std::string::npos || dontPos != std::string::npos);
        if (!p2) continue;
        add = dontPos == std::string::npos;
      }
    }

    return result;
  }
};

unsigned part1(const Input &input) {
  unsigned res = 0;
  for (const auto &mul : input.muls)
    res += mul.res();
  return res;
}

unsigned part2(const Input &input) {
  unsigned res = 0;
  for (const auto &mul : input.muls)
    res += mul.res();
  return res;
}

int main() {
  std::cout << "The solution to part 1 is " << part1(Input::parse()) << "." << std::endl;
  std::cout << "The solution to part 2 is " << part2(Input::parse(true)) << "." << std::endl;

  return 0;
}
