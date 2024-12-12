#include<iostream>
#include<fstream>
#include<vector>
#include<stdint.h>
#include<set>
#include<optional>

static const std::vector<std::pair<int, int>> dirs = {{
    {-1, 0}, {1, 0}, {0, -1}, {0, 1}
}};

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

  bool inBounds(int y, int x) const {
    return y >= 0 && y < grid.size() && x >= 0 && x < grid[0].size();
  }

  std::optional<char> at(int y, int x) const {
    return inBounds(y, x) ? std::optional<char>(grid[y][x]) : std::nullopt;
  }
};

struct Region {
  std::set<std::pair<int, int>> plants;

  inline bool operator<(const Region &other) const {
    return plants < other.plants;
  }

  uint64_t area() const {
    return plants.size();
  }

  uint64_t perimeter() const {
    uint64_t perimeter = 0;
    for (auto [y, x] : plants)
      for (auto [dy, dx] : dirs)
        if (plants.find({y + dy, x + dx}) == plants.end())
          ++perimeter;
    return perimeter;
  }

  uint64_t sides() const {
    using Plant = std::pair<int, int>;
    std::set<std::pair<Plant, Plant>> perimeters;

    for (const auto &p : plants) {
      int y = p.first, x = p.second;
      for (auto [dy, dx] : dirs)
        if (plants.find({y + dy, x + dx}) == plants.end())
          perimeters.insert({{y, x}, {y + dy, x + dx}});
    }

    for (auto it = perimeters.begin(); it != perimeters.end();) {
      auto [a, b] = *it;
      auto [y1, x1] = a;
      auto [y2, x2] = b;

      bool erase = false;
      for (auto [dy, dx] : dirs)
        if (perimeters.find({{y1 + dy, x1 + dx}, {y2 + dy, x2 + dx}}) != perimeters.end()) {
          it = perimeters.erase(it);
          erase = true;
          break;
        }
      if (!erase) ++it;
    }

    return perimeters.size();
  }
};

Region dfs(const Input &input, int y, int x, std::set<std::pair<int, int>> &visited) {
  if (visited.find({y, x}) != visited.end() || !input.inBounds(y, x))
    return {};

  Region region;
  region.plants.insert({y, x});
  visited.insert({y, x});

  for (const auto& [dy, dx] : dirs) {
    int _y = y + dy, _x = x + dx;
    if (input.at(_y, _x) == input.grid[y][x]) {
      Region adjacentRegion = dfs(input, _y, _x, visited);
      for (const auto &p : adjacentRegion.plants)
        region.plants.insert(p);
    }
  }

  return region;
}

uint64_t part1(const std::set<Region> &regions) {
  uint64_t total = 0;
  for (const auto &region : regions)
    total += region.area() * region.perimeter();
  return total;
}

uint64_t part2(const std::set<Region> &regions) {
  uint64_t total = 0;
  for (const auto &region : regions)
    total += region.area() * region.sides();
  return total;
}

int main() {
  auto input = Input::parse();

  std::set<Region> regions;
  std::set<std::pair<int, int>> visited;
  for (int y = 0; y < input.grid.size(); ++y)
    for (int x = 0; x < input.grid[0].size(); ++x) {
      auto region = dfs(input, y, x, visited);
      if (!region.plants.empty())
        regions.insert(region);
    }

  std::cout << "The solution to part 1 is " << part1(regions) << "." << std::endl;
  std::cout << "The solution to part 2 is " << part2(regions) << "." << std::endl;

  return 0;
}
