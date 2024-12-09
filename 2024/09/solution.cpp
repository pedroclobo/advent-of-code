#include<iostream>
#include<fstream>
#include<vector>
#include<optional>

struct Input {
  std::vector<std::optional<unsigned>> fs;

  static Input parse() {
    std::ifstream file("input.txt");
    std::vector<std::optional<unsigned>> fs;
    std::string line;
    while (std::getline(file, line))
      if (!line.empty())
        break;

    file.close();

    Input input;

    unsigned id = '0';
    for (unsigned i = 0; i < line.size(); ++i) {
      std::optional<unsigned> push = i % 2 == 0
        ? std::optional<unsigned>(id)
        : std::nullopt;
      for (unsigned j = 0; j < line[i] - '0'; ++j)
        input.fs.push_back(push);

      if (i % 2 == 0) ++id;
    }

    return input;
  }

  std::optional<unsigned> consecutive(unsigned free, unsigned max_i) const {
    unsigned count = 0;
    for (unsigned i = 0; i < max_i; ++i) {
      if (fs[i].has_value()) {
        if (count >= free) return i - count;
        count = 0;
      } else {
        ++count;
      }
    }
    return std::nullopt;
  }

  unsigned long long checksum() const {
    unsigned long long sum = 0;
    for (unsigned long i = 0; i < fs.size(); ++i)
      if (fs[i].has_value())
        sum += i * (fs[i].value() - '0');
    return sum;
  }
};

unsigned long long part1(Input &input) {
  unsigned i = 0, j = input.fs.size() - 1;

  while (i < j) {
    while (input.fs[i].has_value())  ++i;
    while (!input.fs[j].has_value()) --j;
    if (i < j) std::swap(input.fs[i], input.fs[j]);
  }

  return input.checksum();
}

unsigned long long part2(Input &input) {
  unsigned j = input.fs.size() - 1;

  while (j > 0) {
    while (j > 0 && !input.fs[j].has_value()) --j;

    // lookahead
    unsigned lhj = 0;
    for (unsigned _j = j; _j > 0 && input.fs[_j].has_value() &&
                          input.fs[_j].value() == input.fs[j].value(); --_j)
      ++lhj;

    std::optional<unsigned> _i = input.consecutive(lhj, j);
    if (!_i.has_value()) { j -= lhj; continue; }

    unsigned value = input.fs[j].value();
    unsigned i = _i.value();
    for (unsigned steps = lhj; steps > 0; steps--) {
      std::swap(input.fs[i], input.fs[j]);
      ++i; --j;
    }
  }

  return input.checksum();
}

int main() {
  auto input = Input::parse();
  std::cout << "The solution to part 1 is " << part1(input) << "." << std::endl;
  input = Input::parse();
  std::cout << "The solution to part 2 is " << part2(input) << "." << std::endl;

  return 0;
}
