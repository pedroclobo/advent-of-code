#include<iostream>
#include<fstream>
#include<set>
#include<vector>

struct Input {
  std::vector<std::string> grid;

  static Input parse() {
    std::ifstream file("input.txt");
    std::vector<std::string> grid;
    std::string line;

    while (std::getline(file, line))
      grid.push_back(line);

    return {grid};
  }

  bool inBounds(unsigned y, unsigned x) const {
    return y >= 0 && y < grid.size() && x >= 0 && x < grid[0].size();
  }
};

std::set<std::pair<unsigned, unsigned>>
score(const Input &input, unsigned y, unsigned x) {
  char c = input.grid[y][x];
  if (c == '9') return {{y, x}};

  std::set<std::pair<unsigned, unsigned>> rec;
  for (int dy = -1; dy <= 1; ++dy)
    for (int dx = -1; dx <= 1; ++dx) {
      if (dy == 0 && dx == 0) continue;
      if (dy != 0 && dx != 0) continue;
      if (!input.inBounds(y + dy, x + dx)) continue;

      if (input.grid[y + dy][x + dx] == c + 1)
        for (const auto &p : score(input, y + dy, x + dx))
          rec.insert(p);
    }

  return rec;
}

unsigned rating(const Input &input, unsigned y, unsigned x) {
  char c = input.grid[y][x];
  if (c == '9') return 1;

  unsigned rec = 0;
  for (int dy = -1; dy <= 1; ++dy)
    for (int dx = -1; dx <= 1; ++dx) {
      if (dy == 0 && dx == 0) continue;
      if (dy != 0 && dx != 0) continue;
      if (!input.inBounds(y + dy, x + dx)) continue;

      if (input.grid[y + dy][x + dx] == c + 1)
        rec += rating(input, y + dy, x + dx);
    }

  return rec;
}

unsigned part1(const Input &input) {
  unsigned scores = 0;
  for (unsigned y = 0; y < input.grid.size(); ++y)
    for (unsigned x = 0; x < input.grid[0].size(); ++x)
      if (input.grid[y][x] == '0')
        scores += score(input, y, x).size();

  return scores;
}

unsigned part2(const Input &input) {
  unsigned scores = 0;
  for (unsigned y = 0; y < input.grid.size(); ++y)
    for (unsigned x = 0; x < input.grid[0].size(); ++x)
      if (input.grid[y][x] == '0')
        scores += rating(input, y, x);

  return scores;
}

int main() {
  auto input = Input::parse();
  std::cout << "The solution to part 1 is " << part1(input) << "." << std::endl;
  std::cout << "The solution to part 2 is " << part2(input) << "." << std::endl;

  return 0;
}
