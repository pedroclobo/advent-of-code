#include<algorithm>
#include<cassert>
#include<cstdint>
#include<deque>
#include<fstream>
#include<iostream>
#include<map>
#include<memory>
#include<set>
#include<stdint.h>
#include<sys/types.h>
#include<vector>

std::map<char, std::pair<int, int>> deltas = {
  {'^', {-1, 0}},
  {'v', {1, 0}},
  {'>', {0, 1}},
  {'<', {0, -1}}
};

class Grid {
protected:
  std::vector<std::string> grid;

public:
  Grid(const std::vector<std::string> &grid) : grid(grid) {}
  virtual ~Grid() = default;

  bool inBounds(int y, int x) const {
    return x >= 0 && x < grid[0].size() && y >= 0 && y < grid.size();
  }

  unsigned size() const {
    return grid.size();
  }

  virtual void move(char dir, std::pair<int, int> &robot) = 0;

  const std::string &operator[](int i) const {
    return grid[i];
  }

  virtual uint64_t gps() const = 0;

  static std::unique_ptr<Grid> parse(bool extended=false);
};

class SimpleGrid : public Grid {
public:
  SimpleGrid(const std::vector<std::string> &grid) : Grid(grid) {}

  static SimpleGrid parse() {
    std::ifstream file("input.txt");
    std::vector<std::string> grid;

    std::string line;
    while (std::getline(file, line)) {
      if (line.empty()) break;
      grid.push_back(line);
    }

    return {grid};
  }

  void move(char dir, std::pair<int, int> &robot) override {
    auto [y, x] = robot;
    auto [dy, dx] = deltas[dir];
    auto [ny, nx] = std::pair(y + dy, x + dx);

    if (grid[ny][nx] == '.') std::swap(grid[y][x], grid[ny][nx]);
    else if (grid[ny][nx] == '#') return;
    else if (grid[ny][nx] == 'O') {
      int fx = nx, fy = ny;
      while (inBounds(fy, fx) && grid[fy][fx] != '.') {
        if (grid[fy][fx] == '#') return;
        fx += dx, fy += dy;
      }
      std::swap(grid[ny][nx], grid[fy][fx]); // swap 'O' with '.'
      std::swap(grid[y][x], grid[ny][nx]);   // swap robot with nearest 'O'
    }

    robot = {ny, nx};
  }

  uint64_t gps() const override {
    uint64_t sum = 0;
    for (int y = 0; y < grid.size(); ++y)
      for (int x = 0; x < grid[0].size(); ++x)
        if (grid[y][x] == 'O')
          sum += y * 100 + x;
    return sum;
  }

};

class ExtendedGrid : public Grid {
public:
  ExtendedGrid(const std::vector<std::string> &grid) : Grid(grid) {}

  static ExtendedGrid parse() {
    std::ifstream file("input.txt");
    std::vector<std::string> grid;

    std::string line;
    while (std::getline(file, line)) {
      if (line.empty()) break;
      std::string newLine;
      for (char &c : line) {
        if (c == '#') newLine += "##";
        else if (c == 'O') newLine += "[]";
        else if (c == '.') newLine += "..";
        else if (c == '@') newLine += "@.";
      }
      grid.push_back(newLine);
    }

    return {grid};
  }

  void move(char dir, std::pair<int, int> &robot) override {
    auto [y, x] = robot;
    auto [dy, dx] = deltas[dir];
    auto [ny, nx] = std::pair(y + dy, x + dx);

    if (grid[ny][nx] == '#') return;
    else if (grid[ny][nx] == '[' || grid[ny][nx] == ']') {
      std::deque<std::pair<int, int>> queue = {robot};
      std::set<std::pair<int, int>> visited;
      bool wall = false;

      while (!queue.empty()) {
        auto [cy, cx] = queue.front();
        queue.pop_front();
        if (visited.contains({cy, cx})) continue;
        visited.insert({cy, cx});

        int ncy = cy + dy, ncx = cx + dx;

        if (grid[ncy][ncx] == '#') {
          wall = true; break;
        } else if (grid[ncy][ncx] == '[') {
          assert(grid[ncy][ncx + 1] == ']');
          queue.push_back({ncy, ncx});
          queue.push_back({ncy, ncx + 1});
        } else if (grid[ncy][ncx] == ']') {
          assert(grid[ncy][ncx - 1] == '[');
          queue.push_back({ncy, ncx});
          queue.push_back({ncy, ncx - 1});
        }
      }

      // can't move as we've indirectly hit a wall
      if (wall) return;

      // copy from bot to top
      while (!visited.empty()) {
        std::vector<std::pair<int, int>> sorted(visited.begin(), visited.end());
        std::sort(sorted.begin(), sorted.end(), [](const auto &a, const auto &b) {
          return a.first < b.first || (a.first == b.first && a.second < b.second);
        });
        for (const auto [cy, cx] : sorted) {
          int ncy = cy + dy, ncx = cx + dx;
          if (!visited.contains({ncy, ncx})) {
            assert(grid[ncy][ncx] == '.');
            grid[ncy][ncx] = grid[cy][cx];
            grid[cy][cx] = '.';
            visited.erase({cy, cx});
          }
        }
      }

      std::swap(grid[y][x], grid[ny][nx]);
      robot = {ny, nx};
    }

    std::swap(grid[y][x], grid[ny][nx]);
    robot = {ny, nx};
  }

  uint64_t gps() const override {
    uint64_t sum = 0;
    for (int y = 0; y < grid.size(); ++y)
      for (int x = 0; x < grid[0].size(); ++x)
        if (grid[y][x] == '[')
          sum += y * 100 + x;
    return sum;
  }

};

std::unique_ptr<Grid> Grid::parse(bool extended) {
  if (extended)
    return std::make_unique<ExtendedGrid>(ExtendedGrid::parse());
  return std::make_unique<SimpleGrid>(SimpleGrid::parse());
}

struct Input {
  std::unique_ptr<Grid> grid;
  std::string directions;
  unsigned idx;
  std::pair<int, int> robot;

  static Input parse(bool extended = false) {
    std::ifstream file("input.txt");
    auto grid = Grid::parse(extended);
    std::string directions;
    std::string line;

    while (std::getline(file, line))
      if (line.empty()) break;

    while (std::getline(file, line))
      directions += line;

    for (int y = 0; y < grid->size(); ++y)
      for (int x = 0; x < (*grid)[0].size(); ++x)
        if ((*grid)[y][x] == '@')
          return {std::move(grid), directions, 0, {y, x}};

    assert(false);
  }

  void move() {
    char dir = directions[idx++];
    grid->move(dir, robot);
  }
};

uint64_t part1(Input &input) {
  for (unsigned i = 0; i < input.directions.size(); ++i)
    input.move();
  return input.grid->gps();
}

uint64_t part2(Input &input) {
  for (unsigned i = 0; i < input.directions.size(); ++i)
    input.move();
  return input.grid->gps();
}

int main() {
  auto input = Input::parse();
  std::cout << "The solution to part 1 is " << part1(input) << "." << std::endl;

  input = Input::parse(true);
  std::cout << "The solution to part 2 is " << part2(input) << "." << std::endl;

  return 0;
}
