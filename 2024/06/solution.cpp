#include<cassert>
#include<iostream>
#include<vector>
#include<fstream>
#include<set>
#include<map>

enum class Direction {
  UP, RIGHT, DOWN, LEFT
};

Direction turnRight(Direction dir) {
  switch (dir) {
    case Direction::UP:    return Direction::RIGHT;
    case Direction::RIGHT: return Direction::DOWN;
    case Direction::DOWN:  return Direction::LEFT;
    case Direction::LEFT:  return Direction::UP;
  }
}

// implement << operator for Direction
std::ostream &operator<<(std::ostream &os, const Direction &dir) {
  switch (dir) {
    case Direction::UP:    os << "UP";    break;
    case Direction::RIGHT: os << "RIGHT"; break;
    case Direction::DOWN:  os << "DOWN";  break;
    case Direction::LEFT:  os << "LEFT";  break;
  }
  return os;
}

class Grid {
public:
  // true is there is no obstacle, false otherwise
  std::vector<std::vector<bool>> grid;

  bool outOfBounds(int y, int x) const {
    return y < 0 || y >= grid.size() || x < 0 || x >= grid[y].size();
  }

  bool free(int y, int x) const {
    assert(!outOfBounds(y, x));
    return grid[y][x];
  }

};

class Input {
public:
  Grid grid;
  std::pair<int, int> start;
  Direction dir;

  static Input parse() {
    std::ifstream file("input.txt");
    Input input;
    Grid grid;

    std::string line;
    while (std::getline(file, line)) {
      std::vector<bool> row;
      for (size_t x = 0; x < line.length(); ++x) {
        if (line[x] == '^')
          input.start = std::make_pair(grid.grid.size(), x);
        row.push_back(line[x] != '#');
      }
      grid.grid.push_back(row);
    }

    input.grid = grid;
    input.dir = Direction::UP;

    return input;
  }
};

int part1(const Input &input, std::set<std::pair<int, int>> &visited) {
  int y = input.start.first, x = input.start.second;
  Direction dir = Direction::UP;

  while (1) {
    visited.insert({y, x});
    int y_next = y, x_next = x;

    if (dir == Direction::UP)         y_next -= 1;
    else if (dir == Direction::RIGHT) x_next += 1;
    else if (dir == Direction::DOWN)  y_next += 1;
    else if (dir == Direction::LEFT)  x_next -= 1;

    if (input.grid.outOfBounds(y_next, x_next)) break;

    if (input.grid.free(y_next, x_next)) {
      y = y_next;
      x = x_next;
    } else {
      dir = turnRight(dir);
    }
  }

  return visited.size();
}

bool loops(Input &input, int yo, int xo) {
  int y = input.start.first, x = input.start.second;
  int y_next = y, x_next = x;
  Direction dir = Direction::UP;

  input.grid.grid[yo][xo] = false;

  std::set<std::tuple<int, int, Direction>> seen;

  while (1) {
    if (seen.find({y, x, dir}) != seen.end()) {
      input.grid.grid[yo][xo] = true;
      return true;
    }
    seen.insert({y, x, dir});

    if (dir == Direction::UP)         y_next = y - 1;
    else if (dir == Direction::RIGHT) x_next = x + 1;
    else if (dir == Direction::DOWN)  y_next = y + 1;
    else if (dir == Direction::LEFT)  x_next = x - 1;

    if (input.grid.outOfBounds(y_next, x_next)) {
      input.grid.grid[yo][xo] = true;
      return false;
    }

    if (input.grid.free(y_next, x_next)) {
      y = y_next; x = x_next;
    } else {
      if (dir == Direction::UP)         y_next++;
      else if (dir == Direction::RIGHT) x_next--;
      else if (dir == Direction::DOWN)  y_next--;
      else if (dir == Direction::LEFT)  x_next++;
      dir = turnRight(dir);
      y = y_next; x = x_next;
    }
  }
}

int part2(Input &input, const std::set<std::pair<int, int>> &visited) {
  int y = input.start.first, x = input.start.second;

  unsigned count = 0;

  for (const auto &v : visited)
    if (v.first != y || v.second != x)
      if (loops(input, v.first, v.second))
        ++count;

  return count;
}

int main() {
  auto input = Input::parse();
  std::set<std::pair<int, int>> visited;
  std::cout << "The solution to part 1 is " << part1(input, visited) << "." << std::endl;
  std::cout << "The solution to part 2 is " << part2(input, visited) << "." << std::endl;

  return 0;
}
