#include<iostream>
#include<cstdint>
#include<fstream>
#include<sys/types.h>
#include<vector>
#include<stdint.h>
#include<regex>

#define STB_IMAGE_WRITE_IMPLEMENTATION
#include "stb_image_write.h"

#define N 103
#define M 101

int rem(int dividend, int divisor) {
  int rem = dividend % divisor;
  if (rem < 0) rem += divisor;
  return rem;
}

struct Robot {
  int px, py;
  int vx, vy;

  void move() {
    py = rem(py + vy, N);
    px = rem(px + vx, M);
  }
};

struct Input {
  std::vector<Robot> robots;

  static Input parse() {
    std::regex digit(R"(-?\d+)");
    std::ifstream file("input.txt");
    std::vector<Robot> robots;

    std::string line;
    while (std::getline(file, line)) {
      std::sregex_iterator it(line.begin(), line.end(), digit);
      int px = std::stoi(it++->str());
      int py = std::stoi(it++->str());
      int vx = std::stoi(it++->str());
      int vy = std::stoi(it++->str());
      robots.push_back({px, py, vx, vy});
    }

    return {robots};
  }

  uint64_t safety_score() {
    uint64_t q1 = 0, q2 = 0, q3 = 0, q4 = 0;
    for (auto [px, py, vx, vy] : robots) {
      if (px < M / 2 && py < N / 2) ++q1;
      else if (px > M / 2 && py < N / 2) ++q2;
      else if (px < M / 2 && py > N / 2) ++q3;
      else if (px > M / 2 && py > N / 2) ++q4;
    }
    return q1 * q2 * q3 * q4;
  }
};

struct Grid {
  char grid[N][M];

  Grid() {
    for (int i = 0; i < N; ++i)
      for (int j = 0; j < M; ++j)
        grid[i][j] = '.';
  }

  void draw(const std::vector<Robot> &robots) {
    for (int i = 0; i < N; ++i)
      for (int j = 0; j < M; ++j)
        grid[i][j] = ' ';

    for (auto [px, py, vx, vy] : robots)
      grid[py][px] = 'X';
  }

  void print() {
    for (int i = 0; i < N; ++i) {
      for (int j = 0; j < M; ++j)
        std::cout << grid[i][j];
      std::cout << std::endl;
    }
  }

  void toBmp(const std::string& filename) {
    std::vector<uint8_t> pixels(N * M * 3);

    for (int y = 0; y < N; ++y) {
      for (int x = 0; x < M; ++x) {
        int idx = (y * M + x) * 3;
        if (grid[y][x] == 'X') {
          pixels[idx] = 0;
          pixels[idx + 1] = 0;
          pixels[idx + 2] = 0;
        } else {
          pixels[idx] = 255;
          pixels[idx + 1] = 255;
          pixels[idx + 2] = 255;
        }
      }
    }

    stbi_write_bmp(filename.c_str(), M, N, 3, pixels.data());
  }
};

uint64_t part1(Input &input) {
  for (unsigned i = 0; i < 100; ++i)
    for (auto &robot : input.robots)
      robot.move();

  uint64_t q1 = 0, q2 = 0, q3 = 0, q4 = 0;
  for (auto [px, py, vx, vy] : input.robots) {
    if (px < M / 2 && py < N / 2) ++q1;
    else if (px > M / 2 && py < N / 2) ++q2;
    else if (px < M / 2 && py > N / 2) ++q3;
    else if (px > M / 2 && py > N / 2) ++q4;
  }

  return q1 * q2 * q3 * q4;
}

unsigned part2(Input &input) {
  Grid grid;
  uint64_t min_score = input.safety_score();
  unsigned frame = 0;

  for (unsigned i = 1; i < 10000; ++i) {
    for (auto &robot : input.robots)
      robot.move();
    uint64_t score = input.safety_score();
    if (score < min_score) {
      min_score = score;
      frame = i;
    }
    grid.draw(input.robots);
    grid.toBmp("frame" + std::to_string(i) + ".bmp");
  }

  return frame;
}

int main() {
  auto input = Input::parse();
  std::cout << "The solution to part 1 is " << part1(input) << "." << std::endl;

  input = Input::parse();
  std::cout << "The solution to part 2 is " << part2(input) << "." << std::endl;
  std::cout << "The images for the first 10000 frames have been saved." << std::endl;

  return 0;
}
