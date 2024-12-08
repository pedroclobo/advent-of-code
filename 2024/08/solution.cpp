#include<iostream>
#include<vector>
#include<fstream>

struct Input {
  std::vector<std::string> antenas;
  std::vector<std::vector<bool>> antinodes;

  Input() {
    antenas = std::vector<std::string>();
    antinodes = std::vector<std::vector<bool>>();
  }

  bool inBounds(int y, int x) const {
    return y >= 0 && y < antenas.size() && x >= 0 && x < antenas[y].size();
  }

  static Input parse() {
    std::ifstream file("input.txt");
    std::string line;

    Input input;

    while (std::getline(file, line))
      if (!line.empty())
        input.antenas.push_back(line);

    file.close();

    int n = input.antenas.size();
    int m = input.antenas[0].size();

    input.antinodes = std::vector<std::vector<bool>>(n, std::vector<bool>(m, false));

    return input;
  }
};

int part1(Input &input) {
  for (unsigned y = 0; y < input.antenas.size(); ++y) {
    for (unsigned x = 0; x < input.antenas[y].size(); ++x) {
      for (unsigned b = 0; b < input.antenas.size(); ++b) {
        for (unsigned a = 0; a < input.antenas[b].size(); ++a) {
          auto first_antena = input.antenas[y][x];
          auto second_antena = input.antenas[b][a];
          if (first_antena == '.' || second_antena == '.' ||
              first_antena != second_antena || (y == b && x == a))
            continue;
          int dx = x - a, dy = y - b;
          if (input.inBounds(y + dy, x + dx))
            input.antinodes[y + dy][x + dx] = true;
          if (input.inBounds(b - dy, a - dx))
            input.antinodes[b - dy][a - dx] = true;
        }
      }
    }
  }

  int antinodes = 0;
  for (unsigned y = 0; y < input.antenas.size(); ++y) {
    for (unsigned x = 0; x < input.antenas[y].size(); ++x) {
      if (input.antinodes[y][x])
        ++antinodes;
    }
  }

  return antinodes;
}

int part2(Input &input) {
  for (unsigned y = 0; y < input.antenas.size(); ++y) {
    for (unsigned x = 0; x < input.antenas[y].size(); ++x) {
      for (unsigned b = 0; b < input.antenas.size(); ++b) {
        for (unsigned a = 0; a < input.antenas[b].size(); ++a) {
          auto first_antena = input.antenas[y][x];
          auto second_antena = input.antenas[b][a];
          if (first_antena == '.' || second_antena == '.' ||
              first_antena != second_antena || (y == b && x == a))
            continue;

          input.antinodes[y][x] = true;
          input.antinodes[b][a] = true;

          int dx = x - a, dy = y - b;

          unsigned ny = y + dy, nx = x + dx;
          unsigned nb = b - dy, na = a - dx;

          while (input.inBounds(ny, nx)) {
            input.antinodes[ny][nx] = true;
            ny-= dy, nx -= dx;
          }
          while (input.inBounds(nb, na)) {
            input.antinodes[nb][na] = true;
            nb -= dy, na -= dx;
          }
        }
      }
    }
  }

  int antinodes = 0;
  for (unsigned y = 0; y < input.antenas.size(); ++y) {
    for (unsigned x = 0; x < input.antenas[y].size(); ++x) {
      if (input.antinodes[y][x])
        ++antinodes;
    }
  }

  return antinodes;
}

int main() {
  auto input = Input::parse();
  std::cout << "The solution to part 1 is " << part1(input) << "." << std::endl;
  std::cout << "The solution to part 2 is " << part2(input) << "." << std::endl;

  return 0;
}
