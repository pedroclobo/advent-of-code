#include<algorithm>
#include<fstream>
#include<iostream>
#include<map>
#include<queue>
#include<set>
#include<sys/types.h>
#include<vector>

std::map<char, std::pair<int, int>> deltas = {
  {0, {-1, 0}},  // up
  {1, {0, 1}},   // right
  {2, {1, 0}},   // down
  {3, {0, -1}}   // left
};

struct Input {
  std::vector<std::string> grid;
  std::pair<int, int> start;
  std::pair<int, int> end;

  static Input parse() {
    std::ifstream file("input.txt");
    std::string line;

    std::vector<std::string> grid;
    std::pair<int, int> start;
    std::pair<int, int> end;

    while (std::getline(file, line))
      grid.push_back(line);

    for (int y = 0; y < grid.size(); ++y)
      for (int x = 0; x < grid[0].size(); ++x) {
        if (grid[y][x] == 'S')
          start = {y, x};
        else if (grid[y][x] == 'E')
          end = {y, x};
      }

    return {grid, start, end};
  }
};

enum class Direction {
  Up, Down, Left, Right
};

#define INF 1000000

unsigned dijkstra(const Input &input, std::pair<int, int> loc, char dir, std::set<std::tuple<int, int, char>> &visited) {
  auto [y, x] = loc;

  // min priority queue
  std::priority_queue<
    std::tuple<unsigned, int, int, char>,
    std::vector<std::tuple<unsigned, int, int, char>>,
  std::greater<>> queue;
  queue.push({0, y, x, dir});

  std::map<std::tuple<int, int, char>, unsigned> dists = {{{y, x, dir}, 0}};

  unsigned res = INF;

  while (!queue.empty()) {
    auto [cdist, cy, cx, cdir] = queue.top();
    queue.pop();

    if (visited.count({cy, cx, cdir})) continue;
    visited.insert({cy, cx, cdir});

    if (std::make_pair(cy, cx) == input.end)
      res = std::min(res, static_cast<unsigned>(cdist));

    auto [dy, dx] = deltas[cdir];
    int ny = cy + dy, nx = cx + dx;

    for (const int dir : {(cdir + 1) % 4, (cdir + 3) % 4})
      if (!dists.count({cy, cx, dir}) || cdist + 1000 < dists[{cy, cx, dir}]) {
        dists[{cy, cx, dir}] = cdist + 1000;
        queue.push({cdist + 1000, cy, cx, dir});
      }

    if (input.grid[ny][nx] == '#') continue;

    if (!dists.count({ny, nx, cdir}) || cdist + 1 < dists[{ny, nx, cdir}]) {
      dists[{ny, nx, cdir}] = cdist + 1;
      queue.push({cdist + 1, ny, nx, cdir});
    }
  }

  return res;
}

unsigned part1(const Input &input) {
  auto visited = std::set<std::tuple<int, int, char>>();
  return dijkstra(input, input.start, 1, visited);
}

unsigned part2(const Input &input) {
  return 0;
}

int main() {
  auto input = Input::parse();
  std::cout << "The solution to part 1 is " << part1(input) << "." << std::endl;
  std::cout << "The solution to part 2 is " << part2(input) << "." << std::endl;

  return 0;
}
