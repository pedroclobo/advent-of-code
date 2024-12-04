#include<cassert>
#include<iostream>
#include<vector>
#include<fstream>

class Input {
public:
  std::vector<std::string> grid;

  Input() {
    grid = std::vector<std::string>();
  }

  char at(unsigned y, unsigned x) const {
    if (y >= grid.size() || x >= grid[0].length())
      return '-';
    return grid[y][x];
  }

  static Input parse() {
    std::ifstream file("input.txt");
    std::string line;

    Input input;

    while (std::getline(file, line))
      if (!line.empty())
        input.grid.push_back(line);

    file.close();

    return input;
  }
};

int part1(const Input &input) {
  unsigned n = input.grid.size();
  unsigned d = input.grid[0].length();
  assert(n == d);

  unsigned count = 0;

  for (unsigned y = 0; y < n; ++y)
    for (unsigned x = 0; x < n; ++x)
      if (input.at(y, x) == 'X') {
        // vertical
        if (input.at(y, x + 1) == 'M' && input.at(y, x + 2) == 'A' && input.at(y, x + 3) == 'S')
          ++count;
        if (input.at(y, x - 1) == 'M' && input.at(y, x - 2) == 'A' && input.at(y, x - 3) == 'S')
          ++count;

        // horizontal
        if (input.at(y + 1, x) == 'M' && input.at(y + 2, x) == 'A' && input.at(y + 3, x) == 'S')
          ++count;
        if (input.at(y - 1, x) == 'M' && input.at(y - 2, x) == 'A' && input.at(y - 3, x) == 'S')
          ++count;

        // diagonal
        if (input.at(y + 1, x + 1) == 'M' && input.at(y + 2, x + 2) == 'A' && input.at(y + 3, x + 3) == 'S')
          ++count;
        if (input.at(y - 1, x - 1) == 'M' && input.at(y - 2, x - 2) == 'A' && input.at(y - 3, x - 3) == 'S')
          ++count;
        if (input.at(y + 1, x - 1) == 'M' && input.at(y + 2, x - 2) == 'A' && input.at(y + 3, x - 3) == 'S')
          ++count;
        if (input.at(y - 1, x + 1) == 'M' && input.at(y - 2, x + 2) == 'A' && input.at(y - 3, x + 3) == 'S')
          ++count;
      }

  return count;
}

int part2(const Input &input) {
  unsigned n = input.grid.size();
  unsigned d = input.grid[0].length();
  assert(n == d);

  unsigned count = 0;

  // vertical
  for (unsigned y = 0; y < n; ++y)
    for (unsigned x = 0; x < n; ++x)
      if (input.at(y, x) == 'A') {
        /* M.S
           .A.
           M.S */
        if (input.at(y - 1, x - 1) == 'M' && input.at(y - 1, x + 1) == 'S' &&
            input.at(y + 1, x - 1) == 'M' && input.at(y + 1, x + 1) == 'S')
          ++count;

        /* S.S
           .A.
           M.M */
        if (input.at(y - 1, x - 1) == 'S' && input.at(y - 1, x + 1) == 'S' &&
            input.at(y + 1, x - 1) == 'M' && input.at(y + 1, x + 1) == 'M')
          ++count;

        /* S.M
           .A.
           S.M */
        if (input.at(y - 1, x - 1) == 'S' && input.at(y - 1, x + 1) == 'M' &&
            input.at(y + 1, x - 1) == 'S' && input.at(y + 1, x + 1) == 'M')
          ++count;

        /* M.M
           .A.
           S.S */
        if (input.at(y - 1, x - 1) == 'M' && input.at(y - 1, x + 1) == 'M' &&
            input.at(y + 1, x - 1) == 'S' && input.at(y + 1, x + 1) == 'S')
          ++count;
      }

  return count;
}

int main() {
  auto input = Input::parse();

  std::cout << "The solution to part 1 is " << part1(input) << "." << std::endl;
  std::cout << "The solution to part 2 is " << part2(input) << "." << std::endl;

  return 0;
}
